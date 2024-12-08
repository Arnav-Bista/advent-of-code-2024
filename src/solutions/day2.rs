use std::sync::{Arc, Mutex};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub struct Day2 {
    data: Vec<Vec<i64>>,
    minimum: i64,
    maximum: i64,
}

impl Day2 {
    pub fn new(lines: Vec<String>) -> Self {
        let mut data: Vec<Vec<i64>> = Vec::with_capacity(lines.len());
        for line in lines {
            let line = line.split_whitespace();
            let numbers: Vec<i64> = line.map(|x| x.parse().unwrap()).collect();
            data.push(numbers);
        }

        Self {
            data,
            minimum: 1,
            maximum: 3,
        }
    }

    pub fn part_1(self) {
        let total: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
        self.data.par_iter().for_each(|numbers| {
            let mut inc = false;
            if numbers[0] - numbers[1] < 0 {
                inc = true;
            }
            for i in 1..numbers.len() {
                let diff = numbers[i - 1] - numbers[i];
                if !self.check_valid(diff, inc) {
                    return;
                }
            }

            let mut total = total.lock().unwrap();
            *total += 1;
        });
        println!("{}", *total.lock().unwrap());
    }

    fn check_valid(&self, diff: i64, inc: bool) -> bool {
        if diff.abs() > self.maximum || diff.abs() < self.minimum {
            return false;
        }
        if diff > 0 && inc || diff < 0 && !inc {
            return false;
        }
        true
    }

    pub fn part_2(self) {
        let total: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
        self.data.par_iter().for_each(|numbers| {
            let mut inc = false;
            if numbers[0] - numbers[1] < 0 {
                inc = true;
            }
            let mut prev_diff = 0;
            let mut dampner_used = 0;
            for i in 1..numbers.len() {
                // a - b prev
                // b - c diff
                // prev + diff
                // a - c = a - b + (b - c)
                let mut diff = numbers[i - 1] - numbers[i];
                if  dampner_used == 1 {
                    // diff = c - d
                    // prev = a - c
                    // want b - d
                    // diff = 
                } 
                if !self.check_valid(diff, inc) {
                    if dampner_used == 0 {
                        dampner_used = 1;
                        diff = prev_diff + diff;
                        if !self.check_valid(diff, inc) {
                            return;
                        }
                    }
                    else {
                        return;
                    }
                }
                prev_diff = diff;
            }

            let mut total = total.lock().unwrap();
            *total += 1;
        });
        println!("{}", *total.lock().unwrap());
    }
}
