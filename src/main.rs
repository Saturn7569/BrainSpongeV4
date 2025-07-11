use std::fs;

use clap::Parser;
use parser::parse_bs;

use crate::{cli::Cli, program::{execute, Instance}, utils::{generate_dump, BSError}};

mod parser;
mod utils;
mod program;
mod cli;

fn main() {
    match real_main() {
        Ok(_) => {},
        Err(e) => handle_bs_error(&e),
    }
}

fn real_main() -> Result<(), BSError> {
    let args = Cli::parse();

    let f = fs::read_to_string(&args.file).map_err(|e| {
        BSError::Other(format!("Failed to open file ({})", e))
    })?;

    let code = parse_bs(&f)?;
    let mut inst = Instance::new(args.memory);
    execute(&mut inst, &code)?;

    if args.dump {
        let dump = generate_dump(&inst, 16);
        fs::write(format!("{}.dump", &args.file), &dump).map_err(|e| {
            BSError::Other(format!("Failed to write file ({})", e))
        })?;

        println!("\nDump saved to {}.dump", &args.file);
    }

    Ok(())
}

fn handle_bs_error(e:&BSError) {
    println!("{}", e);
    std::process::exit(match e {
        BSError::Unclosed(_) => 2,
        BSError::Syntax(_) => 3,
        BSError::Other(_) => 1,
    });
}