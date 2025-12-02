use crate::days::run_day;
use std::{env, fs, process};

mod days;

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let day = args
        .get(1)
        .ok_or("Please provide a day number as an argument")?
        .parse::<i32>()
        .map_err(|_| "Day argument must be a valid number")?;

    let contents =
        fs::read_to_string("input.txt").map_err(|e| format!("Failed to read input.txt: {e}"))?;

    let contents = contents.trim();

    let (part1, part2) =
        run_day(day, contents).ok_or_else(|| format!("Day {day} is not implemented yet"))?;

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}
