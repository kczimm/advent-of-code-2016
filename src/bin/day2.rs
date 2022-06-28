use advent_of_code_2016::day2::{Keypad, INPUT};

fn main() {
    let mut keypad = Keypad::new();
    let code = keypad.apply_directions(INPUT);

    println!("part1: {:?}", code);
}
