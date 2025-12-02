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
            position = position - rotation;

            if position < 0 {
                position = 100 + position;
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
            position = position + rotation;

            if position >= 100 {
                position = position % 100;
                result += 1;
            }
        } else {
            let prev_position = position;

            position = position - rotation;

            if position < 0 {
                position = 100 + position;

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

    #[test]
    fn test_part1() {
        let input = "L68\n\
                           L30\n\
                           R48\n\
                           L5\n\
                           R60\n\
                           L55\n\
                           L1\n\
                           L99\n\
                           R14\n\
                           L82";

        assert_eq!(part1(input), "3");
    }

    #[test]
    fn test_part2_1() {
        let input = "L68\n\
                           L30\n\
                           R48\n\
                           L5\n\
                           R60\n\
                           L55\n\
                           L1\n\
                           L99\n\
                           R14\n\
                           L82";

        assert_eq!(part2(input), "6");
    }
}
