# Dicer (name pending)

Dicer is a Rust crate designed specifically for Tabletop RPG dice rolls and parsing such dice notation strings.

It supports configurable dice notations and the stringing of dice in the form of `1d6 + 2d4`.

## Usage
```rust
use dicer::Dice;

fn main() {
    // Creates a dice notated as 1d20
    let d20:Dice = Dice::new(20).unwrap();
    println!("The result of the roll is: {}", d20.roll().result);

    // Parses the string into a dice notated as 1d10 + 1d4
    let d10_plus_d4: DiceSet = "d10+d4".parse::<Dice>().unwrap();
    println!("The result of the roll is: {}", d10_plus_d4.roll().result);
}
```

### TODO
- [] Errors for failed die
- [] Errors for failed parsing
- [] Exploding die
- [] Dropping die
- [] Parsing into DiceSet
- [x] num.cmp for +/- Display
- [] DiceSet for rolls like 1d10+2d6

### Feature Plans
- [] Configurable dice notation syntax