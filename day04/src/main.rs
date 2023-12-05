use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let lines: Vec<String> = read_lines("input.txt").expect("Can't read file");
    let result: i32 = lines
        .into_iter()
        .map(|l| process_line(l.split(':').nth(1).unwrap().trim().to_string()))
        .sum();

    print!("{}", result);
}

fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn fibonnaci(n: i32) -> i32 {
    let mut result: i32 = 1;

    for _i in 1..n {
        result *= 2;
    }
    result
}

fn process_line(s: String) -> i32 {
    let card: Vec<Vec<i32>> = s
        .split('|')
        .map(|split| {
            split
                .trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let result: i32 = card
        .last()
        .into_iter()
        .map(|e| {
            e.into_iter()
                .filter(|n| card.first().unwrap().contains(n))
                .count() as i32
        })
        .sum();

    if result > 1 {
        fibonnaci(result)
    } else {
        result
    }
}
