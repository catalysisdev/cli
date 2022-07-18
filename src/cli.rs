use clap::crate_version;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "catalysis")]
#[clap(version = crate_version!())]
#[clap(about = "A CLI tool to deploy projects through a Catalysis instance", long_about = None)]
#[clap(propagate_version = true)]
pub struct CLI {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(about = "Manage authentication sessions")]
    Auth {
        #[clap(subcommand)]
        commands: AuthCommands,
    },

    #[clap(about = "Run a new deployment")]
    Deploy {},
}

#[derive(Subcommand, Debug)]
enum AuthCommands {
    #[clap(about = "Authenticate with an instance of Catalysis")]
    Login {},
    #[clap(about = "Revoke an existing session and delete it locally")]
    Logout {},
}
