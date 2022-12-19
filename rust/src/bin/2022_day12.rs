use std::collections::{HashSet, HashMap};

use itertools::Itertools;

const M: usize = 41; // 41;
const N: usize = 61; // 60;

type Map = [[Score; N]; M];
type Point = (usize, usize, Score);
type Score = u32;
type Path = Vec<Point>;


fn main() {
    let input: Vec<Vec<char>> = include_str!("../../../inputs/2022_day12.txt").lines()
                                                                              .map(|l| l.chars().collect())
                                                                              .collect();
    let char_map: [[char; N]; M] = to_array(input.iter().map(|l| to_array(l.to_vec())).collect_vec());
    let (start, goal) = get_start_stop(char_map);
    let map: Map = char_map.map(|row| row.map(|c| match c {'S' =>  0,
                                                           'E' => 26,
                                                           _ => (c as u32 - 97) as Score,}));

    // Part 1
    let path = a_star(start, goal, |n| 26-n.2, &map);
    println!("Part 1: {}", path.unwrap().len());

    // Part 2
    let starts: Vec<Point> = (0..M).cartesian_product(0..N)
                                   .filter(|(m, n)| map[*m][*n] == 0)
                                   .map(|(m, n)| (m, n, 0))
                                   .collect();

    // just brute force a_star on all possible starting points
    let best = starts.iter()
                     .flat_map(|start| a_star(*start, goal, |n| 26-n.2, &map))
                     .map(|p| p.len())
                     .min().unwrap();
    println!("Part 2: {}", best);
}

fn reconstruct_path(came_from: &HashMap<Point, Point>, current: &Point) -> Path {
    let mut total_path: Vec<Point> = vec![];
    let mut cur = current;
    while came_from.contains_key(cur) {
        total_path.push(*cur);
        cur = came_from.get(&cur).unwrap();
    }
    return total_path
}

fn get_valid_neighbors(point: &Point, map: &Map) -> Vec<Point> {
    let (m, n, h) = point.clone();
    let mut ret = vec![];
    if m > 0     && map[m-1][n] <= h+1 {ret.push((m-1, n,   map[m-1][n]))};
    if m < (M-1) && map[m+1][n] <= h+1 {ret.push((m+1, n,   map[m+1][n])) };
    if n > 0     && map[m][n-1] <= h+1 {ret.push((m,   n-1, map[m][n-1])) };
    if n < (N-1) && map[m][n+1] <= h+1 {ret.push((m,   n+1, map[m][n+1])) };
    return ret;
}

fn lowest_f_score(open_set: &HashSet<Point>, f_score: &HashMap<Point, Score>) -> Point {
    // open_set should be a heap or a priority queue for performance
    let (_, &current) = open_set.iter()
                                .map(|p| (f_score.get(p).unwrap(), p))
                                .sorted_by(|a, b| a.0.cmp(&b.0))
                                .next().unwrap();
    current
}

fn a_star<F>(start: Point, goal: Point, h: F, map: &Map) -> Option<Path>
where F: Fn(&Point) -> Score {
    let mut open_set:  HashSet<Point> = HashSet::new();
    let mut came_from: HashMap<Point, Point> = HashMap::new();
    let mut g_score:   HashMap<Point, Score> = HashMap::new();  // actual cost to the point
    let mut f_score:   HashMap<Point, Score> = HashMap::new();  // actual cost to the point + heuristic to end

    open_set.insert(start);
    g_score.insert(start, 0);
    f_score.insert(start, 0 + h(&start));

    while !open_set.is_empty() {
        let current = lowest_f_score(&open_set, &f_score);
        if current == goal {
            return Some(reconstruct_path(&came_from, &goal))
        }
        open_set.remove(&current);
        for neighbor in get_valid_neighbors(&current, map) {
            let tentative_g_score = g_score.get(&current).unwrap() + 1; // all weights are equal
            // only step to neighbor, if neighbor is still unknown or we improve the g_score
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&Score::MAX) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                f_score.insert(neighbor, tentative_g_score + h(&neighbor));
                if !open_set.contains(&neighbor) {
                    open_set.insert(neighbor);
                }
            }
        }
    }
    return None
}

fn get_start_stop(map: [[char; N]; M]) -> (Point, Point) {
    let mut start: Point = (255, 255, 255);
    let mut stop: Point = (255, 255, 255);
    for m in 0..M {
        for n in 0..N {
            if map[m][n] == 'S' {
                start = (m, n, 0);
            }
            if map[m][n] == 'E' {
                stop = (m, n, 26);
            }
        }
    }
    return (start, stop)
}

fn to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into().unwrap_or_else(|v: Vec<_>| panic!("Expected length {} but got {}", N, v.len()))
}
