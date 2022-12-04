fn main() {
    let input = include_str!("../../../inputs/2022_day04.txt");

    let mut count = 0;
    for pair in input.lines() {
        let mut tmp = pair.split(",");
        let elf1 = tmp.next().unwrap();
        let elf2 = tmp.next().unwrap();

        let mut tmp = elf1.split("-");
        let elf1_low = tmp.next().unwrap();
        let elf1_high = tmp.next().unwrap();
        let mut tmp = elf2.split("-");
        let elf2_low = tmp.next().unwrap();
        let elf2_high = tmp.next().unwrap();

        if elf1_low >= elf2_low && elf1_high <= elf2_high && elf1 != elf2 {
            count += 1;
        } else if elf2_low >= elf1_low && elf2_high <= elf1_high && elf1 != elf2 {
            count += 1;
        }
    }
    println!("{:?}", count)
}
