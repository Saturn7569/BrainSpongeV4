use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = "0.3.0", about = "An interpreter / compiler for the BrainSponge programming language", long_about = None)]
pub struct Cli {
    /// The file to parse
    #[arg(short, long)]
    pub file: String,

    /// The starting size of the memory
    #[arg(long, default_value_t = 10000)]
    pub memory: usize,

    /// Generates a memory dump at the end of the program (saves to <filename>.txt)
    #[arg(long, default_value_t = false)]
    pub dump: bool,
}
