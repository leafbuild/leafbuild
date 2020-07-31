extern crate libleaf;
extern crate libleafcore;

use clap::{App, Arg};
use libleaf::{handle::Handle, interpreter};
use std::path::Path;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut handle = Handle::new();
    let app = App::new("leafbuild").version(VERSION).author("Dinu Blanovschi <dinu.blanovschi@criptext.com>").about("Automates C/C++ builds").arg(
        Arg::with_name("Directory").short('d').long("dir").takes_value(true).about("The directory containing a leafbuild project to start from"),
    ).arg(
        Arg::with_name("Root").short('r').long("root").takes_value(true).about(
            "The directory at the root of everything. It is used to find the path of the source\
                        file for the SRCFILE define",
        ),
    );
    #[cfg(feature = "angry-errors")]
    let app = app.arg(
        Arg::with_name("Angry errors")
            .long("angry-errors")
            .takes_value(false)
            .about("Makes all error messages uppercase"),
    );

    let matches = app.get_matches();
    let wd = std::env::current_dir().unwrap();
    let proj_path = match matches.value_of("Directory") {
        Some(path) => Path::new(path),
        None => wd.as_path(),
    };
    #[cfg(feature = "angry-errors")]
    handle.set_angry_errors_mode(matches.is_present("Angry errors"));
    interpreter::start_on(&proj_path, &mut handle);
}
