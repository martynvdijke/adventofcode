use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Lines;

//read input
fn read_input() -> String {
    std::fs::read_to_string("src/input.txt").unwrap()
}

fn main() {
    let input = read_input();
    let mut score: u32 = 0;



    let test:u32;
    score = input.lines().filter_map(|line| {
        let line = line.as_bytes();
        let (left, right) = line.split_at(line.len() / 2);

        left.iter().find(|item| right.contains(item))
        .map(|item| match item {
            b'a'..=b'z' => (item - b'a') + 1,
            _ => (item - b'A') + 1 + 26,
        } as u32) 
    }).sum();


    println!("{}", score);
}
