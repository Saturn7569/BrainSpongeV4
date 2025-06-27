use std::io::{self, stdout, Read, Write};

use crate::{parser::ParseTree, utils::BSError};

pub struct Instance {
    pub ptr: usize,
    pub mem: Vec<u32>,
}
impl Instance {
    pub fn new(mem_size:usize) -> Self {
        Instance { ptr: 0, mem: vec![0u32; mem_size] }
    }
}

pub fn execute(instance: &mut Instance, code: &Vec<ParseTree>) -> Result<(), BSError>{
    for instr in code.iter() {
        match instr {
            ParseTree::Add(i) => {
                instance.mem[instance.ptr] = (instance.mem[instance.ptr] + *i) % 256;
                //println!("Cell {}: {}", instance.ptr, instance.mem[instance.ptr]);
            }
            ParseTree::Sub(i) => {
                instance.mem[instance.ptr] = if instance.mem[instance.ptr] >= *i {
                    instance.mem[instance.ptr] - *i
                } else {
                    255 - (*i - instance.mem[instance.ptr])
                };
                //println!("Cell {}: {}", instance.ptr, instance.mem[instance.ptr]);
            }
            ParseTree::ByteOut(i) => {
                let c = instance.mem[instance.ptr] as u8 as char;
                for _ in 0..*i {
                    print!("{}", c);
                }
                stdout().flush().unwrap();
            }
            ParseTree::NumOut(i) => {
                let c = instance.mem[instance.ptr] as u8;
                for _ in 0..*i {
                    print!("{} ", c);
                }
                stdout().flush().unwrap();
            }
            ParseTree::ByteIn(i) => {
                let mut buf = [0u8; 1];
                for _ in 0..*i {
                    io::stdin().read_exact(&mut buf).map_err(|e| BSError::Other(format!("Failed to read ({})", e)))?;
                }
                instance.mem[instance.ptr] = buf[0] as u32;
            }
            ParseTree::Left(i) => {
                instance.ptr = if instance.ptr >= *i as usize {
                    instance.ptr - *i as usize
                } else {
                    instance.mem.len() - 1 - (*i as usize - instance.ptr)
                };
                //println!("Moved left by {} (Cell {})", *i, instance.ptr);
            }
            ParseTree::Right(i) => {
                instance.ptr = (instance.ptr + *i as usize) % instance.mem.len();
                //println!("Moved right by {} (Cell {})", *i, instance.ptr);
            }
            ParseTree::Loop { contents } => {
                while instance.mem[instance.ptr] != 0 {
                    execute(instance, contents)?;
                }
            }
            ParseTree::Check { contents } => {
                if instance.mem[instance.ptr] == 0 {
                    execute(instance, contents)?;
                }
            },
        }
    }
    Ok(())
}