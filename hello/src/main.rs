/* Command Line Interface tool for Marco Polo */

use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Nigel Malaba",
    about = "A CLI tool for Marco Polo"
)]

struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]

enum Commands {
    #[clap(
        version = "1.0",
        author = "Nigel Malaba",
        about = "A CLI tool for Marco Polo"
    )]
    Marco {
        #[clap(short, long)]
        name: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Marco { name }) => {
            let result = hello::marco_polo(&name);
            println!("{}", result);
        }
        None => println!("No command was used"),
    }
}
