use itertools::izip;

fn main() {
    let mut input = include_str!("../../../inputs/2022_day06.txt").to_string();
    input.pop();

    let a = input.chars();
    let b = a.clone().skip(1);
    let c = b.clone().skip(1);
    let d = c.clone().skip(1);
    for marker in izip!(a, b, c, d) {
        println!("{:?}", marker)
    }
}
