use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "An interpreter / compiler for the BrainSponge programming language", long_about = None)]
pub struct Cli {
    /// The file to parse
    #[arg(short, long)]
    pub file: String,

    /// The starting size of the memory
    #[arg(long, default_value_t = 10000)]
    pub memory: usize,
}