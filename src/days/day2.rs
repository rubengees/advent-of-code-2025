use itertools::Itertools;

pub fn part1(input: &str) -> String {
    let ranges = input.split(",");
    let mut result: i64 = 0;

    for range in ranges {
        let (raw_start, raw_end) = range.trim().split_once("-").unwrap();
        let start = raw_start.parse::<i64>().unwrap();
        let end = raw_end.parse::<i64>().unwrap();

        for i in start..=end {
            let i_str = i.to_string();
            let (left, right) = i_str.split_at(i_str.len() / 2);

            if left == right {
                result += i;
            }
        }
    }

    result.to_string()
}

pub fn part2(input: &str) -> String {
    let ranges = input.split(",");
    let mut result: i64 = 0;

    for range in ranges {
        let (raw_start, raw_end) = range.trim().split_once("-").unwrap();
        let start = raw_start.parse::<i64>().unwrap();
        let end = raw_end.parse::<i64>().unwrap();

        for i in start..=end {
            if is_invalid(&i.to_string()) {
                result += i;
            }
        }
    }

    result.to_string()
}

fn is_invalid(id: &str) -> bool {
    for i in 1..=id.len() / 2 {
        let char_chunks = id.chars().chunks(i);
        let all_equal = char_chunks
            .into_iter()
            .map(|it| it.collect::<String>())
            .all_equal();

        if all_equal {
            return true;
        }
    }

    false
}

pub fn run(input: &str) -> (String, String) {
    (part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
                           1698522-1698528,446443-446449,38593856-38593862,565653-565659,
                           824824821-824824827,2121212118-2121212124";

        assert_eq!(part1(input), "1227775554");
    }

    #[test]
    fn test_part2_1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
                           1698522-1698528,446443-446449,38593856-38593862,565653-565659,
                           824824821-824824827,2121212118-2121212124";

        assert_eq!(part2(input), "4174379265");
    }
}
