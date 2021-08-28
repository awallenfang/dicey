use crate::dice::Dice;
use std::fmt;

#[derive(Debug)]
pub struct Roll {
    pub dice: Dice,
    pub dice_rolls: Vec<i32>,
    pub result: u16,
}

impl fmt::Display for Roll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // String together all the strings of the die
        let mut calc_string: String = self
            .dice_rolls
            .iter()
            .map(|n| match n.to_string() {
                s if &s[..1] != "-" => format!("+{}", s),
                s => s,
            })
            .collect();

        // Remove the first char through a very convoluted way O.o
        let calc_string = calc_string.as_mut_str();
        let calc_string = &mut calc_string[1..];

        let roll_string = format!("[{}]: {}", calc_string, self.result);

        write! {f, "{}", roll_string}
    }
}

#[cfg(test)]
mod test {
    use crate::dice::roll::Roll;
    use crate::dice::Dice;
    #[test]
    fn roll_printing() {
        let roll = Roll {
            dice: Dice::new(20).unwrap(),
            dice_rolls: vec![10],
            result: 10,
        };
        assert_eq!(roll.to_string(), "[10]: 10");

        let roll = Roll {
            dice: "d20+d20".parse::<Dice>().unwrap(),
            dice_rolls: vec![10, -10],
            result: 1,
        };
        assert_eq!(roll.to_string(), "[10-10]: 1");
    }
}
