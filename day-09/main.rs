use std::collections::BTreeMap;
use std::io::{self, BufRead};

fn puzzle_1(nums: &Vec<u64>, length: usize) -> Option<u64> {
    // use map as we might have duplicate numbers
    let mut section = BTreeMap::new();

    // populate from preamble
    for ii in 0..length {
        *section.entry(nums[ii]).or_insert(0) += 1;
    }

    // process the rest
    for ii in length..nums.len() {
        let n = nums[ii];

        if !section.keys().any(|a| section.contains_key(&(n - a))) {
            return Some(n);
        }

        match section.entry(nums[ii - length]).or_insert(1) {
            e if *e == 1 => {
                section.remove(&(nums[ii - length]));
            }
            e => *e -= 1,
        }

        *section.entry(nums[ii]).or_insert(0) += 1;
    }

    None
}

fn puzzle_2(nums: &Vec<u64>, target: u64) -> Option<u64> {
    let mut sum = 0;
    let mut from = 0;
    let mut to = 0;

    loop {
        match sum {
            s if s == target => {
                let min = nums[from..to].iter().min()?;
                let max = nums[from..to].iter().max()?;
                return Some(min + max);
            }
            s if s < target && to == nums.len() => return None,
            s if s < target => {
                sum += nums[to];
                to += 1;
            }
            s if s > target && from == nums.len() => return None,
            s if s > target => {
                sum -= nums[from];
                from += 1;
            }
            _ => {}
        }
    }
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

    let preamble_length = 25;
    let invalid = puzzle_1(&nums, preamble_length).expect("invalid entry not found");
    println!("puzzle #1 = {:?}", invalid);
    println!("puzzle #2 = {:?}", puzzle_2(&nums, invalid));
}
