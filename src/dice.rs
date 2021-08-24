use rand::Rng;

pub struct Dice {
    eyes: i32,
    count: i32,
}

impl Dice {
    pub fn new(eyes: i32) -> Result<Dice, &'static str> {
        if eyes > 0 {
            Ok(Dice {
                eyes: eyes,
                count: 1,
            })
        } else {
            Err("Invalid Eye Count")
        }
    }
    pub fn new_counted(eyes: i32, count: i32) -> Result<Dice, &'static str> {
        if eyes == 0 {
            return Err("Invalid Eye Count");
        }
        if count == 0 {
            return Err("Invalid Dice Count");
        }
        Ok(Dice { eyes, count })
    }
    pub fn roll(&self) -> i32 {
        let mut rng = rand::thread_rng();
        self.count * rng.gen_range(1..=self.eyes)
    }
}
