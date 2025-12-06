use itertools::Itertools;

pub fn part1(input: &str) -> String {
    let total = input
        .lines()
        .map(|line| {
            let numbers = line
                .chars()
                .map(|char| char.to_digit(10).unwrap() as u64)
                .collect_vec();

            find_max(&numbers)
        })
        .sum::<u64>();

    total.to_string()
}

fn find_max(numbers: &[u64]) -> u64 {
    let search_window = &numbers[0..numbers.len() - 1];
    let (max_pos, max) = first_max(search_window);

    let second_search_window = &numbers[max_pos + 1..];
    let second_max = *second_search_window.iter().max().unwrap();

    max * 10 + second_max
}

fn first_max(numbers: &[u64]) -> (usize, u64) {
    numbers
        .iter()
        .enumerate()
        .fold((0, 0), |(cur_max_pos, cur_max), (next_pos, next)| {
            if *next > cur_max {
                (next_pos, *next)
            } else {
                (cur_max_pos, cur_max)
            }
        })
}

pub fn part2(input: &str) -> String {
    let total = input
        .lines()
        .map(|line| {
            let numbers = line
                .chars()
                .map(|char| char.to_digit(10).unwrap() as u64)
                .collect_vec();

            find_max_rec(&numbers, 0, 12)
        })
        .sum::<u64>();

    total.to_string()
}

fn find_max_rec(numbers: &[u64], start: usize, remaining: usize) -> u64 {
    if remaining == 0 {
        return 0;
    }

    let search_window = &numbers[start..numbers.len() - (remaining - 1)];
    let (max_pos, max) = first_max(search_window);

    let final_max = max * 10_u64.pow(remaining as u32 - 1);

    final_max + find_max_rec(numbers, start + max_pos + 1, remaining - 1)
}

pub fn run(input: &str) -> (String, String) {
    (part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {"
            987654321111111
            811111111111119
            234234234234278
            818181911112111
        "};

        assert_eq!(part1(input), "357");
    }

    #[test]
    fn test_part2() {
        let input = indoc! {"
            987654321111111
            811111111111119
            234234234234278
            818181911112111
        "};

        assert_eq!(part2(input), "3121910778619");
    }
}
