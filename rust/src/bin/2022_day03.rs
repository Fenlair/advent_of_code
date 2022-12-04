use itertools::Itertools;

fn find_common(compartement1: &[char], compartement2: &[char]) -> Vec<char> {
    let mut vec: Vec<char> = vec![];
    for item in compartement1 {
        for compare in compartement2 {
            if item == compare {
                vec.push(*item);
            }
        }
    }
    return vec
}

fn priority(item: char) -> u32 {
    if item.is_ascii_lowercase() {
        return (item as u8 - b'a' + 1) as u32;
    }
    return (item as u8 - b'A' + 27) as u32;
}

fn main() {
    let input = include_str!("../../../inputs/2022_day03.txt");

    let mut list: Vec<u32> = vec![];
    for rucksack in input.lines() {
        let vec: Vec<char> = rucksack.chars().collect();
        let compartement1 = &vec[0..vec.len()/2];
        let compartement2 = &vec[vec.len()/2..vec.len()];
        list.push(priority(find_common(compartement1, compartement2)[0]))
    }
    println!("{:?}", list.iter().sum::<u32>());

    let mut list: Vec<u32> = vec![];
    for mut group in &input.lines().chunks(3) {
        let elf1: Vec<char> = group.next().unwrap().chars().collect();
        let elf2: Vec<char> = group.next().unwrap().chars().collect();
        let elf3: Vec<char> = group.next().unwrap().chars().collect();
        let tmp = find_common(&elf1, &elf2);
        list.push(priority(find_common(&elf3, &tmp)[0]));
    }
    println!("{:?}", list.iter().sum::<u32>());
}
