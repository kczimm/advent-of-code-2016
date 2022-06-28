use advent_of_code_2016::day1::{Simple, TwiceFinder, Walker, INPUT};

fn main() {
    let blocks_away = Walker::<Simple>::new().walk(INPUT);
    println!("part1: {blocks_away}");

    let blocks_away = Walker::<TwiceFinder>::new().walk(INPUT);
    println!("part2: {blocks_away}");
}
