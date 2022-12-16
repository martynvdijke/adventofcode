use std::collections::HashMap;

fn read_input() -> String {
    std::fs::read_to_string("src/input.txt").unwrap()
}

fn rock(own:String) -> str {
    println!("Rock");
    match own.as_str() {
        "X" => "draw",
        "Y" => "win",
        "Z" => "lost",
        _ => "Error",
    }
}

fn paper(own:String) -> str {
    println!("Paper");
    match own.as_str() {
        "X" => "lost",
        "Y" => "draw",
        "Z" => "win",
        _ => "Error",
    }
}

fn scissors(own:String) -> str {
    println!("Scissors");
    match own.as_str() {
        "X" => "lost",
        "Y" => "win",
        "Z" => "draw",
        _ => "Error",
    }
}


fn error(){
    println!("Error");
}

fn main() {
    let input = read_input();
    let mut score: u64 = 0;
    let mut opponent: String;
    let mut own: String;

    let mut points = HashMap::new();
    points.insert("X", 1);
    points.insert("Y", 2);
    points.insert("Z", 3);
    points.insert("lost", 0);
    points.insert("draw", 3);
    points.insert("win", 6);

    // let test: i32 = *points.get("A").unwrap();
    // println!("{}", test );

    for line in input.lines(){
        let split: Vec<&str> = line.split(' ').collect();
        opponent = split.get(0).unwrap().to_string();
        own = split.get(1).unwrap().to_string();
        score += points.get(&*own).unwrap();
        let mut temp:String;
        temp = match opponent.as_str() {
            "A" => rock(own),
            "B" => paper(own),
            "C" => scisors(own),
            _ => "Error",
        };
        println!("{}", score);
        // opponent = split;
        // split.next();
        // own = split;
        // println!("{}, {}", opponent,own)

    }
    println!("{}", score);
}
