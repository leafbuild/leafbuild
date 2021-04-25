use std::path::Path;

pub mod build;
pub mod docs;
pub mod format;
pub mod git_hooks;
pub mod grammar;
pub mod lint;
pub mod test;

extern crate cmd;
extern crate serde;
extern crate thiserror;

#[macro_export]
macro_rules! err {
    ($($t:tt),+) => {
        {
            eprintln!($($t,)+);
            std::process::exit(1)
        }
    };
}

#[macro_export]
macro_rules! ws_path {
    ($($s:literal)/ * ) => {
        {
            use std::borrow::ToOwned;
            let mut p = $crate::workspace_root().to_owned();
            $(
                p.push($s);
            )*
            p
        }
    }
}

#[macro_export]
macro_rules! ws_path_str {
    ($($s:literal)/ * ) => {
        ws_path!($($s)/ *).to_str().unwrap().to_string()
    }
}

pub fn workspace_root() -> &'static Path {
    let xtask_dir: &Path = env!("CARGO_MANIFEST_DIR").as_ref();
    xtask_dir.parent().unwrap() // parent of xtask = workspace root
}
