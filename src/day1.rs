use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::collections::HashMap;

pub fn start() {
    read_file().unwrap();
}

fn read_file() -> io::Result<()> {
    let file = File::open("inputs/day1/input.txt")?;

    let reader = BufReader::new(file);

    let mut left_nums: Vec<i32> = Vec::new();
    let mut right_nums: Vec<i32> = Vec::new();

    let mut map: HashMap<i32, i32> = HashMap::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let parts: Vec<&str> = line.split_whitespace().collect();

                if let (Ok(left), Ok(right)) = (parts[0].parse(), parts[1].parse()) {
                    left_nums.push(left);
                    right_nums.push(right);
                } else {
                    eprintln!("Error parsing numbers from line: {}", line);
                }
            },
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    left_nums.sort();
    right_nums.sort();

    if left_nums.len() != right_nums.len() {
        eprintln!("Lengths are not equal");
    }

    for i in left_nums.iter() {
        let occurences  = right_nums.iter().filter(|&&x| x == *i).count() as i32;
        if !map.contains_key(i) {
            map.insert(*i, occurences);
        }
    }

    let mut distance = 0;
    let mut similarity = 0;

    for i in 0..left_nums.len() {
        let bigger: i32;
        let smaller: i32;

        if !map.contains_key(&left_nums[i]) {
            eprint!("Error reading HashMap");
        }
        if *map.get(&left_nums[i]).unwrap() > 0 {
            similarity += (left_nums[i] * map.get(&left_nums[i]).unwrap()).abs();
        }

        match left_nums[i].cmp(&right_nums[i]) {
            std::cmp::Ordering::Less => {
                bigger = right_nums[i];
                smaller = left_nums[i];
            }
            std::cmp::Ordering::Greater => {
                bigger = left_nums[i];
                smaller = right_nums[i];
            }
            std::cmp::Ordering::Equal => {
                bigger = left_nums[i];
                smaller = right_nums[i];
            }
        }
        distance += (bigger - smaller).abs();
    }

    println!("Similarity: {}", similarity);
    println!("Distance: {}", distance);

    Ok(())
}