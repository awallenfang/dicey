# Dicer (name pending)

Dicer is a Rust crate designed specifically for Tabletop RPG dice rolls and parsing such dice notation strings.

It supports configurable dice notations and the stringing of dice in the form of `1d6 + 2d4`.

## Usage
```rust
use dicer::Dice;

fn main() {
    // Creates a dice notated as 1d20
    let d20: Dice = Dice::new(20).unwrap();
    println!("The result of the roll is: {}", d20.roll());

    // Parses the string into a dice notated as 1d10 + 1d4
    let d10_plus_d4: Dice = "d10+d4".parse::<Dice>().unwrap();
    println!("The result of the roll is: {}", d10_plus_d4.roll());
}
```

### TODO
- [x] Errors for failed die
- [x] Errors for failed parsing
- [ ] (Exploding die)
- [ ] (Dropping die)
- [x] Parsing single die into Dice
- [x] Parsing multiple dice into Dice
- [ ] Refactor dice slicing
- [ ] Find die strings directly instead of searching for d-s
- [ ] Have roll() return a Roll struct to access the individual dice for output
- [x] Ignore/Remove whitespace when parsing
 
### Feature Plans
- [ ] Configurable dice notation syntax
- [ ] Support for math outside of the die