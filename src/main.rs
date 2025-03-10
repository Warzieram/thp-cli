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
   
    Submit(commands::submit::SubmitArgs)
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Submit(args) => commands::submit::run(args),
    }
}

