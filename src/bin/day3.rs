use advent_of_code_2016::day3::{num_possible_triangles, num_possible_vertical_triangles, INPUT};

fn main() {
    let num_possible = num_possible_triangles(INPUT);
    println!("part1: {num_possible}");

    let num_possible = num_possible_vertical_triangles(INPUT);
    println!("part2: {num_possible}");
}
