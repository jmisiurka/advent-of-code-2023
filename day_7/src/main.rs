use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let input = read_input("input.txt").unwrap();

    let result = part_one(&input);

    println!("{result}");
}


fn part_one(input: &Vec<String>) -> u32 {
    let mut plays = Vec::<(&str, Hand, u32)>::new();
    
    for line in input {
        let mut line = line.split_whitespace();
        let hand_str = line.next().unwrap();
        let hand = calc_hand(&hand_str);
        let bid = line.next().unwrap().parse::<u32>().unwrap();

        plays.push((hand_str, hand, bid));
    }

    plays.sort_by(|l, r| compare_hands((l.0, l.1.clone()), (r.0, r.1.clone())).unwrap());

    //println!("{:?}", plays);

    let mut sum = 0;

    for (i, play) in plays.iter().enumerate() {
        sum += play.2 * (i as u32 + 1);
    };

    sum
}

fn calc_hand(hand_str: &str) -> Hand {
    let mut ranks = Vec::new();
    for _ in 0..13 {
        ranks.push(0);
    };

    for c in hand_str.chars() {
        match c {
            'A' => ranks[12] += 1,
            'K' => ranks[11] += 1,
            'Q' => ranks[10] += 1,
            'J' => ranks[9] += 1,
            'T' => ranks[8] += 1,
            '9' => ranks[7] += 1,
            '8' => ranks[6] += 1,
            '7' => ranks[5] += 1,
            '6' => ranks[4] += 1,
            '5' => ranks[3] += 1,
            '4' => ranks[2] += 1,
            '3' => ranks[1] += 1,
            '2' => ranks[0] += 1,
            _ => ()
        }
    };

    let mut cloned_ranks = ranks.clone();
    cloned_ranks.sort();
    match cloned_ranks[12] {
        5 => Hand::Five,
        4 => Hand::Four,
        3 => {
            if cloned_ranks[11] == 2 {
                Hand::House
            } else {
                Hand::Three
            }
        },
        2 => {
            if cloned_ranks[11] == 2 {
                Hand::TwoPair
            } else {
                Hand::OnePair
            }
        },
        1 => Hand::High,
        _ => return Hand::Err,
    }
}

fn compare_hands(left: (&str, Hand), right: (&str, Hand)) -> Option<Ordering> {
    if left.1.partial_cmp(&right.1) != Some(Ordering::Equal) {
        return left.1.partial_cmp(&right.1);
    };
    
    let left = left.0.chars().collect::<Vec<char>>();
    let right = right.0.chars().collect::<Vec<char>>();
    
    for i in 0..5 {
        let left_val = match left[i] {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => 0
        };

        let right_val = match right[i] {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => 0
        };

        if left_val != right_val {
            return Some(left_val.cmp(&right_val));
        }
    };

    None
}

fn read_input(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<io::Result<_>>()?;
    Ok(lines)
}

#[derive(PartialEq, Clone, Debug)]
enum Hand {
    Five,
    Four,
    House,
    Three,
    TwoPair,
    OnePair,
    High,
    Err
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_num = match self {
            Hand::Five => 6,
            Hand::Four => 5,
            Hand::House => 4,
            Hand::Three => 3,
            Hand::TwoPair => 2,
            Hand::OnePair => 1,
            Hand::High => 0,
            Hand::Err => -1
        };

        let other_num = match other {
            Hand::Five => 6,
            Hand::Four => 5,
            Hand::House => 4,
            Hand::Three => 3,
            Hand::TwoPair => 2,
            Hand::OnePair => 1,
            Hand::High => 0,
            Hand::Err => -1
        };

        Some(self_num.cmp(&other_num))
    }
}