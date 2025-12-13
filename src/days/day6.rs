use itertools::Itertools;

pub fn part1(input: &str) -> String {
    let lines = input.lines().collect_vec();

    let numbers = parse_numbers(&lines[..lines.len() - 1]);
    let operators = parse_operators(lines.last().unwrap());

    let result = (0..operators.len()).map(|index| {
        let operator = operators[index];
        let vertical_numbers = numbers.iter().map(|number_line| number_line[index]);

        if operator == '+' {
            vertical_numbers.sum::<u64>()
        } else {
            vertical_numbers.product::<u64>()
        }
    });

    result.sum::<u64>().to_string()
}

fn parse_numbers(lines: &[&str]) -> Vec<Vec<u64>> {
    lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|part| part.parse::<u64>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn parse_operators(line: &str) -> Vec<char> {
    line.split_whitespace()
        .map(|line| line.parse::<char>().unwrap())
        .collect_vec()
}

pub fn part2(input: &str) -> String {
    let lines = input.lines().collect_vec();

    let numbers = parse_numbers_2(&lines[..lines.len() - 1]);
    let operations = parse_operators(lines.last().unwrap());

    let result = operations.iter().zip(numbers).map(|(operator, numbers)| {
        if *operator == '+' {
            numbers.iter().sum::<u64>()
        } else {
            numbers.iter().product::<u64>()
        }
    }).collect_vec();

    result.iter().sum::<u64>().to_string()
}

fn parse_numbers_2(lines: &[&str]) -> Vec<Vec<u64>> {
    let line_length = lines.iter().map(|line| line.len()).max().unwrap();

    let mut result: Vec<Vec<u64>> = vec![];
    let mut current: Vec<u64> = vec![];

    for i in 0..line_length {
        let number_string = lines
            .iter()
            .map(|line| line.chars().nth(i).unwrap_or(' '))
            .join("");

        let number = number_string.trim().parse::<u64>().ok();

        match number {
            Some(n) => {
                current.push(n);
            }
            None => {
                result.push(current);
                current = vec![]
            }
        }
    }

    if !current.is_empty() {
        result.push(current)
    }

    result
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
            123 328  51 64
             45 64  387 23
              6 98  215 314
            *   +   *   +
        "};

        assert_eq!(part1(input), "4277556");
    }

    #[test]
    fn test_part2() {
        let input = indoc! {"
            123 328  51 64
             45 64  387 23
              6 98  215 314
            *   +   *   +
        "};

        assert_eq!(part2(input), "3263827");
    }
}
