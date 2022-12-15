use itertools::Itertools;

struct Monkey {
    items: Vec<u64>,
    operation: (char, Option<u64>),
    test: (u64, usize, usize),
    acc: u64,
}

impl Monkey {
    pub fn new(items: Vec<u64>, operation: (char, Option<u64>), test: (u64, usize, usize)) -> Monkey {
        Monkey {items, operation, test, acc: 0}
    }
}

fn parse_monkey(input: &str) -> Monkey {
    let lines = input.lines().skip(1).collect::<Vec<&str>>();
    let items = lines[0].strip_prefix("  Starting items: ").unwrap()
                        .split(", ")
                        .map(|n| n.parse::<u64>().unwrap())
                        .collect();

    let tmp = lines[1].strip_prefix("  Operation: new = old ").unwrap();
    let op = tmp.chars().next().unwrap();
    let n = tmp[2..].parse::<u64>().ok();
    let operation = (op, n);

    let test_div   = lines[2].strip_prefix("  Test: divisible by ").unwrap().parse::<u64>().unwrap();
    let test_true  = lines[3].strip_prefix("    If true: throw to monkey ").unwrap().parse::<usize>().unwrap();
    let test_false = lines[4].strip_prefix("    If false: throw to monkey ").unwrap().parse::<usize>().unwrap();
    Monkey::new(items, operation, (test_div, test_true, test_false))
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(parse_monkey).collect()
}

fn round(monkeys: &mut Vec<Monkey>, div: u64, modulo: u64) {
    for i in 0..monkeys.len() {
        for _ in 0..monkeys[i].items.len() {
            let item = monkeys[i].items.remove(0);  // O(n) but n is small
            monkeys[i].acc += 1;
            let n = if let Some(n) = monkeys[i].operation.1 {n} else {item};
            let worry = match monkeys[i].operation.0 {
                '+' => (item + n) / div % modulo,
                '*' => (item * n) / div % modulo,
                _ => panic!("Illegal operation!")
            };
            let (test_div, test_true, test_false) = monkeys[i].test;
            if worry % test_div == 0 {
                monkeys[test_true].items.push(worry);
            } else {
                monkeys[test_false].items.push(worry);
            }
        }
    }
}

// fn print_monkeys(monkeys: &Vec<Monkey>) {
//     println!("-------------------------------------------------------");
//     for (i, monkey) in monkeys.iter().enumerate() {
//         println!("Monkey {}: {:?} | acc = {}", i, monkey.items, monkey.acc);
//     }
// }

fn main() {
    let input = include_str!("../../../inputs/2022_day11.txt");

    let mut monkeys = parse_input(input);
    for _ in 0..20 {
        round(&mut monkeys, 3, u64::MAX);
    }
    let res: Vec<u64> = monkeys.into_iter().map(|m| m.acc).sorted().rev().take(2).collect();
    println!("Part 1: {} * {} = {}", res[0], res[1], res[0] * res[1]);


    let mut monkeys = parse_input(input);
    let modulo: u64 = monkeys.iter().map(|m| m.test.0).product();
    println!("{}", modulo);
    for _ in 0..10000 {
        round(&mut monkeys, 1, modulo);
    }
    let res: Vec<u64> = monkeys.into_iter().map(|m| m.acc).sorted().rev().take(2).collect();
    println!("Part 1: {} * {} = {}", res[0], res[1], res[0] * res[1]);
}
