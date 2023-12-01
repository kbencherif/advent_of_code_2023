use std::collections::HashMap;
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

fn process_line(line: String) -> String {
    let mut result: String = String::new();
    let mut index: usize = 0;

    while index != line.len() {
        let tmp = &line[index..];
        if tmp.starts_with("one") {
            result.push('1');
        } else if tmp.starts_with("two") {
            result.push('2');
        } else if tmp.starts_with("three") {
            result.push('3');
        } else if tmp.starts_with("four") {
            result.push('4');
        } else if tmp.starts_with("five") {
            result.push('5');
        } else if tmp.starts_with("six") {
            result.push('6');
        } else if tmp.starts_with("seven") {
            result.push('7');
        } else if tmp.starts_with("eight") {
            result.push('8');
        } else if tmp.starts_with("nine") {
            result.push('9');
        } else {
            result.push(line.chars().nth(index).unwrap());
        }
        index += 1;
    }
    return result;
}

fn main() {
    let lines: Vec<String> = read_lines("input.txt").expect("Can't read file");
    let mut result: i32 = 0;

    for mut line in lines {
        line = process_line(line);
        println!("{:?}", line);
        result = result + combine_digit(line);
    }
    println!("{:?}", result);
}
