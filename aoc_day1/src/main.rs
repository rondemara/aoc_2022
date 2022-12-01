use std::fs::File;
use std::io::{BufRead, BufReader};

fn calculate_maximum(file: &str) {
    let mut v : Vec<i32> = Vec::new();
    let mut accum = 0;

    let ifile = File::open(file).unwrap();
    let ifile = BufReader::new(ifile);

    for line in ifile.lines() {
        let s = line.unwrap();
        if s.is_empty() {
            v.push(accum);
            accum = 0;
        } else {
            accum += s.parse::<i32>().unwrap();
        }
    }

    // Reverse sort the array
    v.sort_by(|a,b| b.cmp(a));

    println!("Max: {}, Sum of Top 3: {}", v[0], v[0..3].into_iter().sum::<i32>());

}


fn main() {
    calculate_maximum("/home/demarr/Projects/aoc_2022/aoc_day1/res/input.txt");
}
