use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "An interpreter / compiler for the BrainSponge programming language", long_about = None)]
struct Cli {
    /// The file to parse
    #[arg(short, long)]
    file: String,

    /// The starting size of the memory (default: 10 000 bytes)
    #[arg(long, default_value_t = 10000)]
    memory: usize,
}