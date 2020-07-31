
#[derive(Debug)]
pub enum ParseError {
    EndOfFile(String),
    ErrorAt(usize, String),
}
