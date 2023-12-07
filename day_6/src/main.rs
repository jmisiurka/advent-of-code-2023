use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let input = read_input("input.txt").unwrap();

    let result = part_one(&input);

    println!("Part one: {result}");

    let result = part_two(&input);
    
    println!("Part two: {result}")
}

fn part_one(input: &Vec<String>) -> usize {
    let times: Vec<usize> = input[0].split(':').last().unwrap().split_whitespace().map(|time| time.parse::<usize>().unwrap()).collect();
    let records: Vec<usize> = input[1].split(':').last().unwrap().split_whitespace().map(|record| record.parse::<usize>().unwrap()).collect();

    let mut total = 1;

    for i in 0..times.len() {
        let mut hold_time = 0;
        while hold_time * (times[i] - hold_time) <= records[i] {
            hold_time += 1;
        }

        total *= times[i] - 2 * hold_time + 1;
    };

    return total;
}

fn part_two(input: &Vec<String>) -> u128 {
    let time = input[0].split(":").last().unwrap().replace(' ', "").parse::<u128>().unwrap();
    let record = input[1].split(":").last().unwrap().replace(' ', "").parse::<u128>().unwrap();

    let mut hold_time = 0;
    while hold_time * (time - hold_time) <= record {
        hold_time += 1;
    }

    return time - 2 * hold_time + 1;
}

fn read_input(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<io::Result<_>>()?;
    Ok(lines)
}