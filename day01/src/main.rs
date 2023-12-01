use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn combine_digit(line: String) -> i32 {
    let result: String = line
        .chars()
        .filter(|c: &char| c >= &'0' && c <= &'9')
        .take(1)
        .chain(
            line.chars()
                .rev()
                .filter(|c: &char| c >= &'0' && c <= &'9')
                .take(1),
        )
        .collect();

    result.parse::<i32>().unwrap()
}

fn main() {
    let lines: Vec<String> = read_lines("input.txt").expect("Can't read file");
    let mut result: i32 = 0;

    for line in lines {
        result = result + combine_digit(line);
    }
    println!("{:?}", result)
}
