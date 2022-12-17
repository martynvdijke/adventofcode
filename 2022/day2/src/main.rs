use std::collections::HashMap;

fn read_input() -> String {
    std::fs::read_to_string("src/input.txt").unwrap()
}

fn rock(own: String) -> String {
    match own.as_str() {
        "X" => "draw".to_owned(),
        "Y" => "win".to_owned(),
        "Z" => "lost".to_owned(),
        _ => "Error".to_owned(),
    }
}

fn paper(own: String) -> String {
    match own.as_str() {
        "X" => "lost".to_owned(),
        "Y" => "draw".to_owned(),
        "Z" => "win".to_owned(),
        _ => "Error".to_owned(),
    }
}

fn scissors(own: String) -> String {
    match own.as_str() {
        "X" => "win".to_owned(),
        "Y" => "lost".to_owned(),
        "Z" => "draw".to_owned(),
        _ => "Error".to_owned(),
    }
}

fn lose(opponent: String) -> String {
    match opponent.as_str() {
        "A" => "scissors".to_owned(),
        "B" => "rock".to_owned(),
        "C" => "paper".to_owned(),
        _ => "Error".to_owned(),
    }
}

fn draw(opponent: String) -> String {
    match opponent.as_str() {
        "A" => "rock".to_owned(),
        "B" => "paper".to_owned(),
        "C" => "scissors".to_owned(),
        _ => "Error".to_owned(),
    }
}

fn win(opponent: String) -> String {
    match opponent.as_str() {
        "A" => "paper".to_owned(),
        "B" => "scissors".to_owned(),
        "C" => "rock".to_owned(),
        _ => "Error".to_owned(),
    }
}

fn part1() {
    let input = read_input();
    let mut score: u64 = 0;
    let mut opponent: String;
    let mut own: String;

    //generate hashmap to store point value
    let mut points = HashMap::new();
    points.insert("X", 1);
    points.insert("Y", 2);
    points.insert("Z", 3);
    points.insert("lost", 0);
    points.insert("draw", 3);
    points.insert("win", 6);

    //loop over every line in txt and find out the score, first count points based on action then on outcome.
    for line in input.lines() {
        let split: Vec<&str> = line.split(' ').collect();

        opponent = split.get(0).unwrap().to_string();
        own = split.get(1).unwrap().to_string();

        score += points.get(&*own).unwrap();

        let outcome: String = match opponent.as_str() {
            "A" => rock(own),
            "B" => paper(own),
            "C" => scissors(own),
            _ => "Error".to_owned(),
        };
        score += points.get(&*outcome).unwrap();
    }
    println!("{}", score);
}

fn part2() {
    let input = read_input();
    let mut score: u64 = 0;
    let mut opponent: String;
    let mut own: String;

    //generate hashmap to store point value
    let mut points = HashMap::new();
    points.insert("X", 0);
    points.insert("Y", 3);
    points.insert("Z", 6);
    points.insert("rock", 1);
    points.insert("paper", 2);
    points.insert("scissors", 3);

    //loop over every line in txt and find out the score, first count points based on action then on outcome.
    for line in input.lines() {
        let split: Vec<&str> = line.split(' ').collect();

        opponent = split.get(0).unwrap().to_string();
        own = split.get(1).unwrap().to_string();

        score += points.get(&*own).unwrap();

        let outcome: String = match own.as_str() {
            "X" => lose(opponent),
            "Y" => draw(opponent),
            "Z" => win(opponent),
            _ => "Error".to_owned(),
        };
        score += points.get(&*outcome).unwrap();
    }
    println!("{}", score);
}

fn main() {
    part1();
    part2();
}