#![allow(unused)]
use clap::{Parser, Subcommand};
use hex;
use magic_switcheroo::base::BaseChoice;
use magic_switcheroo::errors;
use magic_switcheroo::fs::read_file;
#[derive(Parser)]
#[command(author, version, about, long_about = None, disable_help_flag = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Encode {
        filename: String,
        #[arg(short, long)]
        magic: String,
    },
    Chr {
        #[arg(short, long)]
        hex: bool,
        #[arg(short, long)]
        bin: bool,
        #[arg(short, long)]
        oct: bool,
        #[arg(short, long)]
        dec: bool,
        parts: String,
    },
    Ord {
        #[arg(short, long)]
        hex: bool,
        #[arg(short, long)]
        bin: bool,
        #[arg(short, long)]
        oct: bool,
        #[arg(short, long)]
        dec: bool,
        parts: String,
    },
}

fn main() -> Result<(), errors::Error> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Encode { filename, magic }) => {
            println!("encode {} with magic {}", filename, magic);
            let data = read_file(filename)?;
            println!("{}", hex::encode(data));
        }
        Some(Commands::Chr {
            parts,
            hex,
            bin,
            oct,
            dec,
        }) => {
            let bc = BaseChoice {
                bin: *bin,
                dec: *dec,
                hex: *hex,
                oct: *oct,
            };
            println!("chr {}", bc);
        }
        Some(Commands::Ord {
            parts,
            hex,
            bin,
            oct,
            dec,
        }) => {
            let bc = BaseChoice {
                bin: *bin,
                dec: *dec,
                hex: *hex,
                oct: *oct,
            };
            println!("ord {}", bc);
        }
        None => {}
    }
    Ok(())
}
