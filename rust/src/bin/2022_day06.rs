use std::collections::HashSet;

fn is_unique<T>(marker: &[T]) -> bool
where
    T: std::hash::Hash + std::cmp::Eq
{
    let set: HashSet<&T> = HashSet::from_iter(marker.iter());
    if set.len() == marker.len() { return true }
    return false
}

fn main() {
    let input: Vec<char> = include_str!("../../../inputs/2022_day06.txt").chars().collect();
    let window_size = 14;

    for (i, marker) in input.windows(window_size).enumerate() {
        if is_unique(marker) {
            println!("{}", i + window_size);
            break;
        }
    }
}
