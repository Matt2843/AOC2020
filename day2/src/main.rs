use std::fs;
use regex::Regex;

fn main() {
    let file_content = fs::read_to_string("input.txt")
        .expect("failed to read file..");
    let re = Regex::new(r"(?P<min>\d+)-(?P<max>\d+)\s*?(?P<c>\w):\s*?(?P<v>\w+)").unwrap();

    let captures = file_content.trim()
    .split('\n')
    .map(|x| re.captures(x).unwrap());

    let mut validpasswords = 0;
    for capture in captures {
        let i1 = capture["min"].parse::<usize>().unwrap();
        let i2 = capture["max"].parse::<usize>().unwrap();
        let c = &capture["c"];
        let v = &capture["v"];

        if (v[i1-1..i1] == *c) ^ (v[i2-1..i2] == *c) {
            validpasswords += 1;
        }
    }
    println!("{}", validpasswords);
}