use std::fs::File;
use std::io::{BufRead, BufReader};

// Map a character to an integer "score"
fn score (letter : char, debug : bool) -> u32 {
    let sc = match letter {
        'a'..='z' => {letter as u32 - 'a' as u32 + 1},
        'A'..='Z' => {letter as u32 - 'A' as u32 + 27},
        _ => {0}
    };
    if debug {println!("Score: {}", sc)}
    return sc
}

fn part1(file : &str) -> u32 {
    let ifile = File::open(file).unwrap();
    let ifile = BufReader::new(ifile);
    let mut score : u32 = 0;

    for line in ifile.lines() {
        let row = line.unwrap();
        let pair : (&str, &str) = row.split_at(row.chars().count()/2);
        score += find_intersections(pair);
    }
    return score
}

fn find_intersections(pair : (&str, &str)) -> u32 {
    let intersections = pair.0.chars().find(|&c| pair.1.contains(c));
    return intersections.iter().map(|_v| score(*_v, false)).sum();
}

fn part2(file : &str) -> u32 {
    let ifile = File::open(file).unwrap();
    let ifile = BufReader::new(ifile);

    // Read all of the lines into a String vector
    let ls : Vec<String>= ifile.lines().map(|l| l.expect("Could not parse line")).collect();

    // Take 3 lines, and send themn to the intersection function, and then sum their scores
    let s : u32 = ls.chunks_exact(3).map(|chunk| find_intersections_2(&chunk[0], &chunk[1], &chunk[2])).sum();
    return s
}

fn find_intersections_2(c1 : &str, c2 : &str, c3 : &str ) -> u32 {
    let intersections = c1.chars().find(|&c| c2.contains(c) && c3.contains(c));
    return intersections.iter().map(|_v| score(*_v, false)).sum();
}

fn main() {
    let p1 = part1("/home/demarr/Projects/aoc_2022/day3/res/input.txt_p1");
    println!("Part 1: {}", p1);
    
    let p2 = part2("/home/demarr/Projects/aoc_2022/day3/res/input.txt_p2");
    println!("Part2: {}", p2);
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_score() {
        assert_eq!(1,score('a', false));
        assert_eq!(26,score('z', false));
        assert_eq!(27,score('A', false));
        assert_eq!(52,score('Z', false));
    }

    #[test]
    fn test_p1() {
        assert_eq!(157, part1("/home/demarr/Projects/aoc_2022/day3/res/test_input.txt_p1"));
    }

    #[test]
    fn test_p2() {
        assert_eq!(70, part2("/home/demarr/Projects/aoc_2022/day3/res/test_input.txt_p2"));
    }
}
