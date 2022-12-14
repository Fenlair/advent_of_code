// use itertools::Itertools;

// fn parse_stacks(stacks_str: &str) -> [Vec<char>; 9] {
//     let mut stacks: [Vec<char>; 9] = core::array::from_fn(|_| Vec::new());
//     stacks_str.lines().rev().skip(1).for_each(|line| {
//         let line_vec: Vec<char> = line.chars().collect();
//         for i in 0..9 {
//             let position: [char; 3] = line_vec[i*4..i*4+3].try_into().unwrap();
//             if position != [' ', ' ', ' '] {
//                 stacks[i].push(position[1]);
//             }
//         }
//     });
//     return stacks
// }

fn parse_stacks(stacks_str: &str) -> [Vec<char>; 9] {
    let mut stacks: [Vec<char>; 9] = core::array::from_fn(|_| Vec::new());
    stacks_str.lines().rev().skip(1)
        .map(|line| line.chars())
        .array_chunks::<4>()
        .map(|chunks| chunks.map(|chars| chars.next().unwrap()))
        .filter(|chunk| chunk.clone() != [' ', ' ', ' ', ' '])
        .enumerate()
        .map(|(i, chunk)| (i, chunk[1]))
        .for_each(|(i, chunk)| stacks[i].push(chunk));
    return stacks;
}


fn parse_proc(proc_str: &str) -> Vec<(usize, usize, usize)> {
    let mut proc: Vec<_> = vec![];
    for line in proc_str.lines() {
        let tokens = line.split(" ")
                         .enumerate()
                         .filter(|(i, _)| i % 2 != 0)
                         .map(|(_, num_str)| num_str.parse::<usize>().unwrap())
                         .collect::<Vec<_>>();
        proc.push((tokens[0], tokens[1]-1, tokens[2]-1));
    }
    return proc;
}

fn mutate_stacks1(stacks: &mut [Vec<char>; 9], proc: &Vec<(usize, usize, usize)>) {
    for &(amount, from, to) in proc.iter() {
        for _ in 0..amount {
            if let Some(next_crate) = stacks[from].pop(){
                stacks[to].push(next_crate);
            }
        }
    }
}

fn mutate_stacks2(stacks: &mut [Vec<char>; 9], proc: &Vec<(usize, usize, usize)>) {
    for &(amount, from, to) in proc.iter() {
        let from_len = stacks[from].len();
        let mut to_move: Vec<char> = stacks[from].drain(from_len-amount..).collect();
        stacks[to].append(&mut to_move);
    }
}

fn top_crates(stacks: &[Vec<char>; 9]) -> Vec<&char> {
    stacks.iter()
        .map(|stack| stack.last().unwrap())
        .collect()
}

fn main() {
    let mut input = include_str!("../../../inputs/2022_day05.txt").split("\n\n");
    let (stacks_str, proc_str) = (input.next().unwrap(), input.next().unwrap());
    println!("{:?}", stacks_str);

    let mut stacks = parse_stacks(stacks_str);
    let proc = parse_proc(proc_str);
    println!("{:?}", stacks);
    println!("{:?}", proc);

    mutate_stacks1(&mut stacks, &proc);
    let solution1 = top_crates(&stacks);
    println!("{:?}", solution1);

    let mut stacks = parse_stacks(stacks_str);
    mutate_stacks2(&mut stacks, &proc);
    let solution2 = top_crates(&stacks);
    println!("{:?}", solution2);
}
