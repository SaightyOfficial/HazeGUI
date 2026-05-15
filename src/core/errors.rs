#[derive(Debug, PartialEq)]
pub enum ColorError {
    InvalidLength(usize),
    InvalidHexCharacter(char),
}