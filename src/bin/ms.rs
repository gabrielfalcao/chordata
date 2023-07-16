#![allow(unused)]
use std::io;
use std::io::Write;
use clap::{Parser, Subcommand};
use hex;
use chordata::base::{BaseChoice, parse_u32_from_string};
use chordata::errors;
use chordata::fs::read_file;
#[derive(Parser)]
#[command(author, version, about, long_about = None, disable_help_flag = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "resolves a number from any base to decimal base", long_about = "prints out the UTF-8 Char based on the given space-separated numbers")]
    Resolve {
        parts: Vec<String>,
    },
    #[command(about = "converts numbers to characters", long_about = "prints out the UTF-8 Char based on the given space-separated numbers")]
    Chr {
        parts: Vec<String>,
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
        parts: Vec<String>,
    },
}

fn main() -> Result<(), errors::Error> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Resolve {
            parts,
        }) => {
            let stdout = io::stdout();
            let mut buffer = stdout.lock();
            for part in parts {
                write!(buffer, "{} ", match parse_u32_from_string((&part).to_string()) {
                    Ok((_, num)) => num,
                    Err(e) => panic!("{}", e)
                });
            }
            write!(buffer, "\n");
        }
        Some(Commands::Chr {
            parts,
        }) => {
            let stdout = io::stdout();
            let mut buffer = stdout.lock();
            for part in parts {
                write!(buffer, "{}", match parse_u32_from_string((&part).to_string()) {
                    Ok((base, num)) => match base.to_choice().chr(num) {
                        Ok(c) => c,
                        Err(e) => panic!("{}", e)
                    },
                    Err(e) => panic!("{}", e)
                });
            }
            write!(buffer, "\n");
        }
        Some(Commands::Ord {
            parts,
            hex,
            bin,
            oct,
            dec,
        }) => {
            let stdout = io::stdout();
            let mut buffer = stdout.lock();
            let bc = BaseChoice {
                bin: *bin,
                dec: *dec,
                hex: *hex,
                oct: *oct,
            };
            let radix = match bc.to_radix() {
                Ok(o) => o,
                Err(e) => panic!("{}", e)
            };
            for part in parts {
                for c in part.chars() {
                    write!(buffer, "{}", match c.to_digit(radix) {
                        Some(ta) => ta,
                        None => panic!("cannot convert {} to number of base {}", c, radix)
                    });
                }
            }
            write!(buffer, "\n");
        }
        None => {}
    }
    Ok(())
}
