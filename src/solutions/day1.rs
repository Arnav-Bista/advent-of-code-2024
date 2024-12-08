use std::collections::{BinaryHeap, HashMap};

pub struct Day1 {
    left: BinaryHeap<u64>,
    right: BinaryHeap<u64>,
}

impl Day1 {
    pub fn new(lines: Vec<String>) -> Self {
        let mut left = BinaryHeap::new();
        let mut right = BinaryHeap::new();
        for line in lines {
            let mut line = line.split_whitespace();
            left.push(line.next().unwrap().parse().unwrap());
            right.push(line.next().unwrap().parse().unwrap());
        }

        Self { left, right }
    }
    pub fn part_1(mut self) {
        let mut total = 0;
        while self.left.len() != 0 {
            total += self.left.pop().unwrap().abs_diff(self.right.pop().unwrap())
        }
        println!("{}", total);
    }

    pub fn part_2(self) {
        let mut right_freq: HashMap<u64, u64> = HashMap::new();
        self.right.iter().for_each(|r| {
            right_freq.insert(*r, right_freq.get(r).unwrap_or(&0) + 1);
        });

        let total = self.left
            .iter()
            .fold(0, |acc, x| acc + right_freq.get(x).unwrap_or(&0) * x);
        println!("{}", total);
    }
}
