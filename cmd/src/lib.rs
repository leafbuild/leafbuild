use std::process::ExitStatus;
use std::{fmt, io};
#[macro_export]
macro_rules! cmd_process {
    ($s:expr) => {{
        let mut cmd: ::std::process::Command;
        if cfg!(target_os = "windows") {
            cmd = ::std::process::Command::new("cmd");
            cmd.args(&["/C", $s]);
        } else {
            cmd = ::std::process::Command::new("sh");
            cmd.arg("-c").arg($s);
        }

        cmd
    }};

    ($s:expr, workdir = $wd:expr) => {{
        let mut cmd: ::std::process::Command;
        if cfg!(target_os = "windows") {
            cmd = ::std::process::Command::new("cmd");
            cmd.args(&["/C", $s]);
        } else {
            cmd = ::std::process::Command::new("sh");
            cmd.arg("-c").arg($s);
        }

        cmd.current_dir($wd);

        cmd
    }};
}

#[macro_export]
macro_rules! cmd {
    ($s:expr) => {{
        $crate::cmd_process!($s)
            .output()
            .expect("failed to execute process")
    }};

    ($s:expr, workdir = $wd:expr) => {{
        $crate::cmd_process!($s, workdir = $wd)
            .output()
            .expect("failed to execute process")
    }};
}

#[macro_export]
macro_rules! cmd_call {
    ($s:expr) => {
        (|| -> $crate::CmdResult {
            let exit_status = $crate::cmd_process!($s).spawn()?.wait()?;
            if exit_status.success() {
                Ok(())
            } else {
                Err($crate::CmdError::NotSuccess(exit_status))
            }
        })()
    };

    ($s:expr, workdir = $wd:expr) => {
        (|| -> $crate::CmdResult {
            let exit_status = $crate::cmd_process!($s, workdir = $wd).spawn()?.wait()?;
            if exit_status.success() {
                Ok(())
            } else {
                Err($crate::CmdError::NotSuccess(exit_status))
            }
        })()
    };
}

#[macro_export]
macro_rules! cargo_cmd {
    ($s:literal) => {
        $crate::cmd_call!(concat!(env!("CARGO"), " ", $s))
    };
    ($s:expr) => {
        $crate::cmd_call!(&format!(concat!(env!("CARGO"), " {}"), $s))
    };
}

#[derive(Debug)]
pub enum CmdError {
    IoError(io::Error),

    NotSuccess(ExitStatus),
}

impl fmt::Display for CmdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(err) => write!(f, "io error: {}", err),
            Self::NotSuccess(status) => write!(f, "not success: {}", status),
        }
    }
}

impl From<io::Error> for CmdError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl std::error::Error for CmdError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::IoError(err) => Some(err),
            Self::NotSuccess(_) => None,
        }
    }
}

pub type CmdResult<T = ()> = Result<T, CmdError>;
