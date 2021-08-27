use rand::Rng;
use std::cmp::Ordering;
use std::fmt;

#[derive(PartialEq, Copy, Clone, Debug)]
pub(crate) struct Die {
    eyes: u16,
    count: u16,
    add: i32,
    neg: bool,
}

impl Die {
    pub(crate) fn new(eyes: u16) -> Die {
        Die {
            eyes,
            count: 1,
            add: 0,
            neg: false,
        }
    }
    pub(crate) fn new_counted(eyes: u16, count: u16) -> Die {
        Die {
            eyes,
            count,
            add: 0,
            neg: false,
        }
    }
    pub(crate) fn new_added(eyes: u16, add: i32) -> Die {
        Die {
            eyes,
            count: 1,
            add,
            neg: false,
        }
    }
    pub(crate) fn new_subbed(eyes: u16, sub: i32) -> Die {
        Die {
            eyes,
            count: 1,
            add: -sub,
            neg: false,
        }
    }
    pub(crate) fn new_full(eyes: u16, count: u16, add: i32) -> Die {
        Die {
            eyes,
            count,
            add,
            neg: false,
        }
    }
    pub(crate) fn new_internal(eyes: u16, count: u16, add: i32, neg: bool) -> Die {
        Die {
            eyes,
            count,
            add,
            neg,
        }
    }
    pub(crate) fn roll(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let res = match (self.count * rng.gen_range(1..=self.eyes)) as i32 + self.add {
            num if num <= 0 => 1,
            num => num,
        };
        if self.neg {
            -res
        } else {
            res
        }
    }
}

impl fmt::Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pre = match self.neg {
            true => "-",
            false => "+",
        };
        let base = format!("{}d{}", self.count, self.eyes);
        let added = match self.add.cmp(&0) {
            Ordering::Equal => String::from(""),
            Ordering::Greater => format!("+{}", self.add),
            Ordering::Less => format!("{}", self.add),
        };
        write! {f, "{}{}{}", pre, base, added}
    }
}

#[test]
fn basic_die() {
    let d6 = Die::new(6);
    let num = d6.roll();
    assert!((1..=6).contains(&num));
}

#[test]
fn basic_counted_die() {
    let d6 = Die::new_counted(6, 1);
    let num = d6.roll();
    assert!((1..=6).contains(&num));
}

#[test]
fn basic_added_die() {
    let d20_plus_69 = Die::new_added(20, 69);
    let num = d20_plus_69.roll();
    assert!((21..=89).contains(&num));
}

#[test]
fn basic_subbed_die() {
    let d20_minus_5 = Die::new_subbed(20, 5);
    let num = d20_minus_5.roll();
    assert!((-4..=15).contains(&num));
}

#[test]
fn display() {
    let d20 = Die::new(20);
    assert_eq!(d20.to_string(), "+1d20");

    let d20_plus_1 = Die::new_added(20, 1);
    assert_eq!(d20_plus_1.to_string(), "+1d20+1");

    let five_d20_plus_69 = Die::new_full(20, 5, 69);
    assert_eq!(five_d20_plus_69.to_string(), "+5d20+69");

    let five_d20_minus_69 = Die::new_full(20, 5, -69);
    assert_eq!(five_d20_minus_69.to_string(), "+5d20-69");
}
