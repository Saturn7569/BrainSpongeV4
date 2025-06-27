use std::io::{stdout, Write};

use crate::parser::ParseTree;

pub struct Instance {
    pub ptr: usize,
    pub mem: Vec<u32>,
}
impl Instance {
    pub fn new() -> Self {
        Instance { ptr: 0, mem: vec![0u32; 10000] }
    }
}

pub fn execute(instance: &mut Instance, code: &Vec<ParseTree>) {
    for instr in code.iter() {
        match instr {
            ParseTree::Add(i) => {
                instance.mem[instance.ptr] = (instance.mem[instance.ptr] + *i) % 256;
                println!("Cell {}: {}", instance.ptr, instance.mem[instance.ptr]);
            }
            ParseTree::Sub(i) => {
                instance.mem[instance.ptr] = if instance.mem[instance.ptr] >= *i {
                    instance.mem[instance.ptr] - *i
                } else {
                    255 - (*i - instance.mem[instance.ptr])
                };
                println!("Cell {}: {}", instance.ptr, instance.mem[instance.ptr]);
            }
            ParseTree::ByteOut(i) => {
                let c = instance.mem[instance.ptr] as u8 as char;
                for _ in 0..*i {
                    print!("{}", c);
                }
                stdout().flush().unwrap();
            }
            ParseTree::ByteIn(_i) => {
                // TODO: Make input work the way I want
            }
            ParseTree::Left(i) => {
                instance.ptr = if instance.ptr >= *i as usize {
                    instance.ptr - *i as usize
                } else {
                    instance.mem.len() - 1 - (*i as usize - instance.ptr)
                };
                println!("Moved left by {} (Cell {})", *i, instance.ptr);
            }
            ParseTree::Right(i) => {
                instance.ptr = (instance.ptr + *i as usize) % instance.mem.len();
                println!("Moved right by {} (Cell {})", *i, instance.ptr);
            }
            ParseTree::Loop { contents } => {
                while instance.mem[instance.ptr] != 0 {
                    execute(instance, contents);
                }
            }
        }
    }
}