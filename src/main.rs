use std::io::{self, Write};

use parser::parse_bs;

use crate::program::{execute, Instance};

mod parser;
mod utils;
mod program;

fn main() -> io::Result<()> {
    let mut buf = String::new();

    loop {
        buf.clear();
        print!("> ");
        io::stdout().flush()?;

        io::stdin().read_line(&mut buf)?;
        if buf.trim() == "exit".to_string() {
            break;
        }

        let code = parse_bs(&buf.trim()).unwrap();
        let mut inst = Instance::new();
        execute(&mut inst, &code);
    }

    Ok(())
}
