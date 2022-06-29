use advent_of_code_2016::day2::{BasicKeypad, CrazyKeypad, Keypad, INPUT};

fn main() {
    println!("part1: {:?}", BasicKeypad::apply_directions(INPUT));
    println!("part2: {:?}", CrazyKeypad::apply_directions(INPUT));
}
