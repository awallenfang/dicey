use rand::Rng;

pub struct Dice {
    eyes: u16,
    count: u16,
    add: i32,
}

impl Dice {
    pub fn new(eyes: u16) -> Result<Dice, &'static str> {
        if eyes == 0 {
            return Err("Invalid Eye Count");
        }
        Ok(Dice {
            eyes,
            count: 1,
            add: 0,
        })
    }
    pub fn new_counted(eyes: u16, count: u16) -> Result<Dice, &'static str> {
        if eyes == 0 {
            return Err("Invalid Eye Count");
        }
        if count == 0 {
            return Err("Invalid Dice Count");
        }
        Ok(Dice {
            eyes,
            count,
            add: 0,
        })
    }
    pub fn new_added(eyes: u16, add: i32) -> Result<Dice, &'static str> {
        if eyes == 0 {
            return Err("Invalid Eye Count");
        }
        Ok(Dice {
            eyes,
            count: 1,
            add,
        })
    }
    pub fn new_subbed(eyes: u16, sub: i32) -> Result<Dice, &'static str> {
        if eyes == 0 {
            return Err("Invalid Eye Count");
        }
        Ok(Dice {
            eyes,
            count: 1,
            add: -sub,
        })
    }
    pub fn new_full(eyes: u16, count: u16, add: i32) -> Result<Dice, &'static str> {
        if eyes == 0 {
            return Err("Invalid Eye Count");
        }
        if count == 0 {
            return Err("Invalid Dice Count");
        }
        Ok(Dice { eyes, count, add })
    }
    pub fn roll(&self) -> i32 {
        let mut rng = rand::thread_rng();
        match (self.count * rng.gen_range(1..=self.eyes)) as i32 + self.add {
            num if num <= 0 => 1,
            num => num,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dice::Dice;
    #[test]
    fn basic_dice() {
        let d6 = Dice::new(6).unwrap();
        let num = d6.roll();
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
        let num = d6.roll();
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
        let num = d20_plus_69.roll();
        assert!((21..=89).contains(&num));
    }

    #[test]
    fn basic_subbed_dice() {
        let d20_minus_5 = Dice::new_subbed(20, 5).unwrap();
        let num = d20_minus_5.roll();
        assert!((1..=15).contains(&num));
    }

    #[test]
    fn negative_roll() {
        let d6_minus_10 = Dice::new_subbed(6, 10).unwrap();
        let num = d6_minus_10.roll();
        assert_eq!(&num, &1_i32);
    }
}
