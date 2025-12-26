use itertools::Itertools;

pub fn part1(input: &str) -> String {
    let polygon = parse(input);

    let max_area = polygon
        .iter()
        .combinations(2)
        .map(|combination| {
            let a = *combination[0];
            let b = *combination[1];

            (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)
        })
        .max();

    max_area.unwrap().to_string()
}

type Point = (u64, u64);

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();

            (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap())
        })
        .collect_vec()
}

pub fn part2(input: &str) -> String {
    let polygon = parse(input);

    let edges = polygon
        .iter()
        .tuple_windows()
        .chain([(polygon.last().unwrap(), polygon.first().unwrap())])
        .collect_vec();

    let max_area = polygon
        .iter()
        .combinations(2)
        .filter_map(|combination| {
            let (ax, ay) = *combination[0];
            let (bx, by) = *combination[1];

            let x_min = ax.min(bx);
            let x_max = ax.max(bx);
            let y_min = ay.min(by);
            let y_max = ay.max(by);

            // Bounding box check if any edge overlaps with our rect. If yes the rect is not in the
            // polygon. Edge cases exist that lead to false positives but works with the input.
            let has_overlap = edges.iter().any(|(a, b)| {
                let edge_x_min = a.0.min(b.0);
                let edge_x_max = a.0.max(b.0);
                let edge_y_min = a.1.min(b.1);
                let edge_y_max = a.1.max(b.1);

                edge_x_min < x_max
                    && edge_y_min < y_max
                    && edge_x_max > x_min
                    && edge_y_max > y_min
            });

            if !has_overlap {
                Some((x_max - x_min + 1) * (y_max - y_min + 1))
            } else {
                None
            }
        })
        .max();

    max_area.unwrap().to_string()
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
            7,1
            11,1
            11,7
            9,7
            9,5
            2,5
            2,3
            7,3
        "};

        assert_eq!(part1(input), "50");
    }

    #[test]
    fn test_part2() {
        let input = indoc! {"
            7,1
            11,1
            11,7
            9,7
            9,5
            2,5
            2,3
            7,3
        "};

        assert_eq!(part2(input), "24");
    }
}
