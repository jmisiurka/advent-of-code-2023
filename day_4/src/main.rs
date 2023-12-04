use std::io;
use std::io::BufRead;
use std::fs::File;
use std::error::Error;

fn main() {
    let input = read_input("input.txt").unwrap();

    let result = part_one(&input).unwrap();

    println!("{result}");
}

fn part_one(input: &Vec<String>) -> Result<u32, Box<dyn Error>> {
    let mut total_score: u32 = 0;

    for card in input {
        let mut score: u32 = 0;

        let split = card.splitn(2, ':').collect::<Vec<&str>>();
        let numbers = split[1].split('|').collect::<Vec<&str>>();

        let winning_numbers = numbers[0].split_whitespace().map(|number| number.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let card_numbers = numbers[1].split_whitespace().map(|number| number.parse::<u32>().unwrap()).collect::<Vec<u32>>();

        for number in winning_numbers {
            if card_numbers.iter().any(|num| num == &number) {
                score += 1;
            }
        }

        if score > 0
        {
            let mut temp = 1;
            score -= 1;
            temp *= 2_u32.pow(score);
            total_score += temp;
        }
    };

    Ok(total_score)
}

fn read_input(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<io::Result<_>>()?;
    Ok(lines)
}