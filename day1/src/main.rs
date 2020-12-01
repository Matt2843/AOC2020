extern crate itertools;

use itertools::Itertools;
use std::fs;

fn q2(x: &Vec<u64>) -> u64 {
    let solutions: Vec<u64> = x.iter().tuple_combinations()
        .filter(|&(a, b, c)| a + b + c == 2020)
        .map(|(&a, &b, &c)| a * b * c)
        .collect();
    solutions[0]
}

fn q1(x: &Vec<u64>) -> u64 {

    let solutions: Vec<u64> = x.iter().tuple_combinations()
        .filter(|&(a, b)| a + b == 2020)
        .map(|(&a, &b)| a * b)
        .collect();
    solutions[0]
}

fn main() {
    let file_content = fs::read_to_string("input")
        .expect("failed to read file..");
    let numbers: Vec<u64> = file_content.trim().split("\n").map(|x| x.parse::<u64>().unwrap()).collect();
    println!("q1: {}", q1(&numbers));
    println!("q2: {}", q2(&numbers));
}
