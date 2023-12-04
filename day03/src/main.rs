use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Copy, Clone)]
struct Position {
    x: i32,
    x_max: i32,
    num: i32,
    y: i32,
}

fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn fill_position(line: &String, first: usize, y: usize) -> Option<Position> {
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
            x: first as i32,
            x_max: if end == 0 {
                (line.len() as i32) - 1
            } else {
                end
            },
            num,
            y: y as i32,
        }),
        Err(_) => None,
    }
}

fn is_symbol_above(pos: &Position, line_above: Option<&String>) -> bool {
    if line_above.is_none() {
        return false;
    }

    for i in pos.x - 1..pos.x_max + 2 {
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
    if pos.x - 1 > 0 {
        if line.chars().nth((pos.x - 1) as usize).unwrap() != '.' {
            return true;
        }
    }
    if (pos.x_max as usize) + 1 < line.len() {
        if line.chars().nth((pos.x_max + 1) as usize).unwrap() != '.' {
            return true;
        }
    }
    false
}

fn is_symbol_below(pos: &Position, line_below: Option<&String>) -> bool {
    if line_below.is_none() {
        return false;
    }

    for i in pos.x - 1..pos.x_max + 2 {
        if i < 0 || i as usize >= line_below.unwrap().len() {
            continue;
        }
        if line_below.unwrap().chars().nth(i as usize).unwrap() != '.' {
            return true;
        }
    }
    false
}

fn process_line(line: &String, y: usize) -> Vec<Position> {
    let mut index: usize = 0;
    let mut pos: Vec<Position> = vec![];

    for (i, c) in line.chars().enumerate() {
        if i < index {
            continue;
        }
        if c.is_digit(10) {
            let maybe_p = fill_position(line, index, y);
            match maybe_p {
                Some(p) => {
                    index = (p.x_max + 1) as usize;
                    pos.push(p)
                }
                None => continue,
            }
        } else {
            index += 1
        }
    }

    return pos;
}

fn check_adjacent(above: Option<&String>, mid: &String, below: Option<&String>, y: usize) -> i32 {
    let pos: Vec<Position> = process_line(mid, y);

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

fn get_gear_result(pos: &Vec<Position>, line: &String, y: i32) -> i32 {
    let mut result: i32 = 0;

    for (i, c) in line.chars().enumerate() {
        let mut tmp: Vec<i32> = vec![];
        if c == '*' {
            pos.into_iter().for_each(|p: &Position| {
                if p.x - 1 <= i as i32 && p.x_max + 1 >= i as i32 && p.y <= y + 1 && p.y >= y - 1 {
                    tmp.push(p.num);
                }
            });
            if tmp.len() > 1 {
                result += tmp.into_iter().product::<i32>();
            }
        }
    }
    result
}

fn main() {
    let lines: Vec<String> = read_lines("input.txt").expect("Can't read file");
    let (left, right) = lines.split_at(2);
    let mut result: i32 = left
        .windows(2)
        .map(|obj| check_adjacent(None, &obj[0], Some(&obj[1]), 0))
        .sum();
    result += lines
        .windows(3)
        .enumerate()
        .map(|(i, obj)| check_adjacent(Some(&obj[0]), &obj[1], Some(&obj[2]), i + 1))
        .sum::<i32>();

    println!("length => {}", lines.len());
    result += right
        .into_iter()
        .rev()
        .take(2)
        .collect::<Vec<&String>>()
        .windows(2)
        .map(|obj| check_adjacent(Some(obj[1]), obj[0], None, lines.len() - 1))
        .sum::<i32>();

    println!("{}", result);
    let pos: Vec<Position> = lines
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, l)| process_line(&l, i))
        .flatten()
        .collect();

    result = lines
        .into_iter()
        .enumerate()
        .map(|(i, s)| get_gear_result(&pos, &s, i as i32))
        .sum::<i32>();
    println!("{}", result);
}
