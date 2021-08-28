use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ParsingError {
    WrongFormat,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParsingError::WrongFormat => write!(f, "The dice format is not correct."),
        }
    }
}

impl Error for ParsingError {}

#[derive(Debug, Clone)]
pub enum StructureError {
    ZeroEyes,
    ZeroCount,
    InvalidEyes,
    InvalidCount,
}

impl fmt::Display for StructureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StructureError::ZeroCount => write!(f, "One of the dice has a count of zero"),
            StructureError::ZeroEyes => write!(f, "One of the dice has zero eyes"),
            StructureError::InvalidCount => write!(f, "One of the dice has an invalid count"),
            StructureError::InvalidEyes => {
                write!(f, "One of the dice has an invalid amount of eyes")
            }
        }
    }
}

impl Error for StructureError {}
