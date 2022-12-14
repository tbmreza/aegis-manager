use aegis::{task, tui};
use clap::{Parser, StructOpt};

#[derive(StructOpt, Debug)]
#[clap(version)]
struct Cli {
    #[clap(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    Start(StartArgs),
    Stop(StopArgs),
    #[clap(alias = "doctor")]
    Checkhealth(CheckhealthArgs),
    Deebug,
}

#[derive(StructOpt, Debug)]
struct StartArgs {
    #[clap(
        default_value = "0",
        help = "1 Frontend, 11 Alunalun, 0 All aegis containers"
    )]
    profile: u8,
}

#[derive(StructOpt, Debug)]
struct StopArgs {
    #[clap(short, long, help = "Open interactive list of aegis containers")]
    interactive: bool,
}

#[derive(StructOpt, Debug)]
struct CheckhealthArgs {
    #[clap(short, help = "Yes to all suggested autofixes")]
    yes_to_all: bool,
}

fn main() {
    use Action::*;
    match Cli::parse() {
        Cli { action: Deebug } => {}
        Cli {
            action: Checkhealth(CheckhealthArgs { yes_to_all }),
        } => task::checkhealth::run(yes_to_all),
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
