use std::io;
use std::process::ExitStatus;
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
}

#[macro_export]
macro_rules! cmd {
    ($s:expr) => {{
        $crate::cmd_process!($s)
            .output()
            .expect("failed to execute process")
    }};
}

#[macro_export]
macro_rules! cmd_call {
    ($s:expr) => {
        (|| -> $crate::cmd::CmdResult {
            let exit_status = $crate::cmd_process!($s).spawn()?.wait()?;
            if exit_status.success() {
                Ok(())
            } else {
                Err($crate::cmd::CmdError::NotSuccess(exit_status))
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
        $crate::cmd_call!(&format!("{} {}", env!("CARGO"), $s))
    };
}

#[derive(Debug, Error)]
pub enum CmdError {
    #[error("io error: {0}")]
    IoError(#[from] io::Error),

    #[error("not success: {0}")]
    NotSuccess(ExitStatus),
}

pub type CmdResult<T = ()> = Result<T, CmdError>;
