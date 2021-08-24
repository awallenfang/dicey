mod dice;

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
}
