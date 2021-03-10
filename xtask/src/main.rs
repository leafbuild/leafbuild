use clap::Clap;
use std::error::Error;
use std::path::PathBuf;

#[derive(Clap)]
enum Xtask {
    #[clap(name = "install-git-hooks")]
    InstallGitHooks,

    #[clap(name = "prepare-commit")]
    PrepareCommit,

    #[clap(name = "verify-commit-message")]
    VerifyCommitMessage {
        #[clap(parse(try_from_str))]
        path: PathBuf,
    },

    #[clap(name = "test")]
    Test { args: Vec<String> },

    #[clap(name = "fmt")]
    Fmt,

    #[clap(name = "format-check")]
    FmtCheck,

    #[clap(name = "generate-parser-tests")]
    GenerateParserTests,

    #[clap(name = "lint")]
    Lint,
}

fn main() -> Result<(), Box<dyn Error>> {
    let task: Xtask = Xtask::parse();
    match task {
        Xtask::InstallGitHooks => xtask::git_hooks::install_git_hooks()?,
        Xtask::PrepareCommit => xtask::git_hooks::prepare_commit(),
        Xtask::VerifyCommitMessage { path } => xtask::git_hooks::verify_commit_message(path),
        Xtask::Test { args } => xtask::test::run_tests(args)?,
        Xtask::GenerateParserTests => xtask::test::generate_parser_tests()?,
        Xtask::Fmt => xtask::format::format()?,
        Xtask::FmtCheck => xtask::format::format_check()?,
        Xtask::Lint => xtask::lint::lint()?,
    }

    Ok(())
}
