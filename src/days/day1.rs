pub fn part1(input: &str) -> String {
    let mut position = 50;
    let mut result = 0;

    for line in input.lines() {
        let count = line
            .get(1..)
            .ok_or(format!("Invalid input: {line}"))
            .and_then(|it| it.parse::<i32>().map_err(|it| it.to_string()))
            .unwrap();

        let rotation = count % 100;

        if line.starts_with("R") {
            position = (position + rotation) % 100;
        } else {
            position -= rotation;

            if position < 0 {
                position += 100;
            }
        }

        if position == 0 {
            result += 1;
        }
    }

    result.to_string()
}

pub fn part2(input: &str) -> String {
    let mut position = 50;
    let mut result = 0;

    for line in input.lines() {
        let count = line
            .get(1..)
            .ok_or(format!("Invalid input: {line}"))
            .and_then(|it| it.parse::<i32>().map_err(|it| it.to_string()))
            .unwrap();

        result += count / 100;

        let rotation = count % 100;

        if line.starts_with("R") {
            position += rotation;

            if position >= 100 {
                position %= 100;
                result += 1;
            }
        } else {
            let prev_position = position;

            position -= rotation;

            if position < 0 {
                position += 100;

                if prev_position > 0 {
                    result += 1;
                }
            } else if prev_position != 0 && position == 0 {
                result += 1;
            }
        }
    }

    result.to_string()
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
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
        "};

        assert_eq!(part1(input), "3");
    }

    #[test]
    fn test_part2() {
        let input = indoc! {"
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
        "};

        assert_eq!(part2(input), "6");
    }
}
