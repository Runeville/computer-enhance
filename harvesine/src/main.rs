mod cli;
mod generate;
mod json;

use clap::Parser;

use cli::{Cli, Command};
use generate::{Input, generate_pairs_clustered, generate_pairs_uniform};
use json::parse_json;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Setup(args) => {
            if args.uniform {
                let input = generate_pairs_uniform(args.n);
                println!("{}", serde_json::to_string(&input).unwrap());
            } else {
                let input = generate_pairs_clustered(args.n);
                println!("{}", serde_json::to_string(&input).unwrap());
            }
        }
        Command::Calculate(args) => {
            let data = std::fs::read_to_string(args.path).unwrap();
            let parsed: Input = parse_json(&data).unwrap();
            println!("{parsed:#?}");
        }
    }
}
