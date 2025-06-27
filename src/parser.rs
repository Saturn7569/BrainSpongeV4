use crate::utils::BSError;

#[derive(Debug)]
pub enum ParseTree {
    Add(u32),
    Sub(u32),

    ByteOut(u32),
    ByteIn(u32),

    Left(u32),
    Right(u32),

    Loop {
        contents: Vec<ParseTree>,
    },
    Check {
        contents: Vec<ParseTree>,
    },
}

pub fn parse_bs(txt:&str) -> Result<Vec<ParseTree>, BSError>{
    let (res, _) = get_code(txt, None)?;
    Ok(res)
}
fn get_code(txt:&str, end:Option<char>) -> Result<(Vec<ParseTree>, usize), BSError> {
    let mut idx:usize = 0;
    let mut res:Vec<ParseTree> = Vec::new();
    while idx < txt.len() {
        let current = txt.chars().nth(idx);
        if let Some(c) = current {
            //println!("get_code: {}", c);
            if let Some(end_c) = end {
                if c == end_c {
                    return Ok((res, idx)); // +1 to move past end char
                }
            }
            match c {
                '+' => {
                    let (tok, tok_i) = get_opers(&txt[idx..], '+')?;
                    res.push(tok);
                    idx += tok_i;
                },
                '-' => {
                    let (tok, tok_i) = get_opers(&txt[idx..], '-')?;
                    res.push(tok);
                    idx += tok_i;
                },
                '.' => {
                    let (tok, tok_i) = get_opers(&txt[idx..], '.')?;
                    res.push(tok);
                    idx += tok_i;
                },
                ',' => {
                    let (tok, tok_i) = get_opers(&txt[idx..], ',')?;
                    res.push(tok);
                    idx += tok_i;
                },
                '>' => {
                    let (tok, tok_i) = get_opers(&txt[idx..], '>')?;
                    res.push(tok);
                    idx += tok_i;
                },
                '<' => {
                    let (tok, tok_i) = get_opers(&txt[idx..], '<')?;
                    res.push(tok);
                    idx += tok_i;
                },
                '[' => {
                    let (tok, tok_i) = get_code(&txt[idx + 1..], Some(']'))?;
                    res.push(ParseTree::Loop { contents: tok });
                    idx += tok_i + 2; // 1 for '[', 1 for ']'
                },
                '{' => {
                    let (tok, tok_i) = get_code(&txt[idx + 1..], Some('}'))?;
                    res.push(ParseTree::Check { contents: tok });
                    idx += tok_i + 2; // 1 for '[', 1 for ']'
                },
                _ => {
                    idx += 1; // Skip unrecognized characters
                },
            }
        } else {
            return Err(BSError::Other(format!("Failed to get character")));
        }
    }

    if let Some(end_c) = end {
        return Err(BSError::Unclosed(format!("expected to close {}", end_c)));
    }
    Ok((res, idx))
}
fn get_opers(txt:&str, oper:char) -> Result<(ParseTree, usize), BSError> {
    let mut idx:usize = 0;
    let mut count:u32 = 0;
    while idx < txt.len() {
        let current = txt.chars().nth(idx);
        if let Some(c) = current {
            //println!("get_opers: {}", c);
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
        '>' => ParseTree::Right(count),
        '<' => ParseTree::Left(count),
        _ => panic!("Inavlid control flow in get_opers or oper {} not implemented", oper),
    }, idx))
}
