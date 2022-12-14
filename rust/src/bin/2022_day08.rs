use std::cmp;

use itertools::Itertools;

fn str2nums(line: &str) -> Vec<u32> {
    return line.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn transpose<T: Copy>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let row_len = v[0].len();
    (0..row_len).into_iter()
        .map(|i| v.iter().map(|row| row[i]).collect())
        .collect()
}

fn reverse_copy<T: Copy>(v: &Vec<T>) -> Vec<T> {
    v.iter().map(|e| e.clone()).rev().collect()
}

fn scan_left_right(row: &Vec<u32>) -> Vec<u32> {
    let mut ret: Vec<u32> = vec![0; row.len()];
    let mut max_height: i64 = -1;
    for (i, tree) in row.iter().enumerate() {
        if *tree as i64 > max_height {
            max_height = *tree as i64;
            ret[i] = 1
        }
    }
    return ret
}

fn scan_both(row: &Vec<u32>) -> Vec<u32> {
    let rev_row: Vec<u32> = reverse_copy(row);
    let left_right = scan_left_right(row);
    let right_left = reverse_copy(&scan_left_right(&rev_row));
    left_right.into_iter().zip(right_left).map(|(l, r)| cmp::max(l, r)).collect()
}

fn mark_field(v: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    v.iter().map(scan_both).collect()
}

fn compare_matrices<F>(m1: &Vec<Vec<u32>>, m2: &Vec<Vec<u32>>, f: F) -> Vec<Vec<u32>>
    where F: Fn(u32, u32) -> u32 {
    m1.iter()
      .zip(m2.iter())
      .map(|(row1, row2)| row1.iter()
                              .zip(row2.iter())
                              .map(|(tree1, tree2)| f(*tree1, *tree2))
                              .collect_vec())
      .collect()
}

// Part 2
fn visibility_at_index(i: usize, v: &Vec<u32>) -> u32 {
    let size = v[i];
    let mut acc1 = 0;
    for j in (0..i).rev() {
        acc1 = acc1 + 1;
        if v[j] >= size { break; }
    }
    let mut acc2 = 0;
    for j in i+1..v.len() {
        acc2 = acc2 + 1;
        if v[j] >= size { break; }
    }
    return acc1 * acc2
}

fn visibility(v: &Vec<u32>) -> Vec<u32> {
    (0..v.len()).into_iter().map(|i| visibility_at_index(i, v)).collect()
}

fn main() {
    let rows: Vec<Vec<u32>> = include_str!("../../../inputs/2022_day08.txt").lines().map(str2nums).collect();
    let columns = transpose(&rows);

    let left_right = mark_field(&rows);
    let top_bottom = transpose(&mark_field(&columns));

    let field = compare_matrices(&left_right, &top_bottom, cmp::max);
    let result: u32 = field.iter().map(|row| row.iter().sum::<u32>()).sum();
    println!("Part1: {:?}", result);

    let vis_left_right = rows.iter().map(visibility).collect();
    let vis_top_bottom = transpose(&columns.iter().map(visibility).collect());
    let field = compare_matrices(&vis_left_right, &vis_top_bottom, |l, r| l*r);
    let result = field.iter().map(|row| row.iter().max().unwrap()).max().unwrap();
    println!("Part2: {:?}", result)
}
