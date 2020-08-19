extern crate clap;
extern crate libleaf;

use std::path::Path;
use std::process::exit;

use clap::{App, AppSettings, Arg};

use libleaf::interpreter::EnvConfig;
use libleaf::{handle::Handle, interpreter};
use log::LevelFilter;
use pretty_env_logger::env_logger::WriteStyle;

const VERSION: &str = env!("CARGO_PKG_VERSION");

macro_rules! arg_names {
    ($($name:ident = $val:literal),* $(,)?) => {
    $(
        const $name: &str = $val;
    )*
    };
}

arg_names! {
    // internals
    EXIT_CODE = "Exit code",
    IN = "In",
    OUT = "Out",
    MOD_ID = "Module id",


    // build
    DIRECTORY = "Directory",
    ROOT = "Root",
    OUTPUT_DIRECTORY = "Output directory",
    ANGRY_ERRORS = "Angry errors",
    DISABLE_ERROR_CASCADE = "Disable error cascade",
    ENABLE_CI_OPTIONS = "Enable CI options",
    ENABLE_BUILD_FAILURE_SIGNALS = "Enable build failure signals",
}

fn main() {
    pretty_env_logger::formatted_timed_builder()
        .write_style(WriteStyle::Always)
        .filter_level(LevelFilter::Trace)
        .init();
    let angry_errors_help = format!(
        "Makes all error messages uppercase{}",
        if cfg!(feature = "angry-errors") {
            ""
        } else {
            "(requires the `angry-errors` feature)"
        }
    );
    let app = App::new("leafbuild")
        .setting(AppSettings::ColoredHelp)
        .version(VERSION)
        .author("Dinu Blanovschi <dinu.blanovschi@criptext.com>")
        .about("Automates C/C++ builds")
        .subcommand(
            App::new("internal")
                .version(VERSION)
                .subcommand(
                    App::new("compilation-failed")
                        .arg(
                            Arg::with_name(EXIT_CODE)
                                .about("The exit code of the compilation process")
                                .long("exit-code")
                                .takes_value(true)
                                .long("exit-code")
                                .required(true)
                        ).arg(
                        Arg::with_name(IN)
                            .about("Compilation inputs")
                            .long("in")
                            .takes_value(true)
                            .required(true)
                    ).arg(
                        Arg::with_name(OUT)
                            .about("Compilation outputs")
                            .long("out")
                            .takes_value(true)
                            .required(true)
                    ).arg(
                        Arg::with_name(MOD_ID)
                            .long("module-id")
                            .takes_value(true)
                            .required(true)
                    )
                )
                .subcommand(
                    App::new("link-failed")
                        .arg(
                            Arg::with_name(EXIT_CODE)
                                .about("The exit code of the compilation process")
                                .long("exit-code")
                                .takes_value(true)
                                .long("exit-code")
                                .required(true)
                        ).arg(
                        Arg::with_name(IN)
                            .about("Link inputs")
                            .long("in")
                            .takes_value(true)
                            .required(true)
                    ).arg(
                        Arg::with_name(OUT)
                            .about("Link outputs")
                            .long("out")
                            .takes_value(true)
                            .required(true)
                    ).arg(
                        Arg::with_name(MOD_ID)
                            .long("module-id")
                            .takes_value(true)
                            .required(true)
                    )
                )
        ).subcommand(App::new("build").about("Creates a build directory").setting(AppSettings::ColoredHelp).arg(
            Arg::with_name(DIRECTORY).short('d').long("dir").takes_value(true).about("The directory containing a leafbuild project to start from"),
            ).arg(
                Arg::with_name(ROOT).short('r').long("root").takes_value(true).about(
                    "The directory at the root of everything. It is used to find the path of the source\
                                file for the SRCFILE define",
                ),
            ).arg(
                Arg::with_name(OUTPUT_DIRECTORY).short('o').long("output-dir").takes_value(true).default_value("leafbuild-dir").about("The place where the build system will live; note that if it contains a `/`, the paths to the sources will all be absolute")
            ).arg(
                Arg::with_name(ANGRY_ERRORS).long("angry-errors").takes_value(false).about(&angry_errors_help),
            ).arg(
                Arg::with_name(DISABLE_ERROR_CASCADE).long("disable-error-cascade").takes_value(false).about("Disables error cascades")
            ).arg(
                Arg::with_name(ENABLE_CI_OPTIONS).long("ci")
                    .about("Enables all options used for CI; can be enabled manually(equivalent to): --enable-build-failure-signals")
                    .conflicts_with_all(&["Enable build failure signals"])
            ).arg(
                Arg::with_name(ENABLE_BUILD_FAILURE_SIGNALS).long("enable-build-failure-signals").about("Enables build failure signals in the build directory; part of the CI feature")
            )
        );

    let matches = app.get_matches();

    let (name, m) = matches.subcommand();
    match name {
        "build" => {
            let m = m.unwrap();

            let wd = std::env::current_dir().unwrap();
            let proj_path = match m.value_of(DIRECTORY) {
                Some(path) => Path::new(path),
                None => wd.as_path(),
            };
            #[allow(unused_mut)]
            let mut config = EnvConfig::new();
            #[cfg(feature = "angry-errors")]
            config.set_angry_errors(m.is_present(ANGRY_ERRORS));
            if m.is_present(ANGRY_ERRORS) && !(cfg!(feature = "angry-errors")) {
                println!(
                    "\x1B[4;33m[WARN]\x1B[0m Cannot use --angry-errors without the angry-errors feature"
                );
            }

            if m.is_present(DISABLE_ERROR_CASCADE) {
                config.set_error_cascade(false);
            }

            config.set_output_directory(
                m.value_of(OUTPUT_DIRECTORY)
                    .expect("Couldn't get output directory"),
            );

            let ci_enabled = m.is_present(ENABLE_CI_OPTIONS);

            config
                .set_signal_build_failure(ci_enabled || m.is_present(ENABLE_BUILD_FAILURE_SIGNALS));

            let mut handle = Handle::new(config);
            interpreter::start_on(&proj_path, &mut handle);
        }
        "internal" => {
            let m = m.unwrap();
            let (name, m) = m.subcommand();
            match name {
                "compilation-failed" => {
                    let m = m.unwrap();
                    let exit_code = m.value_of(EXIT_CODE).unwrap();
                    let in_ = m.value_of(IN).unwrap();
                    let out = m.value_of(OUT).unwrap();
                    let mod_id = m.value_of(MOD_ID).unwrap();
                    println!(
                        "exit code = {}, in = '{}', out = '{}', mod_id = {}",
                        exit_code, in_, out, mod_id
                    );
                    exit(exit_code.parse().unwrap());
                }
                "link-failed" => {
                    let m = m.unwrap();
                    let exit_code = m.value_of(EXIT_CODE).unwrap();
                    let in_ = m.value_of(IN).unwrap();
                    let out = m.value_of(OUT).unwrap();
                    let mod_id = m.value_of(MOD_ID).unwrap();
                    println!(
                        "exit code = {}, in = '{}', out = '{}', mod_id = {}",
                        exit_code, in_, out, mod_id
                    );
                    exit(exit_code.parse().unwrap());
                }
                _ => {}
            }
        }
        // clap already handles errors
        _ => {}
    }
}
