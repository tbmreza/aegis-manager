use aegis::{task, tui};
use clap::{Parser, StructOpt};

#[derive(StructOpt, Debug)]
struct Cli {
    #[clap(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    Start(StartArgs),
    Stop(StopArgs),
}

#[derive(StructOpt, Debug)]
struct StartArgs {
    #[clap(default_value = "0")]
    // `aegis help start` lists available profiles?
    profile: u8,
}

#[derive(StructOpt, Debug)]
struct StopArgs {
    #[clap(short)]
    interactive: bool,
}

fn main() {
    use Action::*;
    match Cli::parse() {
        Cli {
            action: Start(StartArgs { profile }),
        } => task::up_aegis_apps(profile),
        Cli {
            action: Stop(StopArgs { interactive: false }),
        } => task::stop_aegis_apps(),
        Cli {
            action: Stop(StopArgs { interactive: true }),
        } => tui::index(),
    }
}
