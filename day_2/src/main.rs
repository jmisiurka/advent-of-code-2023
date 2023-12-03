use std::io;
use std::io::Error;
use std::io::BufRead;
use std::fs::File;
use std::result::Result::Err;

fn main() {
    let input = read_input("input.txt");

    let input = match input {
        Ok(lines) => lines,
        Err(_) => panic!("Error while reading input."),
    };

    let result = part_one(&input);

    match result {
        Ok(val) => println!("Sum of ids: {}\nGame power sum: {}", val.0, val.1),
        Err(e) => println!("Err: {}", e.to_string()),
    }
}

fn part_one(lines: &Vec<String>) -> Result<(u32, u32), Error> {
    let mut id_sum = 0;
    let mut power_sum = 0;

    for line in lines {
        let mut split = line.splitn(2, ':');
        let game_id = split
                                                    .next()
                                                    .unwrap()
                                                    .chars()
                                                    .filter(|x| x.is_ascii_digit())
                                                    .collect::<String>()
                                                    .parse::<u32>()
                                                    .unwrap();

        let sets = split.next().unwrap().split(';').collect::<Vec<&str>>();
        
        let mut possible = true;

        let mut most_cubes_rgb = (0, 0, 0);

        // 'game label and break statements can be uncommented for part one, but it breaks part two
        //'game:
        for set in sets {
            possible = true;
           let subsets = set.split(','); 
           for subset in subsets {
                let n_of_cubes = subset.chars().filter(|x| x.is_ascii_digit()).collect::<String>().parse::<u32>().unwrap();

                if subset.contains("blue") {
                    if n_of_cubes > most_cubes_rgb.2
                    {
                        most_cubes_rgb.2 = n_of_cubes;
                    }

                    if n_of_cubes > 14 {
                        possible = false;
                        //break 'game; 
                    }
                } else if subset.contains("green") {
                    if n_of_cubes > most_cubes_rgb.1
                    {
                        most_cubes_rgb.1 = n_of_cubes;
                    }

                    if n_of_cubes > 13 {
                        possible = false;
                        //break 'game;
                    }
                } else if subset.contains("red") {
                    if n_of_cubes > most_cubes_rgb.0
                    {
                        most_cubes_rgb.0 = n_of_cubes;
                    }

                    if n_of_cubes > 12 {
                        possible = false;
                        //break 'game;
                    }
                }
            }

        }

        println!("Minimum cubes: {}, {}, {}", most_cubes_rgb.0, most_cubes_rgb.1, most_cubes_rgb.2);

        power_sum += most_cubes_rgb.0 * most_cubes_rgb.1 * most_cubes_rgb.2;

        if possible {
            id_sum += game_id;
        }
    };
    
    Ok((id_sum, power_sum))
}

fn read_input(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<io::Result<_>>()?;
    Ok(lines)
}