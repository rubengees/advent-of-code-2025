use itertools::Itertools;
use std::ops::{Range, RangeInclusive};

pub fn part1(input: &str) -> String {
    let (raw_ranges, raw_ingredients) = input.split_once("\n\n").unwrap();

    let ranges = raw_ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap();

            start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap()
        })
        .collect_vec();

    raw_ingredients
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .filter(|ingredient| ranges.iter().any(|range| range.contains(ingredient)))
        .count()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let (raw_ranges, _) = input.split_once("\n\n").unwrap();

    let ranges: Vec<RangeInclusive<u64>> = raw_ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap();

            start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap()
        })
        .collect_vec();

    #[allow(clippy::single_range_in_vec_init)]
    let ranges_without_overlap = ranges
        .iter()
        .enumerate()
        .flat_map(|(index, range)| {
            let init: Vec<Range<u64>> = vec![*range.start()..range.end() + 1];

            ranges
                .iter()
                .skip(index + 1)
                .fold(init, |current_ranges, other_range| {
                    let other_start = *other_range.start();
                    let other_end = *other_range.end();

                    current_ranges
                        .iter()
                        .flat_map(|current_range| {
                            let current_start = current_range.start;
                            let current_end = current_range.end - 1;

                            let result = if current_start > other_start && current_end < other_end {
                                // Current range is included the other range completely.
                                vec![]
                            } else if current_start <= other_start && current_end >= other_end {
                                // Current range completes the other range completely.
                                vec![current_start..other_start, (other_end + 1)..current_end + 1]
                            } else if current_start >= other_start && current_start <= other_end {
                                // Current range includes part of the other range (to the left).
                                vec![other_end + 1..current_end + 1]
                            } else if current_end >= other_start && current_end < other_end {
                                // Current range includes part of the other range (to the left).
                                vec![current_start..other_start]
                            } else {
                                vec![current_range.clone()]
                            };

                            let filtered_result =
                                result.into_iter().filter(|x| !x.is_empty()).collect_vec();

                            println!("{filtered_result:?}");

                            filtered_result
                        })
                        .collect_vec()
                })
        })
        .collect_vec();

    ranges_without_overlap
        .iter()
        .map(|range| range.end - range.start)
        .sum::<u64>()
        .to_string()
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
            3-5
            10-14
            16-20
            12-18

            1
            5
            8
            11
            17
            32
        "};

        assert_eq!(part1(input), "3");
    }

    #[test]
    fn test_part2() {
        let input = indoc! {"
            3-5
            10-14
            16-20
            12-18

            1
            5
            8
            11
            17
            32
        "};

        assert_eq!(part2(input), "14");
    }

    #[test]
    fn test_part2_3() {
        let input = indoc! {"
            3-5
            3-5

            1
        "};

        assert_eq!(part2(input), "3");
    }
}
