use dicer::Dice;

fn main() {
    "2d6+4+3d20".parse::<Dice>();
    "d20".parse::<Dice>();
}
