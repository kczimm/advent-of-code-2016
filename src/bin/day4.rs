use advent_of_code_2016::day4::{
    sector_id_of_northpole_object_store, sum_of_sector_ids_of_real_rooms, INPUT,
};

fn main() {
    let sum = sum_of_sector_ids_of_real_rooms(INPUT);
    println!("part1: {sum}");
    let sector_id = sector_id_of_northpole_object_store(INPUT);
    println!("part2: {sector_id}");
}
