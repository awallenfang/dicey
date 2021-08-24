use rand::Rng;

pub struct Dice {
    eyes: i32,
}

impl Dice {
    pub fn new(eyes: i32) -> Result<Dice, &'static str>{
        if eyes > 0 {
            Ok(Dice{eyes})
        } else {
            Err("Invalid Eye Count")
        }
        
    }
    pub fn roll(&self) -> i32 {
        
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=self.eyes)
    }
}