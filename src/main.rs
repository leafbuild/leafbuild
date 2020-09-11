use std::path::{Path, PathBuf};
use std::process::exit;

use leafbuild::handle::Handle;
use leafbuild::interpreter;
use leafbuild::interpreter::EnvConfig;
use log::LevelFilter;
use pretty_env_logger::env_logger::WriteStyle;
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct BuildCommand {
    #[structopt(short, long = "dir", parse(from_os_str), default_value = ".")]
    directory: PathBuf,
    #[structopt(
        short = "o",
        long = "output-dir",
        parse(from_os_str),
        default_value = "leafbuild-dir"
    )]
    output_directory: PathBuf,
    #[structopt(long = "angry-errors")]
    angry_errors: bool,
    #[structopt(long = "disable-error-cascade")]
    disable_error_cascade: bool,
    #[structopt(long = "ci")]
    ci_enabled: bool,

    #[structopt(long = "build-failure-signals")]
    build_failure_signals: bool,
}

#[derive(Debug, StructOpt)]
enum InternalSubcommand {
    #[structopt(name = "compilation-failed")]
    CompilationFailed {
        #[structopt(long = "exit-code")]
        exit_code: i32,
        #[structopt(long = "in")]
        in_: String,
        #[structopt(long = "out")]
        out: String,
        #[structopt(long = "module-id")]
        mod_id: usize,
    },
    #[structopt(name = "link-failed")]
    LinkFailed {
        #[structopt(long = "exit-code")]
        exit_code: i32,
        #[structopt(long = "in")]
        in_: String,
        #[structopt(long = "out")]
        out: String,
        #[structopt(long = "module-id")]
        mod_id: usize,
    },
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    Build {
        #[structopt(flatten)]
        build_command: BuildCommand,
    },
    Internal {
        #[structopt(flatten)]
        internal_subcommand: InternalSubcommand,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "leafbuild",
    about = "Automates C/C++ builds",
    setting = AppSettings::ColoredHelp,
)]
struct Cli {
    #[structopt(flatten)]
    subcommand: Subcommand,
}

fn main() {
    pretty_env_logger::formatted_timed_builder()
        .write_style(WriteStyle::Always)
        .filter_level(LevelFilter::Trace)
        .init();
    let cli: Cli = Cli::from_args();

    match cli.subcommand {
        Subcommand::Build { build_command } => {
            let _wd = std::env::current_dir().unwrap();
            let proj_path = Path::new(&build_command.directory);
            #[allow(unused_mut)]
            let mut config = EnvConfig::new();
            #[cfg(feature = "angry-errors")]
            config.set_angry_errors(build_command.angry_errors);
            if build_command.angry_errors && !(cfg!(feature = "angry-errors")) {
                println!(
                    "\x1B[4;33m[WARN]\x1B[0m Cannot use --angry-errors without the angry-errors feature"
                );
            }

            if build_command.disable_error_cascade {
                config.set_error_cascade(false);
            }

            config.set_output_directory(build_command.output_directory);

            let ci_enabled = build_command.ci_enabled;

            config.set_signal_build_failure(ci_enabled || build_command.build_failure_signals);

            let mut handle = Handle::new(config);
            interpreter::start_on(&proj_path, &mut handle);
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
                println!(
                    "exit code = {}, in = '{}', out = '{}', mod_id = {}",
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
                println!(
                    "exit code = {}, in = '{}', out = '{}', mod_id = {}",
                    exit_code, in_, out, mod_id
                );
                exit(exit_code);
            }
        },
    }
}
