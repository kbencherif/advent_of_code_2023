use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Copy, Clone, Debug)]
struct Card {
    nb_match: i32,
    result: i32,
}

fn main() {
    let lines: Vec<String> = read_lines("input.txt").expect("Can't read file");
    let cards: Vec<Card> = lines
        .into_iter()
        .map(|l| process_line(l.split(':').nth(1).unwrap().trim().to_string()))
        .collect();
    let result: i32 = cards.iter().map(|card| card.result).sum();
    println!("{}", result);

    let mut count = vec![1usize; cards.len()];

    cards.into_iter().enumerate().for_each(|(i, card)| {
        if card.nb_match > 0 {
            let n: usize = i + 1;
            for index in n..n + card.nb_match as usize {
                count[index] += count[i];
            }
        }
    });
    println!("{}", count.into_iter().sum::<usize>())
}

fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn fibonnaci(n: i32) -> i32 {
    let mut result: i32 = 1;

    if n <= 1 {
        return n;
    }

    for _i in 1..n {
        result *= 2;
    }
    result
}

fn process_line(s: String) -> Card {
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

    Card {
        nb_match: result,
        result: fibonnaci(result),
    }
}
