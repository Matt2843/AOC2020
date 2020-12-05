use std::fs;
use regex::Regex;

fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("failed to read file..");

    let byr = Regex::new(r"byr:(?P<byr>\d{4})").unwrap();
    let iyr = Regex::new(r"iyr:(?P<iyr>\d{4})").unwrap();
    let eyr = Regex::new(r"eyr:(?P<eyr>\d{4})").unwrap();
    let hgt = Regex::new(r"hgt:(?P<hgtv>\d+)(?P<hgtu>cm|in)").unwrap();
    let hcl = Regex::new(r"hcl:#[0-9a-fA-F]{6}").unwrap();
    let ecl = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
    let pid = Regex::new(r"pid:[0-9]{9}").unwrap();

    let mut total = 0;
    let lines: Vec<&str> = content.trim().split("\n\n").collect();
    for x in lines {
        let trimmed = x.replace("\n", " ");
        if !byr.is_match(&trimmed) {
            continue;
        } else {
            let byrv = byr.captures(&trimmed).unwrap()["byr"].parse::<u16>().unwrap();
            if byrv < 1920 || byrv > 2002 {
                continue;
            }
        }

        if !iyr.is_match(&trimmed) {
            continue;
        } else {
            let iyrv = iyr.captures(&trimmed).unwrap()["iyr"].parse::<u16>().unwrap();
            if iyrv < 2010 || iyrv > 2020 {
                continue;
            }
            
        }

        if !eyr.is_match(&trimmed) {
            continue;
        } else {
            let eyrv = eyr.captures(&trimmed).unwrap()["eyr"].parse::<u16>().unwrap();
            if eyrv < 2020 || eyrv > 2030 {
                continue;
            }
        }

        if !hgt.is_match(&trimmed) {
            continue;
        } else {
            let capture = hgt.captures(&trimmed).unwrap();
            let hgtv = &capture["hgtv"].parse::<u16>().unwrap();
            let hgtu = &capture["hgtu"];
            if hgtu == "cm" {
                if *hgtv < 150 || *hgtv > 193 {
                    //println!("Skipping {} hgt out of cm range", trimmed);
                    continue;
                }
            } else if hgtu == "in" {
                if *hgtv < 59 || *hgtv > 76 {
                    //println!("Skipping {} hgt out of in range", trimmed);
                    continue;
                }
            } else {
                //println!("Skipping {} no valid hgt unit", trimmed);
                continue;
            }
        }

        if !hcl.is_match(&trimmed) {
            continue;
        } 

        if !ecl.is_match(&trimmed) {
            continue;
        }

        if !pid.is_match(&trimmed) {
            continue;
        }
        total += 1;
    }
    println!("{}", total);
}
