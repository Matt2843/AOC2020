use std::fs;
use regex::Regex;

fn guided_binary_search(seat: &str) -> u32 {
    let len = seat.chars().count() as u32;
    let mut min: u32 = 0;
    let mut max: u32 = u32::pow(2, len) - 1;
    for c in seat.chars() {
        if c == 'B' || c == 'R' { // upper half
            min += (max - min) / 2 + 1;
        } else if c == 'F' || c == 'L' { // lower half
            max = min + (max - min) / 2;
        }
    }
    max
}

fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("failed to read file..");
    let re = Regex::new(r"(?P<row>[FB]+)(?P<col>[RL]+)").unwrap();
    let captures = content.trim().split('\n')
        .map(|x| re.captures(&x).unwrap());

    let mut res: Vec<u32> = Vec::new();

    for c in captures {        
        let row = guided_binary_search(&c["row"]);
        let col = guided_binary_search(&c["col"]);
        res.push(row * 8 + col);
    }
    println!("q1answer: {:?}", res.iter().max().unwrap());
    res.sort_unstable();
    let mut empty_seat = res[0];
    for i in res {
        if i > empty_seat + 1 {
            println!("q2answer: {}", i - 1);
        }
        empty_seat = i;
    }
}
