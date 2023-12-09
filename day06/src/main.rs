use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() -> io::Result<()> {
    let lines = read_lines("input.txt")?;
    let (durations, distances): (Vec<i32>, Vec<i32>) = process_input(lines);
    let mut result: Vec<i32> = vec![0; durations.len()];

    for (i, duration) in durations.iter().enumerate() {
        for j in 0..*duration + 1 {
            let tmp: i32 = (duration - j as i32) * j as i32;
            if tmp > distances[i] {
                result[i] += 1;
            }
        }
    }

    println!("result: {:?}", result.iter().product::<i32>());
    Ok(())
}

fn process_input(lines: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let first_line: &String = lines.first().unwrap();
    let last_line: &String = lines.last().unwrap();

    (
        first_line
            .split_whitespace()
            .map(|n| n.parse::<i32>())
            .flatten()
            .collect::<Vec<i32>>(),
        last_line
            .split_whitespace()
            .map(|n| n.parse::<i32>())
            .flatten()
            .collect::<Vec<i32>>(),
    )
}
