use std::io::{stdout, Write};

use crate::parser::ParseTree;

pub struct Instance {
    pub ptr: usize,
    pub mem: Vec<u8>,
}
impl Instance {
    pub fn new() -> Self {
        Instance { ptr: 0, mem: vec![0u8; 10000] }
    }
}

pub fn execute(instance: &mut Instance, code: &Vec<ParseTree>) {
    for instr in code.iter() {
        match instr {
            ParseTree::Add(i) => {
                // Wrap around when adding to the cell
                instance.mem[instance.ptr] =
                    instance.mem[instance.ptr].wrapping_add(*i as u8);
            }
            ParseTree::Sub(i) => {
                // Wrap around when subtracting from the cell
                instance.mem[instance.ptr] =
                    instance.mem[instance.ptr].wrapping_sub(*i as u8);
            }
            ParseTree::ByteOut(i) => {
                let c = instance.mem[instance.ptr] as char;
                for _ in 0..*i {
                    print!("{}", c);
                }
                stdout().flush().unwrap();
            }
            ParseTree::ByteIn(_i) => {
                // implement later
            }
            ParseTree::Left(i) => {
                instance.ptr = instance.ptr.wrapping_sub(*i as usize);
            }
            ParseTree::Right(i) => {
                instance.ptr = (instance.ptr + *i as usize) % instance.mem.len();
            }
            ParseTree::Loop { contents } => {
                while instance.mem[instance.ptr] != 0 {
                    execute(instance, contents);
                }
            }
        }
    }
}