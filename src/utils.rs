use std::fmt;

use crate::program::Instance;

#[derive(Debug)]
pub enum BSError {
    Unclosed(String),
    Syntax(String),
    Other(String),
}
impl fmt::Display for BSError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BSError::Unclosed(e) => write!(f, "ERROR: Unclosed ({})", e),
            BSError::Syntax(e) => write!(f, "SYNTAX ERROR: {}", e),
            BSError::Other(e) => write!(f, "ERROR: {}", e),
        }
    }
}

pub fn generate_dump(inst:&Instance, line_size:usize) -> String {
    let mut lines:usize = 0;
    let mut res = String::new();
    let mut data: Vec<u8> = Vec::new();

    let mut l = true;

    while l {
        data.clear();

        for i in 0..line_size {
            let idx = lines * line_size + i;

            if idx >= inst.mem.len() {
                l = false;
                break;
            }

            let val = inst.mem[idx] as u8;
            data.push(val);
        }
        if !l {
            break;
        }

        for val in data.iter() {
            res.push_str(format!("{:02X} ", val).as_str());
        }

        res.push_str("\t\t");

        for val in data.iter() {
            res.push(if (*val as char).is_control() {'.'} else {*val as char});
        }

        res.push('\n');

        lines += 1;
    }

    res
}