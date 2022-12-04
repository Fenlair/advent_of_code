fn main() {
    let input = include_str!("../../../inputs/2021_day01.txt");

    let mut calories = input
        .split("\n\n")
        .map(|elf|
             elf.lines()
             .map(|item| item.parse::<u32>().unwrap())
             .sum::<u32>())
        .collect::<Vec<u32>>();
    calories.sort();
    println!("{:?}", calories.iter().rev().take(3).sum::<u32>());
}
