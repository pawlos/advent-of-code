use std::fs;

fn main() {
    let content = fs::read_to_string("input-01.txt").unwrap();

    let calories_max : i32= content.split("\n\n").map(
            |elf| (elf.split("\n")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()))
    .map(|d| d.iter().sum()).into_iter().max().unwrap();
    /* Find the Elf carrying the most Calories. How many total Calories is that Elf carrying? */
    println!("Part1: {:?}", calories_max);
    /* Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total? */
    let mut elfs_calories : Vec<i32> = content.split("\n\n").map(
            |elf| (elf.split("\n")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()))
    .map(|d| d.iter().sum()).collect();

    elfs_calories.sort();
    elfs_calories.reverse();
    let top3 : i32 = elfs_calories.as_slice()[0..3].iter().sum();
    println!("Part2: {:?}", top3);
}
