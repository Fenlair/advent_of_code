fn addx2num(s: &str) -> i32 {
    let mut sp = s.split(" ");
    sp.next();
    return sp.next().unwrap().parse::<i32>().unwrap()
}

fn parse_input(lines: Vec<&str>) -> Vec<Option<i32>> {
    let mut v = vec![];
    for line in lines {
        match line {
            "noop" => v.push(None),
            _ => {v.push(None); v.push(Some(addx2num(line)))},
        };
    }
    return v
}

fn create_trace(input: Vec<Option<i32>>) -> Vec<i32> {
    let mut v = vec![];
    v.push(1);
    for val in input {
        match val {
            None => v.push(*v.last().unwrap()),
            Some(n) => v.push(v.last().unwrap() + n),
        }
    }
    return v
}

fn line(a: &[i32]) -> String {
    let mut res: Vec<char> = vec![];
    for i in 0..40 {
        res.push(if i32::abs(a[i] - (i as i32)) <= 1 { '#' } else { '.' });
    }
    res.iter().collect::<String>()
}

fn main() {
    let input: Vec<&str> = include_str!("../../../inputs/2022_day10.txt").lines().collect();
    let input = parse_input(input);
    let trace = create_trace(input);
    let result = trace.iter()
                      .skip(19)
                      .step_by(40).copied()
                      .enumerate()
                      .map(|(i, v)| (20 + i as i32 * 40) * v)
                      .sum::<i32>();
    println!("Part 1: {}", result);

    println!("Part 2:");
    trace.chunks_exact(40).map(line).for_each(|l| println!("{}", l));
}
