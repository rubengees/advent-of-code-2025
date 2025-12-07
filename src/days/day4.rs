use itertools::Itertools;
use std::collections::HashSet;

pub fn part1(input: &str) -> String {
    let positions = parse(input);

    let matching_positions = positions
        .iter()
        .filter(|it| can_be_removed(**it, &positions))
        .collect_vec();

    matching_positions.len().to_string()
}

fn parse(input: &str) -> HashSet<(i32, i32)> {
    let mut positions = HashSet::new();

    for (x, line) in input.lines().enumerate() {
        for (y, char) in line.chars().enumerate() {
            if char == '@' {
                positions.insert((x as i32, y as i32));
            }
        }
    }

    positions
}

fn can_be_removed((x, y): (i32, i32), positions: &HashSet<(i32, i32)>) -> bool {
    let count = (x - 1..=x + 1)
        .flat_map(|xx| (y - 1..=y + 1).map(move |yy| (xx, yy)))
        .filter(|it| positions.contains(it))
        .count();

    count <= 4
}

pub fn part2(input: &str) -> String {
    let mut positions = parse(input);
    let mut count = 0;

    loop {
        let matching_positions = positions
            .iter()
            .cloned()
            .filter(|it| can_be_removed(*it, &positions))
            .collect_vec();

        if matching_positions.len() > 0 {
            count += matching_positions.len();

            for x in matching_positions {
                positions.remove(&x);
            }
        } else {
            break;
        }
    }

    count.to_string()
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
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
        "};

        assert_eq!(part1(input), "13");
    }

    #[test]
    fn test_part2() {
        let input = indoc! {"
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
        "};

        assert_eq!(part2(input), "43");
    }
}
