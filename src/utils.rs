use std::fmt;

#[derive(Debug)]
pub enum BSError {
    Unclosed(String),
    Syntax(String),
    Other(String),
}
impl fmt::Display for BSError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BSError::Unclosed(e) => write!(f, "Unclosed ({})", e),
            BSError::Syntax(e) => write!(f, "{}", e),
            BSError::Other(e) => write!(f, "{}", e),
        }
    }
}
