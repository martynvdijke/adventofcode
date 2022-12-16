fn read_input() -> String {
    let txt: String = std::fs::read_to_string("src/input.txt").unwrap();
    txt.replace("\n\n","\n0\n")
}

fn part1() {
    let input = read_input();    
    let mut temp: u64 = 0;
    let mut max: u64 = 0;

    for line in input.lines(){
        let value = line.parse::<u64>().unwrap();
        if value != 0 {
            temp = temp+value;
        }
        else{
            if temp > max {
                max = temp;
            }
            temp = 0;
        }
    }
    println!("{}", max);
}

fn part2() {
    let input = read_input();    
    let mut temp: u64 = 0;
    let mut top_three: [u64;3] = [0;3];

    for line in input.lines(){
        let value = line.parse::<u64>().unwrap();
        if value != 0 {
            temp = temp+value;
        }
        else{
            //loop over array, replace first array value if temp>array[i] 
            'outer: for x in top_three.iter_mut() {
                if temp > *x {
                    *x = temp;
                    break 'outer;
                }
            }
            temp = 0;
        }
    }
    let sum:u64 = top_three.iter().sum();
    println!("{}", sum);
}

fn main() {
    part1();
    part2();
}

