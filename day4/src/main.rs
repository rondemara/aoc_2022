use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn fully_contained(v1 : Vec<u32>, v2 : Vec<u32>) -> bool {
    return (v1[0] <= v2[0] && v1[1] >= v2[1]) ||(v2[0] <= v1[0] && v2[1] >= v1[1]);
}

fn part1(file : &str) -> u32 {
    let ifile = File::open(file).unwrap();
    let ifile = BufReader::new(ifile);
    let mut sum = 0;

    for line in ifile.lines() {
        let row = line.unwrap();
        let pair_set = row.split(",").collect::<Vec<&str>>();
        let set1 :Vec<u32> = pair_set[0].split("-").map(|x| x.parse::<u32>().unwrap()).collect();
        let set2 :Vec<u32> = pair_set[1].split("-").map(|x| x.parse::<u32>().unwrap()).collect();
        if fully_contained(set1,set2) {sum+=1;}
    }
    return sum
}

fn part2(file : &str) -> u32 {
    let ifile = File::open(file).unwrap();
    let ifile = BufReader::new(ifile);
    let mut sum = 0;

    for line in ifile.lines() {
        let row = line.unwrap();
        let pair_set = row.split(",").collect::<Vec<&str>>();
        let set1 :Vec<u32> = pair_set[0].split("-").map(|x| x.parse::<u32>().unwrap()).collect();
        let set2 :Vec<u32> = pair_set[1].split("-").map(|x| x.parse::<u32>().unwrap()).collect();
        let a = (set1[0]..=set1[1]).collect::<HashSet<_>>();
        let b = (set2[0]..=set2[1]).collect::<HashSet<_>>();
        let c = a.intersection(&b).collect::<HashSet<_>>(); 
        if c.len() > 0 {sum+=1;}
    }
    return sum
}

fn main() {
    println!("Part1: {}", part1("/home/demarr/Projects/aoc_2022/day4/res/input_p1.txt"));
    println!("Part2: {}", part2("/home/demarr/Projects/aoc_2022/day4/res/input_p2.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(2, part1("/home/demarr/Projects/aoc_2022/day4/res/test_p1.txt"));
    }

    #[test]
    fn test_p2() {
        assert_eq!(4, part2("/home/demarr/Projects/aoc_2022/day4/res/test_p2.txt"));
    }
}

