use std::io::{self, Write};

use parser::parse_bs;

mod parser;
mod utils;

fn main() -> io::Result<()> {
    let mut buf = String::new();
    print!("> ");
    io::stdout().flush()?;

    io::stdin().read_line(&mut buf)?;
    
    println!("Tree:\n{:#?}", parse_bs(&buf.trim()).unwrap());

    Ok(())
}
