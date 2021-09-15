use dicey::Dice;

fn main() {
    println!("{}", "2d6 +4 +3d20".parse::<Dice>().unwrap());
    let d20 = "d20".parse::<Dice>().unwrap();
    println!("{} results in {}", d20, d20.roll());
}
