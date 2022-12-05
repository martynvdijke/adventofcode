use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn load_from_file(file_path: &str) {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();
}


fn main() {

    let v: Vec<i32> = Vec::new();

    let v = vec![1000,2000,3000,0,4000,0,5000,6000,0,7000,8000,9000,0,10000,0];

    load_from_file("src/1.txt");

    println!("Hello, world!");
    let mut max: i32 = 0;

    for i in &v {
        // iterate immutably
        let i: &i32 = i; // elements are immutable pointers
        // println!("{}", i);
        if *i != 0 {
            max += i;
        }
        else{
            println!("{}", max);
            max = 0;
        }
    }


}
