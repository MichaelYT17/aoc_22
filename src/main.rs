use std::fs::File;
use std::io::{self, BufRead};
use std::collections::BinaryHeap;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut max:i32 = 0;
    let mut counter: i32 = 0;

    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();
    let mut top3: BinaryHeap<i32> = BinaryHeap::new();

    for line_result in lines {
        let line: String = line_result?;
        if line == "" {
            top3.push(counter * -1);
            if top3.len() > 3 {
                top3.pop();
            }
            counter = 0;
            continue;
        }
        let value = line.parse::<i32>()?;
        counter += value;
        if counter > max {
            max = counter
        }
    }

    let mut sum: i32 = 0;
    while top3.len() > 0 {
        sum += top3.pop().unwrap() * -1;
    }
    println!("{}", sum);
    return Ok(());
}
