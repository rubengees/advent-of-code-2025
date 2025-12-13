mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

pub fn run_day(day: i32, input: &str) -> Option<(String, String)> {
    match day {
        1 => Some(day1::run(input)),
        2 => Some(day2::run(input)),
        3 => Some(day3::run(input)),
        4 => Some(day4::run(input)),
        5 => Some(day5::run(input)),
        6 => Some(day6::run(input)),
        _ => None,
    }
}
