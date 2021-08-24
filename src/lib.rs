mod dice;

#[cfg(test)]
mod tests {
    use crate::dice::Dice;
    #[test]
    fn basic_dice() {
        let d6 = Dice::new(6).unwrap();
        let num = d6.roll();
        assert!(num <= 6 && num >= 1);
    }

    #[test]
    fn invalid_eyes() {
        let d = Dice::new(0);
        assert!(d.is_err());
    }
}
