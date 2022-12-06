fn is_overlap_part1(elf1: &Vec<u32>, elf2: &Vec<u32>) -> u32 {
    if elf1[0] >= elf2[0] && elf1[1] <= elf2[1] {
        return 1
    } else
    if elf2[0] >= elf1[0] && elf2[1] <= elf1[1] {
        return 1
    }
    return 0
}
fn is_overlap_part2(elf1: &Vec<u32>, elf2: &Vec<u32>) -> u32 {
    if elf1[0] <= elf2[0] && elf1[1] >= elf2[0] {
        return 1
    } else
    if elf2[0] <= elf1[0] && elf2[1] >= elf1[0] {
        return 1
    }
    return 0
}

fn main() {
    let input = include_str!("../../../inputs/2022_day04.txt");

    let (mut count1, mut count2) = (0, 0);
    for pair in input.lines() {
        let mut elves: Vec<Vec<u32>> = vec![];
        for range in pair.split(",") {
            let elf = range.split("-").map(|number_str| number_str.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            elves.push(elf);
        }
        let (elf1, elf2) = (&elves[0], &elves[1]);

        count1 += is_overlap_part1(elf1, elf2);
        count2 += is_overlap_part2(elf1, elf2);
    }
    println!("{:?}", count1);
    println!("{:?}", count2);
}
