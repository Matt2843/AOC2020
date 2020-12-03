use std::fs;

fn traverse(lines: &[&str], right: usize, down: usize) -> u64 {
    let mut treesencountered: u64 = 0;
    let mut pos = right;
    let length = lines[0].len();
    for x in down..lines.len() {
        if x % down != 0 {continue;}
        if let Some(value) = lines[x].get(pos..pos+1) {
            if value == "#" {
                treesencountered += 1;
            }
        }
        pos = (pos + right) % length;
    }
    treesencountered
}

fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("failed to read file..");
    let lines: Vec<&str> = content.trim().split('\n').collect();
    println!("{}", traverse(&lines, 1, 1)
                   *traverse(&lines, 3, 1)
                   *traverse(&lines, 5, 1)
                   *traverse(&lines, 7, 1)
                   *traverse(&lines, 1, 2))
}
