use clap::Parser;
use clap::crate_version;

#[derive(Parser, Debug)]
#[clap(name = "catalysis")]
#[clap(version = crate_version!())]
#[clap(about = "A CLI tool to deploy projects through a Catalysis instance", long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    /// Name of the person to greet
    #[clap(short, long, value_parser)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, value_parser, default_value_t = 1)]
    count: u8,
}


pub fn run() {
    let args = Cli::parse();
    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}