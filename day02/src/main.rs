use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str;

#[derive(Debug)]
struct Game {
    id: i32,
    red: i32,
    green: i32,
    blue: i32,
}

impl Game {
    pub fn new(id: i32, red: i32, green: i32, blue: i32) -> Self {
        Game {
            id,
            green,
            red,
            blue,
        }
    }

    pub fn new_from_string(s: String) -> Self {
        let str_game_id: Vec<String> = s.split(':').map(|s| s.trim().to_string()).collect();
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        let id = str_game_id
            .first()
            .unwrap()
            .split(' ')
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let data = str_game_id.last().unwrap();

        for tirage in data.split([';']).map(|s| s.trim().to_string()) {
            for part in tirage.split([',']).map(|s| s.trim().to_string()) {
                let tmp = get_color_value(part.clone());
                if part.ends_with("red") {
                    red = if tmp > red { tmp } else { red };
                } else if part.ends_with("blue") {
                    let tmp = get_color_value(part);
                    blue = if tmp > blue { tmp } else { blue };
                } else {
                    let tmp = get_color_value(part);
                    green = if tmp > green { tmp } else { green };
                }
            }
        }

        Game {
            id,
            blue,
            green,
            red,
        }
    }
}

fn get_color_value(s: String) -> i32 {
    s.split(' ')
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .parse::<i32>()
        .unwrap()
}

fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let lines: Vec<String> = read_lines("input.txt").expect("Can't read file");
    let mut games: Vec<Game> = vec![];
    let mut result = 0;
    let default_game = Game::new(-1, 12, 13, 14);

    for line in lines {
        let game = Game::new_from_string(line);
        games.push(game);
    }

    for game in games {
        if game.red <= default_game.red
            && game.blue <= default_game.blue
            && game.green <= default_game.green
        {
            result += game.id;
        }
    }

    println!("{}", result);
}
