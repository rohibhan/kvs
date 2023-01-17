use clap::{Args, Parser, Subcommand};
use std::process::exit;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    ///set key and value
    Set(Set),
    Get(Get),
    Rm(Get),
}

#[derive(Args)]
struct Set {
    ///set a key string
    key1: Option<String>,
    ///set a value string
    value1: Option<String>,
}

#[derive(Args)]
struct Get {
    ///set a key string
    key1: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match &&cli.command {
        Command::Set(_) => {
            eprintln!("unimplemented");
            exit(1);
        }
        Command::Get(_) => {
            eprintln!("unimplemented");
            exit(1);
        }
        Command::Rm(_) => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
}
