use std::fs::File;
use std::io::{self, BufRead};
use std::error::Error;
use regex::{Regex, Match};

fn main() {
    let lines = read_input("./src/input.txt").unwrap();

    //println!("{}", part_one(&lines));

    let two = part_two(&lines);
    match two {
        Ok(sum) => println!("{}", sum),
        Err(_) => println!("Error")
    }
}

fn part_two(lines: &Vec<String>) -> Result<u32, Box<dyn Error>> {
    let regex_digits: Vec<Regex> = vec![Regex::new("one")?, Regex::new("two")?, Regex::new("three")?, 
                                        Regex::new("four")?, Regex::new("five")?, Regex::new("six")?,
                                        Regex::new("seven")?, Regex::new("eight")?, Regex::new("nine")?];

    let mut new_lines = Vec::new(); 

    for line in lines {
        let mut matches: Vec<(usize, Match)> = Vec::new();
        for (digit, regex) in regex_digits.iter().enumerate() {
            for match_ in regex.find_iter(line) {
                matches.push((digit, match_));
            }
        }

        matches.sort_by(|a, b| a.1.start().cmp(&b.1.start()));
        
        let mut new_line = String::from(line);
        for match_ in matches {
            let replacement = char::from_u32(match_.0 as u32 + '1' as u32).unwrap().to_string() + match_.1.as_str();
            new_line = regex_digits[match_.0].replace_all(&new_line, replacement).to_string();
        }

        for digit in &regex_digits {
            new_line = digit.replace_all(&new_line, "").to_string();
        }

        new_lines.push(new_line);
    }

    let result = part_one(&new_lines);

    return Ok(result);
}

fn part_one(lines: &Vec<String>) -> u32 {
    let mut sum: u32 = 0;

    let lines = lines.clone();

    for mut line in lines {
        print!("{} => ", line);
        line.retain(|c| c <= '9' && c >= '0');

        let calibration_value = line.chars().nth(0).unwrap().to_digit(10).unwrap() * 10 + line.chars().last().unwrap().to_digit(10).unwrap();

        println!("{} => {}", line, calibration_value);

        sum += calibration_value;
    }

    return sum;
}

fn read_input(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<io::Result<_>>()?;
    Ok(lines)
}
