use crate::utils::input_aoc;
use std::error::Error;

struct IdRange {
    end: u64,
    pointer: u64,
}

impl IdRange {
    fn new(range: (u64, u64)) -> Self {
        IdRange {
            end: range.1,
            pointer: range.0,
        }
    }
}

impl Iterator for IdRange {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pointer > self.end {
            None
        } else {
            let current = self.pointer;
            self.pointer += 1;
            Some(current)
        }
    }
}

fn parse(input: &str) -> Result<Vec<(u64, u64)>, Box<dyn Error>> {
    let mut ranges = Vec::new();
    for pair in input.trim().split(',') {
        let (start, end) = pair.trim().split_once('-').ok_or("Invalid pair format")?;
        let start: u64 = start.parse()?;
        let end: u64 = end.parse()?;
        ranges.push((start, end));
    }
    Ok(ranges)
}

fn is_even_numeral_count(number: u64) -> bool {
    number.to_string().len() % 2 == 0
}

fn divide_number(number: u64) -> (u64, u64) {
    let s = number.to_string();
    let mid = s.len() / 2;
    let first = s[..mid].parse::<u64>().unwrap();
    let second = s[mid..].parse::<u64>().unwrap();
    (first, second)
}

pub async fn init() -> Result<(), Box<dyn Error>> {
    let input = input_aoc::get_http(2).await.unwrap();
    let mut total_invalid_sum: u64 = 0;

    for (start, end) in parse(&input)? {
        let range = IdRange::new((start, end));
        for id in range {
            if is_even_numeral_count(id) {
                let (first, second) = divide_number(id);
                if first == second {
                    total_invalid_sum += id;
                }
            }
        }
    }

    println!("Result 1: {}", total_invalid_sum);
    Ok(())
}
