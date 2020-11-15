//! Module containing logic to format [`tracing`] events.
use crate::interpreter::LfModName;
use ansi_term::{Color, Style};
use itertools::Itertools;
use std::error::Error;
use std::fmt::Debug;
use std::io::Write as _;
use std::{fmt, io};
use tracing::field::{Field, Visit};
use tracing::span::Attributes;
use tracing::{Event, Id, Subscriber};
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;
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

struct LeafbuildSpanExtension(LfModName);

struct LeafbuildFmtVisitor<'a, W>
where
    W: io::Write,
{
    writer: &'a mut W,
    comma: bool,
}

impl<'a, W> Visit for LeafbuildFmtVisitor<'a, W>
where
    W: io::Write,
{
    fn record_i64(&mut self, field: &Field, value: i64) {
        write!(self.writer, "{}={} ", field.name(), value).unwrap();
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        write!(self.writer, "{}={} ", field.name(), value).unwrap();
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        write!(self.writer, "{}={} ", field.name(), value).unwrap();
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        write!(self.writer, "{}={} ", field.name(), value).unwrap();
    }

    fn record_error(&mut self, field: &Field, value: &dyn Error) {
        write!(self.writer, "{}={} ", field.name(), value).unwrap();
    }

    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        let name = field.name();
        if name == "message" {
            write!(self.writer, "{:?} ", value).unwrap();
            self.comma = true;
        } else {
            write!(self.writer, "{}={:?} ", name, value).unwrap();
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
        let level = event.metadata().level();
        // this comparison should be inverted
        // ----
        // the more verbose levels are supposed to compare greater than the less verbose ones
        // error is supposed to be the lowest
        //
        // <https://github.com/tokio-rs/tracing/blob/6b272c6c4ed02bef9c8409b8fbe0db6bc87abc12/tracing-core/src/metadata.rs#L623-L650>
        if &self.cfg.level < level {
            return;
        }

        let level_str = level.format_with_style(self.cfg.ansi, |s| match *level {
            tracing::Level::TRACE => s.fg(Color::Cyan),
            tracing::Level::DEBUG => s.fg(Color::Green),
            tracing::Level::INFO => s.fg(Color::White),
            tracing::Level::WARN => s.fg(Color::Yellow),
            tracing::Level::ERROR => s.fg(Color::Red),
        });

        let path_sep = " > ".format_with_style(self.cfg.ansi, |s| s.bold().fg(Color::Green));
        let mod_path = ctx
            .scope()
            .filter_map(|span| {
                span.extensions()
                    .get::<LeafbuildSpanExtension>()
                    .map(|extension| {
                        extension
                            .0
                             .0
                            .format_with_style(self.cfg.ansi, |s| s.fg(Color::Blue).bold())
                    })
            })
            .join(&path_sep);
        let mut w = self.make_writer.make_writer();

        write!(w, "{} {}: ", mod_path, level_str).unwrap();

        event.record(&mut LeafbuildFmtVisitor {
            writer: &mut w,
            comma: false,
        });

        writeln!(w).unwrap();
        if self.cfg.debug_mode {
            ctx.scope().for_each(|span_ref| {
                let md = span_ref.metadata();
                let file = md.file().unwrap_or_default();
                let line = md.line().unwrap_or_default();
                let rs_mod_path = md.module_path().unwrap_or_default();
                let name = md.name();

                writeln!(
                    w,
                    "  {} {}::{}, location={}:{}",
                    "in".format_with_style(self.cfg.ansi, |s| s.fg(Color::Cyan)),
                    rs_mod_path.format_with_style(self.cfg.ansi, |s| s.fg(Color::Green)),
                    name.format_with_style(self.cfg.ansi, |s| s.fg(Color::Purple)),
                    file.format_with_style(self.cfg.ansi, |s| s.fg(Color::Yellow)),
                    line.format_with_style(self.cfg.ansi, |s| s.fg(Color::Yellow)),
                )
                .unwrap();
            });

            let file = event.metadata().file().unwrap_or_default();
            let line = event.metadata().line().unwrap_or_default();
            let rs_mod_path = event.metadata().module_path().unwrap_or_default();

            writeln!(
                w,
                "  {}: location={}:{}, rust module path={}",
                "at".format_with_style(self.cfg.ansi, |s| s.fg(Color::Cyan)),
                file.format_with_style(self.cfg.ansi, |s| s.fg(Color::Yellow)),
                line.format_with_style(self.cfg.ansi, |s| s.fg(Color::Yellow)),
                rs_mod_path.format_with_style(self.cfg.ansi, |s| s.fg(Color::Green))
            )
            .unwrap();
        }
    }

    fn on_enter(&self, id: &tracing::Id, ctx: Context<S>) {
        let span = ctx.span(id).unwrap();
        let mut ext = span.extensions_mut();
        let data = ext.get_mut::<Data>().unwrap();
        let path = data.kvs.iter().find_map(|(name, value)| {
            if *name == "path" {
                Some(value.clone())
            } else {
                None
            }
        });
        if let Some(path) = path {
            ext.insert(LeafbuildSpanExtension(LfModName::new(path)));
        }
    }
}

trait FormatWithAnsi {
    fn do_with_ansi<F>(&self, ansi: bool, f: F) -> String
    where
        F: FnOnce(&Self) -> String;

    fn format_with_style<F>(&self, ansi: bool, f: F) -> String
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

    fn format_with_style<F>(&self, ansi: bool, f: F) -> String
    where
        F: FnOnce(Style) -> Style,
    {
        self.do_with_ansi(ansi, |s| f(Style::new()).paint(s.to_string()).to_string())
    }
}
