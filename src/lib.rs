use clap::Parser;

mod cli;
mod credentials;

pub fn run() {
    let args = cli::CLI::parse();
}
