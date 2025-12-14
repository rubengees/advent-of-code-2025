use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> String {
    let lines = input.lines().collect_vec();
    let board = parse_board(&lines);

    count_splits(&board).to_string()
}

struct Board {
    height: usize,
    start: (usize, usize),
    splitters: HashSet<(usize, usize)>,
}

fn parse_board(lines: &[&str]) -> Board {
    let mut start = (0, 0);
    let mut splitters = HashSet::new();

    for (line_index, line) in lines.iter().enumerate() {
        for (char_index, char) in line.chars().enumerate() {
            if char == 'S' {
                start = (char_index, line_index);
            } else if char == '^' {
                splitters.insert((char_index, line_index));
            }
        }
    }

    Board {
        height: lines.len(),
        start,
        splitters,
    }
}

fn count_splits(board: &Board) -> u64 {
    let mut beams = HashSet::from([board.start]);
    let mut splits = 0;

    for _ in 0..board.height {
        beams = beams
            .iter()
            .flat_map(|beam| {
                let (x, y) = *beam;
                let next_beam = (x, y + 1);

                if board.splitters.contains(&next_beam) {
                    splits += 1;

                    vec![(x - 1, y + 1), (x + 1, y + 1)]
                } else {
                    vec![next_beam]
                }
            })
            .collect();
    }

    splits
}

pub fn part2(input: &str) -> String {
    let lines = input.lines().collect_vec();
    let board = parse_board(&lines);

    count_timelines(&board, board.start, &mut HashMap::new()).to_string()
}

fn count_timelines(
    board: &Board,
    beam: (usize, usize),
    cache: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    let (x, y) = beam;

    if y == board.height {
        return 1;
    }

    let next_beam = (x, y + 1);

    if board.splitters.contains(&next_beam) {
        count_timelines_cached(board, (x - 1, y + 1), cache)
            + count_timelines_cached(board, (x + 1, y + 1), cache)
    } else {
        count_timelines_cached(board, next_beam, cache)
    }
}

fn count_timelines_cached(
    board: &Board,
    next_beam: (usize, usize),
    cache: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if let Some(count) = cache.get(&next_beam) {
        *count
    } else {
        let count = count_timelines(board, next_beam, cache);
        cache.insert(next_beam, count);
        count
    }
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
            .......S.......
            ...............
            .......^.......
            ...............
            ......^.^......
            ...............
            .....^.^.^.....
            ...............
            ....^.^...^....
            ...............
            ...^.^...^.^...
            ...............
            ..^...^.....^..
            ...............
            .^.^.^.^.^...^.
            ...............
        "};

        assert_eq!(part1(input), "21");
    }

    #[test]
    fn test_part2() {
        let input = indoc! {"
            .......S.......
            ...............
            .......^.......
            ...............
            ......^.^......
            ...............
            .....^.^.^.....
            ...............
            ....^.^...^....
            ...............
            ...^.^...^.^...
            ...............
            ..^...^.....^..
            ...............
            .^.^.^.^.^...^.
            ...............
        "};

        assert_eq!(part2(input), "40");
    }
}
