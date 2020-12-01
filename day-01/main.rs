use std::io::{self, BufRead};

const TARGET: u64 = 2020;

fn puzzle_1(nums: &Vec<u64>) -> Option<u64> {
    for ii in 0..(nums.len() - 1) {
        for jj in (ii + 1)..nums.len() {
            if nums[ii] + nums[jj] == TARGET {
                return Some(nums[ii] * nums[jj]);
            }
        }
    }

    None
}

fn puzzle_2(nums: &Vec<u64>) -> Option<u64> {
    for ii in 0..(nums.len() - 2) {
        for jj in (ii + 1)..(nums.len() - 1) {
            for kk in (jj + 1)..nums.len() {
                if nums[ii] + nums[jj] + nums[kk] == TARGET {
                    return Some(nums[ii] * nums[jj] * nums[kk]);
                }
            }
        }
    }

    None
}

fn main() {
    let stdin = io::stdin();

    let nums: Vec<_> = stdin
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|l| l.parse::<u64>())
        .map(Result::unwrap)
        .collect();

    println!("puzzle #1: {:?}", puzzle_1(&nums));
    println!("puzzle #2: {:?}", puzzle_2(&nums));
}
