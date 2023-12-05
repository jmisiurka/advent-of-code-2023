use std::io;
use std::io::BufRead;
use std::fs::File;
use std::error::Error;
use std::ops::Range;

fn main() {
    let input = read_input("input.txt").unwrap();

    let result = part_one(&input).unwrap();

    println!("{result}");
}

fn part_one(input: &Vec<String>) -> Result<i64, Box<dyn Error>> {
    let mut maps = Vec::new();
    let mut seeds = Vec::new();

    let mut map: Vec::<(Range<i64>, i64)> = Vec::new();

    for line in input {
        if line.contains("seeds") {
            seeds = line.split(':').collect::<Vec<&str>>()[1].split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        } else if line.contains("map") {
            if !map.is_empty() {
                maps.push(map.clone());
                map.clear();
            }
        } else if !line.is_empty() {
            let numbers = line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            let dest_range_start = numbers[0];
            let source_range_start = numbers[1];
            let range_length = numbers[2];

            map.push((source_range_start as i64..(source_range_start as i64 + range_length as i64), dest_range_start - source_range_start));
        }
    };

    maps.push(map.clone());

    let mut min = std::i64::MAX;


    //for map_ in &maps {
    //    for offset in map_ {
    //       println!("{:?} => {}", offset.0, offset.1);
    //    }
    //}

    for i in (0..seeds.len()).step_by(2)  {
        for seed_id in seeds[i]..(seeds[i] + seeds[i + 1]) {
            let mut location = seed_id as i64;
                //println!("!!!SEED STARTED");
        
                for map_ in &maps {
                    'offset:
                    for offset in map_ {
                        //println!("RANGE: {:?}", offset.0);
                        if offset.0.contains(&location) {
                            location += offset.1;
                            break 'offset;
                        }
                    }
                };
        
                if location < min {
                    min = location;
                    println!("New min: seed {seed_id} => location: {location}");
                };

            }
    }


    Ok(min)
}

fn read_input(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<io::Result<_>>()?;
    Ok(lines)
}