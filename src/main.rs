use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser, Debug)]
#[command(name = "my_cli", version = "1.0", about = "Une CLI avec des sous-commandes", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
   
    Submit(commands::submit::SubmitArgs),
    Connect(commands::connect::ConnectArgs)
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Submit(args) => commands::submit::run(args),
        Commands::Connect(args) => commands::connect::run(args),
    }
}

