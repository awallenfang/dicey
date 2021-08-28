mod die;
pub mod roll;

use crate::errors::{ParsingError, StructureError};
use die::Die;
use lazy_static::lazy_static;
use regex::Regex;
use roll::Roll;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

lazy_static! {
    //TODO: Find a regex way to find the dice strings, instead of just looking for ds
    static ref FIND_DICE: Regex = Regex::new(r"(?ix)\d*d").expect("Failed parsing FIND_DIE");
    static ref DICE_CONTENT: Regex = Regex::new(r"(?ix)(?P<pre>[+-])?
    (?P<count>\d+)?d
    (?P<eyes>[\d%]+)
    (?P<explode>!\d*)?
    (?P<add>[+-]\d*)?").expect("Failed parsing DICE_CONTENT");
}

#[derive(PartialEq, Debug, Clone)]
pub struct Dice {
    dice: Vec<Die>,
}

impl Dice {
    pub fn new(eyes: u16) -> Result<Dice, StructureError> {
        if eyes == 0 {
            return Err(StructureError::ZeroEyes);
        }
        Ok(Dice {
            dice: vec![Die::new(eyes)],
        })
    }
    pub fn new_counted(eyes: u16, count: u16) -> Result<Dice, StructureError> {
        if eyes == 0 {
            return Err(StructureError::ZeroEyes);
        }
        if count == 0 {
            return Err(StructureError::ZeroCount);
        }
        Ok(Dice {
            dice: vec![Die::new_counted(eyes, count)],
        })
    }
    pub fn new_added(eyes: u16, add: i32) -> Result<Dice, StructureError> {
        if eyes == 0 {
            return Err(StructureError::ZeroEyes);
        }
        Ok(Dice {
            dice: vec![Die::new_added(eyes, add)],
        })
    }
    pub fn new_subbed(eyes: u16, sub: i32) -> Result<Dice, StructureError> {
        if eyes == 0 {
            return Err(StructureError::ZeroEyes);
        }
        Ok(Dice {
            dice: vec![Die::new_subbed(eyes, sub)],
        })
    }
    pub fn new_full(eyes: u16, count: u16, add: i32) -> Result<Dice, StructureError> {
        if eyes == 0 {
            return Err(StructureError::ZeroEyes);
        }
        if count == 0 {
            return Err(StructureError::ZeroCount);
        }
        Ok(Dice {
            dice: vec![Die::new_full(eyes, count, add)],
        })
    }
    pub fn roll(&self) -> Roll {
        // Iterate over all the die, roll them and take their value.
        // Take the sum of all the values
        let rolls: Vec<i32> = self.dice.iter().map(|d| d.roll()).collect();
        let result = match rolls.iter().sum() {
            num if num <= 0 => 1,
            num => num,
        } as u16;
        Roll {
            dice: self.clone(),
            dice_rolls: rolls,
            result,
        }
    }
}

impl fmt::Display for Dice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // String together all the strings of the die
        let dice_string = self
            .dice
            .iter()
            .map(|d| d.to_string())
            .fold(String::new(), |d1, d2| d1 + &d2);

        // Skip first char, as that is a leading + we do not want
        let mut dice_string = dice_string.chars();
        dice_string.next();
        let dice_string = dice_string.as_str();

        write! {f, "{}", dice_string}
    }
}

impl FromStr for Dice {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Dice, Self::Err> {
        // Remove whitespace and copy the string
        let removed_whitespace = s.replace(" ", "");
        let mut unsliced_string = removed_whitespace.as_str();

        //TODO: Definitely refactor
        let mut dice_strings: Vec<&str> = vec![];

        // The offset is needed, since the position to split at changes when we remove chunks.
        // So this is the total length of removed chunks
        let mut offset = 0;
        for (i, position) in FIND_DICE.find_iter(unsliced_string).enumerate() {
            if i == 0 {
                continue;
            }
            let (chunk, rest) = unsliced_string.split_at(position.start() - 1 - offset);
            dice_strings.push(chunk);

            offset += chunk.len();
            unsliced_string = rest;
        }
        dice_strings.push(unsliced_string);
        println!("{:?}", dice_strings);

        let mut dice: Vec<Die> = vec![];

        for die in dice_strings {
            //Throws the WrongFormat Error if no capture is found
            let cap = match DICE_CONTENT.captures(die) {
                Some(c) => c,
                None => return Err(ParsingError::WrongFormat.into()),
            };

            //If it is not found, default to false
            let neg = match cap.name("pre") {
                Some(c) => match c.as_str() {
                    "+" => false,
                    "-" => true,
                    _ => false,
                },
                None => false,
            };

            // If it is not found or if the parsing fails, default to 1.
            let count = match cap.name("count") {
                Some(c) => c.as_str().parse::<u16>().unwrap_or(1_u16),
                None => 1_u16,
            };

            // If it is not found or if the parsing fails, default to 6.
            let eyes = match cap.name("eyes") {
                Some(c) => match c.as_str() {
                    "%" => 100_u16,
                    c => c.parse::<u16>().unwrap_or(6_u16),
                },
                None => 6_u16,
            };

            // If it is not found or if the parsing fails, default to 0.
            let add = match cap.name("add") {
                Some(c) => c.as_str().parse::<i32>().unwrap_or(0),
                None => 0,
            };

            if count == 0 {
                return Err(StructureError::ZeroCount.into());
            }
            if eyes == 0 {
                return Err(StructureError::ZeroEyes.into());
            }

            dice.push(Die::new_internal(eyes, count, add, neg));
        }

        Ok(Dice { dice })
    }
}

#[test]
fn basic_dice() {
    let d6 = Dice::new(6).unwrap();
    let num = d6.roll().result;
    assert!((1..=6).contains(&num));
}

#[test]
fn invalid_eyes() {
    let d = Dice::new(0);
    assert!(d.is_err());
}

#[test]
fn basic_counted_dice() {
    let d6 = Dice::new_counted(6, 1).unwrap();
    let num = d6.roll().result;
    assert!((1..=6).contains(&num));
}

#[test]
fn invalid_count() {
    let d = Dice::new_counted(20, 0);
    assert!(d.is_err());
}

#[test]
fn basic_added_dice() {
    let d20_plus_69 = Dice::new_added(20, 69).unwrap();
    let num = d20_plus_69.roll().result;
    assert!((21..=89).contains(&num));
}

#[test]
fn basic_subbed_dice() {
    let d20_minus_5 = Dice::new_subbed(20, 5).unwrap();
    let num = d20_minus_5.roll().result;
    assert!((1..=15).contains(&num));
}

#[test]
fn negative_roll() {
    let d6_minus_10 = Dice::new_subbed(6, 10).unwrap();
    let num = d6_minus_10.roll().result;
    assert_eq!(&num, &1_u16);
}

#[test]
fn display() {
    let d20 = Dice::new(20).unwrap();
    assert_eq!(d20.to_string(), "1d20");

    let d20_plus_1 = Dice::new_added(20, 1).unwrap();
    assert_eq!(d20_plus_1.to_string(), "1d20+1");

    let five_d20_plus_69 = Dice::new_full(20, 5, 69).unwrap();
    assert_eq!(five_d20_plus_69.to_string(), "5d20+69");

    let five_d20_minus_69 = Dice::new_full(20, 5, -69).unwrap();
    assert_eq!(five_d20_minus_69.to_string(), "5d20-69");
}

#[test]
fn regex_find_dice() {
    let caps = FIND_DICE.find_iter("1d10+d69+420d69+1337");
    let mut d1_start = 0;
    let mut d2_start = 0;
    let mut d3_start = 0;

    for (i, find) in caps.enumerate() {
        match i {
            0 => d1_start = find.start(),
            1 => d2_start = find.start(),
            2 => d3_start = find.start(),
            _ => panic!("Too many finds"),
        }
    }

    assert_eq!(d1_start, 0);
    assert_eq!(d2_start, 5);
    assert_eq!(d3_start, 9);
}

#[test]
fn regex_dice_content() {
    let caps1 = DICE_CONTENT.captures("1d10").unwrap();
    assert_eq!(caps1.name("count").unwrap().as_str(), "1");
    assert_eq!(caps1.name("eyes").unwrap().as_str(), "10");
    assert!(caps1.name("add").is_none());
    assert!(caps1.name("pre").is_none());

    let caps2 = DICE_CONTENT.captures("420d69+1337").unwrap();
    assert_eq!(caps2.name("count").unwrap().as_str(), "420");
    assert_eq!(caps2.name("eyes").unwrap().as_str(), "69");
    assert_eq!(caps2.name("add").unwrap().as_str(), "+1337");
    assert!(caps2.name("pre").is_none());

    let caps3 = DICE_CONTENT.captures("-420d69-1337").unwrap();
    assert_eq!(caps3.name("count").unwrap().as_str(), "420");
    assert_eq!(caps3.name("eyes").unwrap().as_str(), "69");
    assert_eq!(caps3.name("add").unwrap().as_str(), "-1337");
    assert_eq!(caps3.name("pre").unwrap().as_str(), "-");

    let caps4 = DICE_CONTENT.captures("d%").unwrap();
    assert!(caps4.name("count").is_none());
    assert_eq!(caps4.name("eyes").unwrap().as_str(), "%");
    assert!(caps4.name("add").is_none());
    assert!(caps4.name("pre").is_none());
}

#[test]
fn dice_parsing() {
    let d20 = "1d20".parse::<Dice>().unwrap();
    assert_eq!(d20, Dice::new(20).unwrap());

    let d100 = "d%".parse::<Dice>().unwrap();
    assert_eq!(d100, Dice::new(100).unwrap());

    let dd100 = "dd100".parse::<Dice>();
    assert!(dd100.is_err());

    let six_d20_minus_1d4_parse = "6d20 - 1d4".parse::<Dice>().unwrap();
    let six_d20_minus_1d4_built = Dice {
        dice: vec![Die::new_counted(20, 6), Die::new_internal(4, 1, 0, true)],
    };
    assert_eq!(six_d20_minus_1d4_built, six_d20_minus_1d4_parse);

    let d0 = "1d0".parse::<Dice>();
    assert!(d0.is_err());

    let zero_d60 = "0d60".parse::<Dice>();
    assert!(zero_d60.is_err());
}
