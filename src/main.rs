extern crate clap;
extern crate libleaf;

use clap::{App, Arg};
use libleaf::interpreter::EnvConfig;
use libleaf::{handle::Handle, interpreter};
use std::path::Path;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let angry_errors_help = format!(
        "Makes all error messages uppercase{}",
        if cfg!(feature = "angry-errors") {
            ""
        } else {
            "(requires the `angry-errors` feature)"
        }
    );
    let app = App::new("leafbuild").version(VERSION).author("Dinu Blanovschi <dinu.blanovschi@criptext.com>").about("Automates C/C++ builds").arg(
        Arg::with_name("Directory").short('d').long("dir").takes_value(true).about("The directory containing a leafbuild project to start from"),
    ).arg(
        Arg::with_name("Root").short('r').long("root").takes_value(true).about(
            "The directory at the root of everything. It is used to find the path of the source\
                        file for the SRCFILE define",
        ),
    ).arg(
        Arg::with_name("Output directory").short('o').long("output-dir").takes_value(true).default_value("leafbuild-dir").about("The place where the build system will live; note that if it contains a `/`, the paths to the sources will all be absolute")
    ).arg(
        Arg::with_name("Angry errors").long("angry-errors").takes_value(false).about(&angry_errors_help),
    ).arg(
        Arg::with_name("Disable error cascade").long("disable-error-cascade").takes_value(false).about("Disables error cascades")
    );

    let matches = app.get_matches();
    let wd = std::env::current_dir().unwrap();
    let proj_path = match matches.value_of("Directory") {
        Some(path) => Path::new(path),
        None => wd.as_path(),
    };
    #[allow(unused_mut)]
    let mut config = EnvConfig::new();
    #[cfg(feature = "angry-errors")]
    config.set_angry_errors(matches.is_present("Angry errors"));
    if matches.is_present("Angry errors") && !(cfg!(feature = "angry-errors")) {
        println!(
            "\x1B[4;33m[WARN]\x1B[0m Cannot use --angry-errors without the angry-errors feature"
        );
    }

    if matches.is_present("Disable error cascade") {
        config.set_error_cascade(false);
    }

    config.set_output_directory(
        matches
            .value_of("Output directory")
            .expect("Couldn't get output directory"),
    );

    let mut handle = Handle::new(config);
    interpreter::start_on(&proj_path, &mut handle);
}
