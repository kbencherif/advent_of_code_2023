use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    As,
}

impl Card {
    fn from_char(c: char) -> Result<Card, ()> {
        match c {
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::Ten),
            'J' => Ok(Card::Jack),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::As),
            _ => {
                println!("c => {}", c);
                Err(())
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Game {
    hand_result: Hand,
    bid: u32,
    hand: [Card; 5],
}

impl Game {
    fn new(bid: u32, hand: [Card; 5]) -> Self {
        let mut rec = [0usize; 5];
        for (i, card) in hand.iter().enumerate() {
            let tmp = hand.iter().filter(|e| **e == *card).count();
            rec[i] = tmp;
        }
        Self {
            hand_result: Game::hand_to_enum(rec),
            bid,
            hand,
        }
    }

    fn hand_to_enum(mut hand: [usize; 5]) -> Hand {
        hand.sort_by(|a, b| b.cmp(a));
        let mut cards_recurrence = hand.into_iter().collect::<Vec<usize>>();

        if cards_recurrence.iter().filter(|i| **i == 2).count() == 4 {
            return Hand::TwoPair;
        }
        cards_recurrence.dedup();
        if cards_recurrence[0] == 3 && cards_recurrence[1] == 2 {
            return Hand::FullHouse;
        }
        match cards_recurrence[0] {
            1 => Hand::HighCard,
            2 => Hand::OnePair,
            3 => Hand::ThreeOfAKind,
            4 => Hand::FourOfAKind,
            5 => Hand::FiveOfAKind,
            _ => Hand::HighCard,
        }
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_result.cmp(&other.hand_result) == Ordering::Equal {
            for i in 0..5usize {
                if self.hand[i] != other.hand[i] {
                    return self.hand[i].cmp(&other.hand[i]);
                }
            }
        }
        self.hand_result.cmp(&other.hand_result)
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_result.cmp(&other.hand_result) == Ordering::Equal {
            for i in 0..5usize {
                if self.hand[i] != other.hand[i] {
                    return Some(self.hand[i].cmp(&other.hand[i]));
                }
            }
        }
        Some(self.hand_result.cmp(&other.hand_result))
    }
}

impl Eq for Game {}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        if self.hand_result == other.hand_result {
            for i in 0..5usize {
                if self.hand[i] != other.hand[i] {
                    return false;
                }
            }
            return true;
        }
        false
    }
}

fn main() -> io::Result<()> {
    let lines = read_lines("input.txt")?;
    let mut games: Vec<Game> = lines.into_iter().map(|l| get_card_type(l)).collect();
    games.sort();
    println!(
        "Game {:?}",
        games
            .iter()
            .enumerate()
            .map(|(i, e)| e.bid * (i as u32 + 1))
            .sum::<u32>()
    );
    Ok(())
}

fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn get_card_type(l: String) -> Game {
    let (hand_str, bid_str) = l.split_once(" ").unwrap();
    let hand: Vec<Card> = hand_str
        .chars()
        .map(|c| Card::from_char(c))
        .flatten()
        .collect();
    let mut array_hand = [Card::As; 5];

    for (i, card) in hand.iter().enumerate() {
        array_hand[i] = *card
    }

    Game::new(bid_str.parse::<u32>().unwrap(), array_hand)
}
