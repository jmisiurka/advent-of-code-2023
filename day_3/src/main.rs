use std::fs::File;
use std::io;
use std::io::BufRead;
use regex::{Regex, Match};

fn main() {
    let input = read_input("input.txt").unwrap();

    let result = part_one(&input);

    println!("{}", result);
}

fn part_one(lines: &Vec<String>) -> u32 {
    let mut rows = Vec::<Vec::<usize>>::new();
    //let mut matches = Vec::new();
    let mut sum = 0;

    let regex = Regex::new(r"([0-9])\w+|[0-9]").unwrap();
    
    let mut lines = lines.clone();

    let line_width = lines.first().unwrap().len();
    lines.insert(0, ".".repeat(line_width));
    lines.insert(lines.len(), ".".repeat(line_width));
    
    for line in &mut lines {
        line.insert(line_width, '.');
        line.insert(0, '.');
        let mut row_symbols = Vec::<usize>::new();
        for (i, c) in line.chars().enumerate() {
            if c.is_ascii_punctuation() && c != '.'
            {
                row_symbols.push(i);
            }
            //matches.push(regex.find_iter(line));
        }
        rows.push(row_symbols);

        println!("{line}");
    };

    let matches: Vec<Vec<Match>> = lines.iter()
                        .map(|line| regex.find_iter(line).collect())
                        .collect();
    
    //println!("{:?}", matches);

    for (row_index, matches_row) in matches.iter().enumerate() {
        for match_ in matches_row {
            let span = (match_.start(), match_.end());
            
            if row_index != 0 && row_index != matches.len() && !match_.is_empty() {
                for i in 0..3 {
                    let row = rows[row_index + i - 1].clone();
            
                    //println!("{:?}", row);
                    //println!("{:?}", match_);

                    if row.into_iter().any(|x| x + 1 >= span.0 && x <= span.1) {
                        //println!("{:?}", match_);
                        sum += match_.as_str().parse::<u32>().unwrap();
                    }
                }
            }
        }
    };

    return sum;
}

fn read_input(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<io::Result<_>>()?;
    Ok(lines)
}