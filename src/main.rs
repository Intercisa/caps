mod cap;
mod img;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[clap(long, short, action)]
    reverse: bool,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Capitalize the input sentence")]
    CAP { sentence: Option<Vec<String>> },
    #[command(about = "Preview the input image")]
    IMG { path: Option<String> },
}

fn main() {
    let cli = Cli::parse();
    let reverse = cli.reverse;
    match &cli.command {
        Commands::CAP { sentence } => {
            cap::capitalize(sentence, reverse)
        }
        Commands::IMG { path } => {
            img::preview(path)
        }
    }
}
