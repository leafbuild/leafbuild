#![crate_type = "bin"]

use clap::Clap;
use leafbuild::cli::Cli;
use leafbuild::trc::{Config, LeafbuildTrcLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Registry;

fn main() {
    let cli: Cli = Cli::parse();
    let subscriber = Registry::default().with(LeafbuildTrcLayer::new(
        Config::default()
            .with_ansi(cli.ansi || atty::is(atty::Stream::Stdout))
            .with_level(cli.log_level)
            .with_debug_mode(cli.debug),
    ));
    subscriber.init();

    leafbuild::run(cli);
}
