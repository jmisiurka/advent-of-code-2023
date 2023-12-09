use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let input = read_input("input.txt").unwrap();

    let result = solve(&input, true);

    println!("{result}");
}

fn solve(input: &Vec<String>, part_two: bool) -> i32 {
    let mut total = 0;
    
    for line in input {
        let history: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

        let result: i32;

        if !part_two {
            result = extrapolate_forward(&history) + history.last().unwrap();
        } else {
            result = history[0] - extrapolate_backward(&history);
        }

        //println!{"{line} => {result}"};
        
        total += result;
    };

    return total;
}

fn extrapolate_forward(history: &Vec<i32>) -> i32 {
    //println!("History input: {:?}", history);

    let mut new_sequence = Vec::new();
    for i in 0..history.len() - 1 {
        new_sequence.push(history[i + 1] - history[i]);
    }
        
    if new_sequence.iter().all(|x| x == &0) {
        return 0;
    }

    let extrapolated_number = extrapolate_forward(&new_sequence);
        
    //println!{"History output: {:?}", new_sequence};
    return *new_sequence.last().unwrap() + extrapolated_number;
}

fn extrapolate_backward(history: &Vec<i32>) -> i32 {
    //println!("History input: {:?}", history);

    let mut new_sequence = Vec::new();
    for i in 0..history.len() - 1 {
        new_sequence.push(history[i + 1] - history[i]);
    }
        
    if new_sequence.iter().all(|x| x == &0) {
        return 0;
    }

    let extrapolated_number = extrapolate_backward(&new_sequence);
        
    //println!{"History output: {extrapolated_number} {:?}", new_sequence};
    return new_sequence[0] - extrapolated_number;
}

fn read_input(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<io::Result<_>>()?;
    Ok(lines)
}