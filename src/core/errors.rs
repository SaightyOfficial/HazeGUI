#[derive(Debug, PartialEq)]
pub enum ColorError {
    InvalidLength(usize),
    InvalidHexCharacter(char),
}

#[derive(Debug, PartialEq)]
pub enum RectError {
    InvalidRectSize,
}
