use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;

struct MoveInstruction {
    num_items: usize,    
    from: usize,
    to: usize
}

impl MoveInstruction {
    fn parse(s : &str) -> Self {
        let parts : Vec<&str> = s.split(" ").collect();
        MoveInstruction {num_items: parts[1].parse::<usize>().unwrap() , from: parts[3].parse::<usize>().unwrap() - 1, to: parts[5].parse::<usize>().unwrap() - 1}
    }

    fn print(&self) {
        println!("Number: {}, From: {}, To: {}", self.num_items, self.from, self.to)
    }
}


type Stack = VecDeque<char>;

struct WorkZone {
    stacks : Vec<Stack> 
}

impl WorkZone {
    fn new() -> Self {
        Self { stacks: Vec::new() }
    }

    fn add_boxes(&mut self, s: &str, debug : bool) {
        let enc_boxes : Vec<char> = s.chars().collect();
        let boxes : Vec<_> = enc_boxes.chunks(4).collect();

        while self.stacks.len() < boxes.len() {
            self.stacks.push(VecDeque::new());
        }

        if debug {println!("Box: {:?}", boxes);}

        for (pos, _) in boxes.iter().enumerate() {
            // Skip over blank boxes.
            if boxes[pos][0] == ' ' {
                continue;
            } else {
                if debug {println!("Adding {} to stack {}", boxes[pos][1], pos);}
                self.stacks[pos].push_front(boxes[pos][1]);
            }
        }
    }

    fn print(&mut self) {
        for i in 0..self.stacks.len() {
            println!("Stack {}: {:?}", i, self.stacks[i])
        }
    }

    fn lift_one(&mut self, instruction : &MoveInstruction ) {
        for _ in 0..instruction.num_items {
            let p = self.stacks[instruction.from].pop_back().unwrap();
            self.stacks[instruction.to].push_back(p);
        }
    }

    fn lift_many(&mut self, instruction : &MoveInstruction, debug : bool ) {
        let stack_size = self.stacks[instruction.from].len();
        if debug {
            println!("------------");
            println!("[from] stack size Before {} {:?}", stack_size, self.stacks[instruction.from]);
            println!("[to] stack size Before {} {:?}", self.stacks[instruction.to].len(), self.stacks[instruction.to]);
        }
        
        let p = self.stacks[instruction.from].drain(stack_size-instruction.num_items..stack_size).collect::<Vec<_>>();
        if debug {println!("[from] stack size After {} {:?}, Slice {} {:?}", self.stacks[instruction.from].len(), self.stacks[instruction.from], p.len(), p);}

        for i in (0..p.len()) {
            self.stacks[instruction.to].push_back(p[i]);
        }
        
        if debug {
            println!("[to] stack size After {} {:?}", self.stacks[instruction.to].len(), self.stacks[instruction.to]);
            println!("------------");
        }
    }

    fn whats_on_top(&mut self) -> String {
        let mut s : String = "".to_string();
        for n in 0..self.stacks.len() {
            s.push(*self.stacks[n].back().unwrap());
        }
        return s
    }
}

fn part1(file : &str) -> String {

    let mut instructions : Vec<MoveInstruction> = Vec::new();
    let mut work_zone : WorkZone = WorkZone::new();

    let f = File::open(file).unwrap();
    let reader = BufReader::new(f).lines().filter_map(|l| l.ok());

    for line in reader {
        if !line.is_empty()
        {
            //Check if an instruction or a box
            if line.chars().nth(0).unwrap().to_ascii_lowercase() == 'm' {
                instructions.push(MoveInstruction::parse(&line));
            } else {
                work_zone.add_boxes(&line, false);
            }
        }
    }

    for instruction in instructions.iter() {
        work_zone.lift_one(instruction);
    }

    work_zone.whats_on_top()
}

fn part2(file : &str) -> String {

    let mut instructions : Vec<MoveInstruction> = Vec::new();
    let mut work_zone : WorkZone = WorkZone::new();

    let f = File::open(file).unwrap();
    let reader = BufReader::new(f).lines().filter_map(|l| l.ok());

    for line in reader {
        if !line.is_empty()
        {
            //Check if an instruction or a box
            if line.chars().nth(0).unwrap().to_ascii_lowercase() == 'm' {
                instructions.push(MoveInstruction::parse(&line));
            } else {
                work_zone.add_boxes(&line, false);
            }
        }
    }

    for instruction in instructions.iter() {
        work_zone.lift_many(instruction, false);
    }

    work_zone.whats_on_top()
}

fn main() {
    println!("Part1: {}", part1("/home/demarr/Projects/aoc_2022/day5/res/input_p1.txt"));
    println!("Part2: {}", part2("/home/demarr/Projects/aoc_2022/day5/res/input_p2.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(2, 2);
    }

}

