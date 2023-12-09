use std::collections::HashMap;
use std::io::{self, BufRead};
use std::fs::File;

fn main() {
    let input = read_input("input.txt").unwrap();

//    let result = part_one(&input);

//    println!("{result}");

    let result = part_two(&input);

    println!("{result}");
}

fn part_one(input: &Vec<String>) -> u32 {
    let mut moves = input[0].chars().cycle();

    let mut map = HashMap::<String, (String, String)>::with_capacity(input.len() - 1);

    for line in &input[2..] {
        let cleaned_line = line.replace('=', "")
            .replace('(', "")
            .replace(')', "")
            .replace(',', "");
        let mut split = cleaned_line.split_whitespace();
        let current_location = split.next().unwrap().to_string();
        let left = split.next().unwrap().to_string();
        let right = split.next().unwrap().to_string();

        map.insert(current_location, (left, right));
    };

    let mut counter = 0;
    let mut current = String::from("AAA");

    while current != "ZZZ" {
        current = match moves.next() {
            Some(dir) => match dir {
                'L' => map[&current].0.clone(),
                'R' => map[&current].1.clone(),
                _ => String::from("ERROR"),
            },
            None => String::from("ERROR"),
        };
        counter += 1;
    };

    counter
}

fn part_two(input: &Vec<String>) -> u32 {
    let mut moves = input[0].chars().cycle();

    let mut map = HashMap::<String, (String, String)>::with_capacity(input.len() - 1);

    for line in &input[2..] {
        let cleaned_line = line.replace('=', "")
            .replace('(', "")
            .replace(')', "")
            .replace(',', "");
        let mut split = cleaned_line.split_whitespace();
        let current_location = split.next().unwrap().to_string();
        let left = split.next().unwrap().to_string();
        let right = split.next().unwrap().to_string();

        map.insert(current_location, (left, right));
    };

    let mut counter = 0;
    let mut current = map.keys().filter(|loc| loc.ends_with('A')).map(|loc| String::from(loc)).collect::<Vec<String>>();

    let all_end_with_z = |locations: &Vec<String>| locations.iter().all(|loc| loc.ends_with('Z'));

    while !all_end_with_z(&current) {
        for i in 0..current.len() {
            let dir = moves.next().unwrap();
            cur = match dir {
                'L' => &map[cur].0,
                'R' => &map[cur].1,
                _ => return 0,
            };
        }

        println!("{:?}", current);
        counter += 1;
    };

    counter
}

fn read_input(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<io::Result<_>>()?;
    Ok(lines)
}