use std::fs::File;
use std::io::{BufRead, BufReader};

fn run_strategy(file : &str, strategy: i32) {
    let mut score = 0;

    let ifile = File::open(file).unwrap();
    let ifile = BufReader::new(ifile);

    for line in ifile.lines() {
        let s = line.unwrap();
        let vec: Vec<&str> = s.split(" ").collect();
        //println!("Letters: {} {}", vec[0], vec[1]);
        let pair : (&str, &str) = (vec[0], vec[1]);
        
        if strategy == 0 {
            match pair {
                // Rock: AX
                // Paper: BY
                // Scisor: CZ

                //Win
                ("A", "Y") => score += 2+6,
                ("B", "Z") => score += 3+6,
                ("C", "X") => score += 1+6,

                //Tie
                ("A", "X") => score += 1+3,
                ("B", "Y") => score += 2+3,
                ("C", "Z") => score += 3+3,

                //Lose
                ("A", "Z") => score += 3+0,
                ("B", "X") => score += 1+0,
                ("C", "Y") => score += 2+0,
                _ => score +=0
            }
        } else {
            match pair {
                // Rock: AX
                // Paper: BY
                // Scisor: CZ

                //Lose
                ("A", "X") => score += 3+0,
                ("B", "X") => score += 1+0,
                ("C", "X") => score += 2+0,

                //Tie
                ("A", "Y") => score += 1+3,
                ("B", "Y") => score += 2+3,
                ("C", "Y") => score += 3+3,

                //Win
                ("A", "Z") => score += 2+6,
                ("B", "Z") => score += 3+6,
                ("C", "Z") => score += 1+6,
                _ => score +=0
            }
        }
    }
    println!("Your score with strategy {}: {}", strategy, score);
}


fn main() {
    run_strategy("/home/demarr/Projects/aoc_2022/day2/res/input.txt",0);
    run_strategy("/home/demarr/Projects/aoc_2022/day2/res/input.txt",1);
}
