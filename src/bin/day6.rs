use advent_of_code_2016::day6::{MessageDecoder, Method, INPUT};

fn main() {
    let mut md = MessageDecoder::new();
    md.process(INPUT);
    println!("part1: {}", md.error_corrected_message(Method::MostLikely));
    println!("part2: {}", md.error_corrected_message(Method::LeastLikely));
}
