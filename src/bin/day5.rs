use advent_of_code_2016::day5::PasswordCalculator;

fn main() {
    let password = PasswordCalculator::from("ojvtpuvg").calculate();
    println!("part1: {password}");
}
