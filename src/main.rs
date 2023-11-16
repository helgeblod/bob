extern crate clap;

use clap::{Parser, Subcommand};

mod languages;
mod cmd;

#[derive(Parser)]
#[command(author, version, about = "Build command shortcuts", long_about = "Utility for running build commands for different build systems")]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build code ⚒️
    Build {},

    /// Clean artifacts 🧹
    Clean {},

    /// Run code 🚀
    Run {},

    /// Run tests 🧪
    Test {},
}

fn main() {
    let cli = Cli::parse();
    let languages = languages::add_languages();

    match &cli.command {
        Commands::Build {} => {
            cmd::exec_build(&languages);
        }
        Commands::Clean {} => {
            cmd::exec_clean(&languages);
        }
        Commands::Run {} => {
            cmd::exec_run(&languages);
        }
        Commands::Test {} => {
            cmd::exec_test(languages);
        }
    }
}
