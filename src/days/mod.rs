mod day1;
mod day2;

pub fn run_day(day: i32, input: &str) -> Option<(String, String)> {
    match day {
        1 => Some(day1::run(input)),
        2 => Some(day2::run(input)),
        _ => None,
    }
}
