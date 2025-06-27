use std::io::{self};

use parser::parse_bs;

use crate::program::{execute, Instance};

mod parser;
mod utils;
mod program;

fn main() -> io::Result<()> {
    let mut buf = String::new();

    loop {
        buf.clear();

        io::stdin().read_line(&mut buf)?;
        if buf.trim() == "exit".to_string() {
            break;
        }

        let code = parse_bs(&buf.trim());
        if code.is_err() {
            println!("{}", code.unwrap_err());
            continue;
        }

        //println!("{:#?}", code);
        let mut inst = Instance::new();
        match execute(&mut inst, &code.unwrap()) {
            Ok(_) => {},
            Err(e) => println!("{}", e),
        }
    }

    Ok(())
}
