use crate::utils::BSError;

#[derive(Debug)]
pub enum ParseTree {
    Add(u32),
    Sub(u32),

    ByteOut(u32),
    ByteIn(u32),

    Loop {
        contents: Vec<ParseTree>,
    },
}

pub fn parse_bs(txt:&str) -> Result<Vec<ParseTree>, BSError>{
    get_code(txt, None)
}
fn get_code(txt:&str, end:Option<char>) -> Result<Vec<ParseTree>, BSError> {
    let mut idx:usize = 0;
    let mut res:Vec<ParseTree> = Vec::new();
    loop {
        if idx >= txt.len() {
            break;
        }
        let current = txt.chars().nth(idx);
        if let Some(c) = current {
            if let Some(end_c) = end {
                if c == end_c {
                    return Ok(res);
                }
            }
            match c {
                '+' => {
                    let (tok, tok_i) = get_opers(&txt[idx..], '+')?;
                    res.push(tok);
                    idx = tok_i;
                },
                '-' => {
                    let (tok, tok_i) = get_opers(&txt[idx..], '-')?;
                    res.push(tok);
                    idx = tok_i;
                },
                '.' => {
                    let (tok, tok_i) = get_opers(&txt[idx..], '.')?;
                    res.push(tok);
                    idx = tok_i;
                },
                ',' => {
                    let (tok, tok_i) = get_opers(&txt[idx..], ',')?;
                    res.push(tok);
                    idx = tok_i;
                },
                '[' => todo!(),
                _ => {},
            }
        } else {
            return Err(BSError::Other(format!("Failed to get character")));
        }

        idx += 1;
    }
    Ok(res)
}
fn get_opers(txt:&str, oper:char) -> Result<(ParseTree, usize), BSError> {
    let mut idx:usize = 0;
    let mut count:u32 = 0;
    loop {
        if idx >= txt.len() {
            break;
        }
        let current = txt.chars().nth(idx);
        if let Some(c) = current {
            if c != oper {
                break;
            }
            count += 1;
        } else {
            return Err(BSError::Other(format!("Failed to get character")));
        }
        idx += 1;
    }
    Ok((match oper {
        '+' => ParseTree::Add(count),
        '-' => ParseTree::Sub(count),
        '.' => ParseTree::ByteOut(count),
        ',' => ParseTree::ByteIn(count),
        _ => panic!("Inavlid control flow in get_opers or oper {} not implemented", oper),
    }, idx))
}
