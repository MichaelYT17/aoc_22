use std::fs::File;
use std::io::{self, BufRead};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut max: u32 = 0;
    let mut counter: u32 = 0;

    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    for line_result in lines {
        let line: String = line_result?;
        if line == "" {
            counter = 0;
            continue;
        }
        let value = line.parse::<u32>()?;
        counter += value;
        if counter > max {
            max = counter
        }
    }
    println!("{}", max);
    return Ok(());
}
