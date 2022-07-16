use clap::{Parser,Subcommand};
use clap::crate_version;

#[derive(Parser, Debug)]
#[clap(name = "catalysis")]
#[clap(version = crate_version!())]
#[clap(about = "A CLI tool to deploy projects through a Catalysis instance", long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
   commands: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(about = "Manage authentication sessions")]
    Auth {
        #[clap(subcommand)]
        commands: AuthCommands
    },

    #[clap(about = "Run a new deployment")]
    Deploy {}
}

#[derive(Subcommand, Debug)]
enum AuthCommands {
    #[clap(about = "Authenticate with an instance of Catalysis")]
    Login {

    },
    #[clap(about = "Revoke an existing session and delete it locally")]
    Logout {

    }
}

pub fn run() {
    let args = Cli::parse();

}