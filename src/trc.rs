//! Module containing logic to format [`tracing`] events.
use ansi_term::{Color, Style};
use itertools::Itertools;
use leafbuild_core::prelude::{AndThenDo, TakeIfUnlessOwned};
use leafbuild_interpreter::LfModName;
use std::error::Error;
use std::fmt::Debug;
use std::io::{BufWriter, Write};
use std::{fmt, io};
use tracing::field::{Field, Visit};
use tracing::span::Attributes;
use tracing::{Event, Id, Level, Subscriber};
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::{LookupSpan, SpanRef};
use tracing_subscriber::Layer;

#[derive(Debug)]
pub(crate) struct Data {
    kvs: Vec<(&'static str, String)>,
}

impl Data {
    pub fn new(attrs: &tracing::span::Attributes<'_>) -> Self {
        let mut span_data = Self { kvs: Vec::new() };
        attrs.record(&mut span_data);
        span_data
    }
}

impl Visit for Data {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.kvs.push((field.name(), format!("{:?}", value)))
    }
}

struct LeafbuildSpanExtension {
    modname: LfModName,
}

struct LeafbuildFmtVisitor<'a, Sink>
where
    Sink: FnMut(String) -> Result<(), io::Error>,
{
    sink: &'a mut Sink,
    comma: bool,
    err: Result<(), io::Error>,
}

impl<'a, Collector> Visit for LeafbuildFmtVisitor<'a, Collector>
where
    Collector: FnMut(String) -> Result<(), io::Error>,
{
    fn record_i64(&mut self, field: &Field, value: i64) {
        self.err = (self.sink)(format!("{}={} ", field.name(), value));
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.err = (self.sink)(format!("{}={} ", field.name(), value));
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.err = (self.sink)(format!("{}={} ", field.name(), value));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.err = (self.sink)(format!("{}={} ", field.name(), value));
    }

    fn record_error(&mut self, field: &Field, value: &dyn Error) {
        self.err = (self.sink)(format!("{}={} ", field.name(), value));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        let name = field.name();
        if name == "message" {
            self.err = (self.sink)(format!("{}{:?} ", if self.comma { "," } else { "" }, value));
            self.comma = true;
        } else {
            self.err = (self.sink)(format!(
                "{}{}={:?} ",
                if self.comma { "," } else { "" },
                name,
                value
            ));
            self.comma = true;
        }
    }
}

/// The leafbuild tracing layer configuration.
#[derive(Debug, Copy, Clone)]
pub struct Config {
    ansi: bool,
    level: tracing::metadata::LevelFilter,
    debug_mode: bool,
}

impl Config {
    /// Sets the ansi field to the parameter and returns the config back
    #[must_use]
    pub const fn with_ansi(mut self, ansi: bool) -> Self {
        self.ansi = ansi;
        self
    }

    /// Sets the logging level
    #[must_use]
    pub const fn with_level(mut self, level: tracing::metadata::LevelFilter) -> Self {
        self.level = level;
        self
    }

    /// Sets whether it's debug mode or not
    /// This adds a lot more data about the context where an event happened.
    #[must_use]
    pub const fn with_debug_mode(mut self, debug_mode: bool) -> Self {
        self.debug_mode = debug_mode;
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ansi: false,
            level: tracing::metadata::LevelFilter::WARN,
            debug_mode: false,
        }
    }
}

/// The leafbuild tracing layer.
#[derive(Debug)]
pub struct LeafbuildTrcLayer<W = fn() -> io::Stdout>
where
    W: MakeWriter + 'static,
{
    make_writer: W,
    cfg: Config,
}

impl<W> LeafbuildTrcLayer<W>
where
    W: MakeWriter + 'static,
{
    fn should_ignore_event(&self, event: &Event<'_>) -> bool {
        // this comparison should be inverted
        // ----
        // the more verbose levels are supposed to compare greater than the less verbose ones
        // error is supposed to be the lowest
        //
        // <https://github.com/tokio-rs/tracing/blob/6b272c6c4ed02bef9c8409b8fbe0db6bc87abc12/tracing-core/src/metadata.rs#L623-L650>
        &self.cfg.level < event.metadata().level()
    }

    fn format_level<Sink>(&self, level: Level, mut sink: Sink) -> Result<(), io::Error>
    where
        Sink: FnMut(String) -> Result<(), io::Error>,
    {
        let s = level.format_with_style_if(self.cfg.ansi, |s| {
            s.fg(match level {
                tracing::Level::TRACE => Color::Cyan,
                tracing::Level::DEBUG => Color::Green,
                tracing::Level::INFO => Color::White,
                tracing::Level::WARN => Color::Yellow,
                tracing::Level::ERROR => Color::Red,
            })
        });
        sink(s)
    }

    fn format_leafbuild_mod_path<'scope, S, Sink>(
        &self,
        scopes: impl Iterator<Item = SpanRef<'scope, S>>,
        mut sink: Sink,
    ) -> Result<(), io::Error>
    where
        S: Subscriber + for<'span> LookupSpan<'span> + fmt::Debug,
        Sink: FnMut(String) -> Result<(), io::Error>,
    {
        let s = scopes
            .filter_map(|span| {
                Some(
                    span.extensions()
                        .get::<LeafbuildSpanExtension>()?
                        .modname
                        .0
                        .format_with_style_if(self.cfg.ansi, |s| s.fg(Color::Blue).bold()),
                )
            })
            .join(&" > ".format_with_style_if(self.cfg.ansi, |s| s.fg(Color::Green).bold()));

        sink(s)
    }

    fn format_event_fields<Sink>(event: &Event, mut sink: Sink) -> Result<(), io::Error>
    where
        Sink: FnMut(String) -> Result<(), io::Error>,
    {
        let mut visitor = LeafbuildFmtVisitor {
            sink: &mut sink,
            comma: false,
            err: Ok(()),
        };
        event.record(&mut visitor);

        visitor.err
    }

    fn format_debug_if_in_debug_mode<'scope, S, Sink>(
        &self,
        scopes: impl Iterator<Item = SpanRef<'scope, S>>,
        event: &Event<'_>,
        sink: Sink,
    ) -> Result<(), io::Error>
    where
        S: Subscriber + for<'span> LookupSpan<'span> + fmt::Debug,
        Sink: FnMut(String) -> Result<(), io::Error>,
    {
        if self.cfg.debug_mode {
            self.format_debug(scopes, event, sink)?;
        }

        Ok(())
    }

    fn format_debug<'scope, S, Sink>(
        &self,
        scopes: impl Iterator<Item = SpanRef<'scope, S>>,
        event: &Event<'_>,
        sink: Sink,
    ) -> Result<(), io::Error>
    where
        S: Subscriber + for<'span> LookupSpan<'span> + fmt::Debug,
        Sink: FnMut(String) -> Result<(), io::Error>,
    {
        self.format_debug_scope(scopes)
            .into_iter()
            .chain(std::iter::once(self.format_debug_event(event)))
            .try_for_each(sink)
    }

    fn format_debug_scope<'scope, S>(
        &self,
        scope: impl Iterator<Item = SpanRef<'scope, S>>,
    ) -> Vec<String>
    where
        S: Subscriber + for<'span> LookupSpan<'span> + fmt::Debug,
    {
        scope.map(|span| self.format_scope(&span)).collect()
    }

    fn format_scope<'scope, S>(&self, scope: &SpanRef<'scope, S>) -> String
    where
        S: Subscriber + for<'span> LookupSpan<'span> + fmt::Debug,
    {
        let md = scope.metadata();
        let file = md.file().unwrap_or_default();
        let line = md.line().unwrap_or_default();
        let rs_mod_path = md.module_path().unwrap_or_default();
        let name = md.name();

        format!(
            "  {} {}::{}, location={}:{}\n",
            "in".format_with_style_if(self.cfg.ansi, |s| s.fg(Color::Cyan)),
            rs_mod_path.format_with_style_if(self.cfg.ansi, |s| s.fg(Color::Green)),
            name.format_with_style_if(self.cfg.ansi, |s| s.fg(Color::Purple)),
            file.format_with_style_if(self.cfg.ansi, |s| s.fg(Color::Yellow)),
            line.format_with_style_if(self.cfg.ansi, |s| s.fg(Color::Yellow)),
        )
    }

    fn format_debug_event(&self, event: &Event<'_>) -> String {
        let md = event.metadata();
        let file = md.file().unwrap_or_default();
        let line = md.line().unwrap_or_default();
        let rs_mod_path = md.module_path().unwrap_or_default();

        format!(
            "  {}: location={}:{}, rust module path={}\n",
            "at".format_with_style_if(self.cfg.ansi, |s| s.fg(Color::Cyan)),
            file.format_with_style_if(self.cfg.ansi, |s| s.fg(Color::Yellow)),
            line.format_with_style_if(self.cfg.ansi, |s| s.fg(Color::Yellow)),
            rs_mod_path.format_with_style_if(self.cfg.ansi, |s| s.fg(Color::Green))
        )
    }
}

impl LeafbuildTrcLayer<fn() -> io::Stdout> {
    /// Creates a new leafbuild tracing layer from the configuration.
    #[must_use]
    pub fn new(cfg: Config) -> Self {
        Self {
            cfg,
            ..Self::default()
        }
    }
}

impl Default for LeafbuildTrcLayer<fn() -> io::Stdout> {
    fn default() -> Self {
        Self {
            make_writer: io::stdout,
            cfg: Config::default(),
        }
    }
}

impl<S, W> Layer<S> for LeafbuildTrcLayer<W>
where
    S: Subscriber + for<'span> LookupSpan<'span> + fmt::Debug,
    W: MakeWriter + 'static,
{
    fn new_span(&self, attrs: &Attributes, id: &Id, ctx: Context<S>) {
        let data = Data::new(attrs);
        let span = ctx.span(id).expect("in new_span but span does not exist");
        let mut ext = span.extensions_mut();
        ext.insert(data);
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        if self.should_ignore_event(event) {
            return;
        }

        self.format_event_and_write(event, ctx).unwrap();
    }

    fn on_enter(&self, id: &tracing::Id, ctx: Context<S>) {
        let span = ctx.span(id).unwrap();
        let mut ext = span.extensions_mut();
        let data = ext.get_mut::<Data>().unwrap();
        data.kvs
            .iter()
            .find_map(|(name, value)| value.as_str().take_if_owned(|_| *name == "path"))
            .and_then_do(|path| {
                ext.insert(LeafbuildSpanExtension {
                    modname: LfModName::new(path),
                })
            });
    }
}

trait FormatWithAnsi {
    fn do_with_ansi<F>(&self, ansi: bool, f: F) -> String
    where
        F: FnOnce(&Self) -> String;

    fn format_with_style_if<F>(&self, ansi: bool, f: F) -> String
    where
        F: FnOnce(Style) -> Style;
}

impl<T> FormatWithAnsi for T
where
    T: fmt::Display,
{
    fn do_with_ansi<F>(&self, ansi: bool, f: F) -> String
    where
        F: FnOnce(&Self) -> String,
    {
        if ansi {
            f(self)
        } else {
            self.to_string()
        }
    }

    fn format_with_style_if<F>(&self, ansi: bool, f: F) -> String
    where
        F: FnOnce(Style) -> Style,
    {
        self.do_with_ansi(ansi, |s| f(Style::new()).paint(s.to_string()).to_string())
    }
}

trait FormatterWithSink<Sink> {
    fn get_sink(&self) -> Sink;
    fn format_event_and_write<S>(
        &self,
        event: &Event<'_>,
        ctx: Context<'_, S>,
    ) -> Result<(), io::Error>
    where
        S: Subscriber + for<'span> LookupSpan<'span> + fmt::Debug,
    {
        self.format_event_and_write_with_sink(event, ctx, self.get_sink())
    }

    fn format_event_and_write_with_sink<S>(
        &self,
        event: &Event<'_>,
        ctx: Context<'_, S>,
        sink: Sink,
    ) -> Result<(), io::Error>
    where
        S: Subscriber + for<'span> LookupSpan<'span> + fmt::Debug;
}

struct LeafbuildTrcLayerSink<W>(BufWriter<W>)
where
    W: Write;

impl<W> LeafbuildTrcLayerSink<W>
where
    W: Write,
{
    fn sink(&mut self, s: impl AsRef<str>) -> Result<(), io::Error> {
        self.0.write_all(s.as_ref().as_bytes())
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        self.0.flush()
    }
}

impl<W> FormatterWithSink<LeafbuildTrcLayerSink<W::Writer>> for LeafbuildTrcLayer<W>
where
    W: MakeWriter + 'static,
{
    fn get_sink(&self) -> LeafbuildTrcLayerSink<W::Writer> {
        LeafbuildTrcLayerSink(BufWriter::with_capacity(
            4096,
            self.make_writer.make_writer(),
        ))
    }

    fn format_event_and_write_with_sink<S>(
        &self,
        event: &Event<'_>,
        ctx: Context<'_, S>,
        mut sink: LeafbuildTrcLayerSink<W::Writer>,
    ) -> Result<(), io::Error>
    where
        S: Subscriber + for<'span> LookupSpan<'span> + fmt::Debug,
    {
        self.format_leafbuild_mod_path(ctx.scope(), |s| sink.sink(&s))
            .and_then(|_| sink.sink(" "))
            .and_then(|_| self.format_level(*event.metadata().level(), |s| sink.sink(&s)))
            .and_then(|_| sink.sink(": "))
            .and_then(|_| Self::format_event_fields(event, |s| sink.sink(&s)))
            .and_then(|_| sink.sink("\n"))
            .and_then(|_| self.format_debug_if_in_debug_mode(ctx.scope(), event, |s| sink.sink(&s)))
            .and_then(|_| sink.flush())
    }
}
