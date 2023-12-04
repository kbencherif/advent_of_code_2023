use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Position {
    begining: i32,
    end: i32,
    num: i32,
}

fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn fill_position(line: &String, first: usize) -> Option<Position> {
    let mut end: i32 = 0;
    let mut str_num: String = String::new();

    for (i, c) in line.chars().enumerate() {
        if i >= first {
            if !c.is_digit(10) {
                end = (i - 1) as i32;
                break;
            } else {
                str_num.push(c);
            }
        }
    }
    match str_num.parse::<i32>() {
        Ok(num) => Some(Position {
            begining: first as i32,
            end: if end == 0 {
                (line.len() as i32) - 1
            } else {
                end
            },
            num,
        }),
        Err(_) => None,
    }
}

fn is_symbol_above(pos: &Position, line_above: Option<&String>) -> bool {
    if line_above.is_none() {
        return false;
    }

    for i in pos.begining - 1..pos.end + 2 {
        if i < 0 || i as usize >= line_above.unwrap().len() - 1 {
            continue;
        }
        if line_above.unwrap().chars().nth(i as usize).unwrap() != '.' {
            return true;
        }
    }
    false
}

fn is_symbol_around(pos: &Position, line: &String) -> bool {
    if pos.begining - 1 > 0 {
        if line.chars().nth((pos.begining - 1) as usize).unwrap() != '.' {
            return true;
        }
    }
    if (pos.end as usize) + 1 < line.len() {
        if line.chars().nth((pos.end + 1) as usize).unwrap() != '.' {
            return true;
        }
    }
    false
}

fn is_symbol_below(pos: &Position, line_below: Option<&String>) -> bool {
    if line_below.is_none() {
        return false;
    }

    for i in pos.begining - 1..pos.end + 2 {
        if i < 0 || i as usize >= line_below.unwrap().len() {
            continue;
        }
        if line_below.unwrap().chars().nth(i as usize).unwrap() != '.' {
            return true;
        }
    }
    false
}

fn check_adjacent(above: Option<&String>, mid: &String, below: Option<&String>) -> i32 {
    let mut pos: Vec<Position> = vec![];
    let mut index: usize = 0;

    for (i, c) in mid.chars().enumerate() {
        if i < index {
            continue;
        }
        if c.is_digit(10) {
            let maybe_p = fill_position(mid, index);
            match maybe_p {
                Some(p) => {
                    index = (p.end + 1) as usize;
                    pos.push(p)
                }
                None => continue,
            }
        } else {
            index += 1
        }
    }

    pos.into_iter()
        .map(|p| {
            if is_symbol_above(&p, above) || is_symbol_around(&p, mid) || is_symbol_below(&p, below)
            {
                p.num
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let lines: Vec<String> = read_lines("input.txt").expect("Can't read file");
    let (left, right) = lines.split_at(2);
    let mut result: i32 = left
        .windows(2)
        .map(|obj| check_adjacent(None, &obj[0], Some(&obj[1])))
        .sum();

    result += lines
        .windows(3)
        .map(|obj| check_adjacent(Some(&obj[0]), &obj[1], Some(&obj[2])))
        .sum::<i32>();

    result += right
        .into_iter()
        .rev()
        .take(2)
        .collect::<Vec<&String>>()
        .windows(2)
        .map(|obj| check_adjacent(Some(obj[1]), obj[0], None))
        .sum::<i32>();

    println!("{}", result)
}
