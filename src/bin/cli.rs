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
    profile: u8,
}

#[derive(StructOpt, Debug)]
struct StopArgs {
    #[clap(short)]
    interactive: bool,
}

fn main() {
    let args = Cli::parse();
    println!("{:#?}", args);
}
