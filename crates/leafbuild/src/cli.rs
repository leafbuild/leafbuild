//! Definition and parsing of Cli.
use clap::{AppSettings, Clap};
use leafbuild_core::lf_buildsys::config::Config;
use leafbuild_interpreter::handle::Handle;
use leafbuild_interpreter::LfModName;
use std::path::{Path, PathBuf};
use std::process::exit;

/// The build command.
#[derive(Debug, Clap)]
#[clap(setting(AppSettings::ColoredHelp))]
pub struct BuildCommand {
    /// The directory where the root `build.leaf` is.
    #[clap(short, long = "dir", parse(from_os_str), default_value = ".")]
    pub directory: PathBuf,
    /// The output directory to use for the build system.
    #[clap(
        short = 'o',
        long = "output-dir",
        parse(from_os_str),
        default_value = "leafbuild-dir"
    )]
    pub output_directory: PathBuf,
    // Options
    /// Disables "error cascades"
    #[clap(long = "disable-error-cascade")]
    pub disable_error_cascade: bool,
    /// Builds in CI mode
    #[clap(long = "ci")]
    pub ci_enabled: bool,

    /// Adds build failure signals(internal subcommand calls if compilation fails)
    #[clap(long = "build-failure-signals")]
    pub build_failure_signals: bool,
}

/// The internal subcommand.
/// Used internally to tell `leafbuild` that a certain compilation / linking command failed and so
/// it can report it.
#[derive(Debug, Clap)]
#[clap(setting(AppSettings::Hidden), setting(AppSettings::ColoredHelp))]
pub enum InternalSubcommand {
    /// Invoked when compilation failed.
    #[clap(name = "compilation-failed", setting(AppSettings::ColoredHelp))]
    CompilationFailed {
        /// The exit code of the compiler process.
        #[clap(long = "exit-code")]
        exit_code: i32,
        /// The input files.
        #[clap(long = "in")]
        in_: String,
        /// The output object file.
        #[clap(long = "out")]
        out: String,
        /// The id of the leafbuild module this target is a part of.
        #[clap(long = "module-id")]
        mod_id: usize,
    },
    /// Invoked when linking failed.
    #[clap(name = "link-failed", setting(AppSettings::ColoredHelp))]
    LinkFailed {
        /// The exit code of the linker process.
        #[clap(long = "exit-code")]
        exit_code: i32,
        /// The input object files to the linker.
        #[clap(long = "in")]
        in_: String,
        /// The output executable / shared library that should have been produced.
        #[clap(long = "out")]
        out: String,
        /// The id of the leafbuild module this target is a part of.
        #[clap(long = "module-id")]
        mod_id: usize,
    },
}

///
#[derive(Debug, Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub enum Subcommand {
    /// The build subcommand
    Build {
        /// The build command
        #[clap(flatten)]
        build_command: BuildCommand,
    },
    /// The internal subcommand
    Internal {
        /// The internal command
        #[clap(flatten)]
        internal_subcommand: InternalSubcommand,
    },
}

///
#[derive(Debug, Clap)]
#[clap(
    name = "leafbuild",
    about = "Cross-platform C/C++ buildsystem",
    setting = AppSettings::ColoredHelp,
)]
pub struct Cli {
    /// The log level to use
    #[clap(default_value = "warn", parse(try_from_str))]
    pub log_level: tracing::metadata::LevelFilter,

    /// To use ansi output
    #[clap(long)]
    pub ansi: bool,

    /// Debug mode
    #[clap(long)]
    pub debug: bool,

    /// The subcommand
    #[clap(subcommand)]
    pub subcommand: Subcommand,
}

/// Runs the given cli
pub fn run(cli: Cli) {
    match cli.subcommand {
        Subcommand::Build { build_command } => {
            let _wd = std::env::current_dir().unwrap();
            let proj_path = Path::new(&build_command.directory);
            let ci_enabled = build_command.ci_enabled;
            let config = Config::new(
                !build_command.disable_error_cascade,
                build_command.output_directory,
                ci_enabled || build_command.build_failure_signals,
            );

            let mut handle = Handle::new(config);
            let path_buf = proj_path.to_path_buf();
            leafbuild_interpreter::execute_on(
                &mut handle,
                &path_buf,
                LfModName::new(
                    path_buf
                        .file_name()
                        .map(|it| it.to_string_lossy().to_string())
                        .or_else(|| {
                            Some(
                                std::env::current_dir()
                                    .ok()?
                                    .file_name()
                                    .map(|it| it.to_string_lossy().to_string())?,
                            )
                        })
                        .unwrap_or_else(|| ".".into()),
                ),
            )
            .and_then(|h| Ok(h.validate()?))
            .and_then(|h| Ok(h.write_results()?))
            .map_or_else(
                |error| {
                    error!("An error occurred: {}", error);
                },
                |_| (),
            );
        }
        Subcommand::Internal {
            internal_subcommand,
        } => match internal_subcommand {
            InternalSubcommand::CompilationFailed {
                exit_code,
                in_,
                out,
                mod_id,
            } => {
                error!(
                    "compilation failed: exit code = {}, in = '{}', out = '{}', mod_id = {}",
                    exit_code, in_, out, mod_id
                );
                exit(exit_code);
            }
            InternalSubcommand::LinkFailed {
                exit_code,
                in_,
                out,
                mod_id,
            } => {
                error!(
                    "link failed: exit code = {}, in = '{}', out = '{}', mod_id = {}",
                    exit_code, in_, out, mod_id
                );
                exit(exit_code);
            }
        },
    }
}
