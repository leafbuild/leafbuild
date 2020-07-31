extern crate clap;
extern crate libleaf;
extern crate libleafcore;

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
        Arg::with_name("Directory").short('d').long("dir").takes_value(true)
            .about("The directory containing a leafbuild project to start from"),
    ).arg(
        Arg::with_name("Root").short('r').long("root").takes_value(true).about(
            "The directory at the root of everything. It is used to find the path of the source\
                        file for the SRCFILE define",
        ),
    ).arg(
        Arg::with_name("Angry errors")
            .long("angry-errors")
            .takes_value(false)
            .about(&angry_errors_help),
    ).arg(
        Arg::with_name("")
    )
        ;

    let matches = app.get_matches();
    let wd = std::env::current_dir().unwrap();
    let proj_path = match matches.value_of("Directory") {
        Some(path) => Path::new(path),
        None => wd.as_path(),
    };
    let mut config: EnvConfig = EnvConfig::new();
    #[cfg(feature = "angry-errors")]
    config.set_angry_errors(matches.is_present("Angry errors"));
    if matches.is_present("Angry errors") && !(cfg!(feature = "angry-errors")) {
        println!(
            "\x1B[4;33m[WARN]\x1B[0m Cannot use --angry-errors without the angry-errors feature"
        );
    }
    let mut handle = Handle::new(config);
    interpreter::start_on(&proj_path, &mut handle);
}
