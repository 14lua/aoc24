use std::fs::File;
use std::io::{self, BufReader, BufRead};

pub fn start() {
    read_file().unwrap();
}

fn read_file() -> io::Result<()> {
    let file = File::open("inputs/day2/input.txt")?;

    let reader = BufReader::new(file);

    let nums: Vec<Vec<i32>> = reader.lines().map(
        |l| l.unwrap().split_whitespace().map(|n| n.parse().unwrap()).collect()
    ).collect();

    let mut total_safe = 0;

    'outer: for list in nums {
        let mut increasing = true;
        let mut decreasing = true;
        for i in 0..list.len() - 1 {
            let current = list[i];
            let next = list[i + 1];
            let diff = (current - next).abs();

            if diff > 3 { continue 'outer; }

            match current.cmp(&next) {
                std::cmp::Ordering::Equal => continue 'outer,
                std::cmp::Ordering::Greater => increasing = false,
                std::cmp::Ordering::Less => decreasing = false
            }
        }
        if (increasing && !decreasing) || (!increasing && decreasing) { total_safe += 1; }
    }

    println!("{}", total_safe);
    Ok(())
}