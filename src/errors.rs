use std::fmt;
#[derive(Debug, Clone)]
pub enum ParsingError {
    WrongFormat,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParsingError::WrongFormat => write!(f, "The dice format is not correct.")
        }
    }
}

#[derive(Debug, Clone)]
pub enum StructureError {
    ZeroEyes,
    ZeroCount
}

impl fmt::Display for StructureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StructureError::ZeroCount => write!(f, "Please roll at least one die."),
            StructureError::ZeroEyes => write!(f, "Please roll a die with at least one eye."),
        }
    }
}