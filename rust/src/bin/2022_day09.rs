use std::collections::HashSet;

fn step(h: (i32, i32), t: (i32, i32)) -> (i32, i32) {
    let horiz_sign = i32::signum(h.0 - t.0);
    let vert_sign  = i32::signum(h.1 - t.1);
    let horiz_dist = i32::abs(h.0 - t.0);
    let vert_dist  = i32::abs(h.1 - t.1);

    let movement = match (horiz_dist, vert_dist) {
        (0, 0) | (0, 1) | (1, 0) | (1, 1) => (0, 0),
        (0, 2)                            => (0, vert_sign),
        (2, 0)                            => (horiz_sign, 0),
        (2, 1) | (1, 2) | (2, 2)          => (horiz_sign, vert_sign),
        _ => panic!("{}, {}", horiz_dist, vert_dist),
    };
    return (t.0 + movement.0, t.1 + movement.1)
}

fn main() {
    let input = include_str!("../../../inputs/2022_day09.txt").lines()
                                                              .map(|l| l.split(" "))
                                                              .map(|mut s| (s.next().unwrap(), s.next().unwrap().parse::<u32>().unwrap()))
                                                              .collect::<Vec<(&str, u32)>>();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut positions = HashSet::new();
    positions.insert(tail);
    for (direction, amount) in input.iter() {
        for _ in 0..*amount {
            match *direction {
                "R" => head.0 = head.0 + 1,
                "L" => head.0 = head.0 - 1,
                "U" => head.1 = head.1 + 1,
                "D" => head.1 = head.1 - 1,
                _ => panic!(),
            }
            tail = step(head, tail);
            positions.insert(tail);
        }
    }
    println!("Part 1: {:?}", positions.len());

    let mut knots: Vec<(i32, i32)> = vec![(0, 0); 10];
    let mut positions = HashSet::new();
    positions.insert(knots[9]);
    for (direction, amount) in input.iter() {
        for _ in 0..*amount {
            match *direction {
                "R" => knots[0].0 = knots[0].0 + 1,
                "L" => knots[0].0 = knots[0].0 - 1,
                "U" => knots[0].1 = knots[0].1 + 1,
                "D" => knots[0].1 = knots[0].1 - 1,
                _ => panic!(),
            }
            let mut h = knots[0];
            for t in knots[1..].iter_mut() {
                *t = step(h, *t);
                h = *t;
            }
            positions.insert(knots[9]);
        }
    }
    println!("Part 2: {:?}", positions.len());
}
