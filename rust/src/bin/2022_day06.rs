use std::collections::HashSet;

fn is_marker_unique(marker: &[char]) -> bool {
    let set: HashSet<&char> = HashSet::from_iter(marker.iter());
    if set.len() == marker.len() {return true}
    return false
}

fn main() {
    let input: Vec<char> = include_str!("../../../inputs/2022_day06.txt").chars().collect();
    let window_size = 14;

    for (i, marker) in input.windows(window_size).enumerate() {
        if is_marker_unique(marker) {
            println!("{}", i + window_size);
            break;
        }
    }
}
