use itertools::Itertools;
use std::collections::HashSet;

pub fn part1(input: &str) -> String {
    let positions = parse(input);

    let distances = calculate_distances(&positions);

    let mut circuits = positions
        .iter()
        .map(|pos| HashSet::from([*pos]))
        .collect_vec();

    for item in distances.iter().take(positions.len()) {
        connect_circuit(&mut circuits, item)
    }

    circuits
        .iter()
        .map(|circuit| circuit.len())
        .sorted()
        .rev()
        .take(3)
        .product::<usize>()
        .to_string()
}

fn parse(input: &str) -> Vec<(u64, u64, u64)> {
    input
        .lines()
        .map(|line| {
            let split = line
                .splitn(3, ",")
                .map(|n| n.parse::<u64>().unwrap())
                .collect_vec();

            (split[0], split[1], split[2])
        })
        .collect_vec()
}

struct PositionDistance {
    distance: f64,
    start: (u64, u64, u64),
    end: (u64, u64, u64),
}

fn calculate_distances(positions: &[(u64, u64, u64)]) -> Vec<PositionDistance> {
    positions
        .iter()
        .combinations(2)
        .map(|permutation| {
            let start = *permutation[0];
            let end = *permutation[1];

            let dx = start.0 as f64 - end.0 as f64;
            let dy = start.1 as f64 - end.1 as f64;
            let dz = start.2 as f64 - end.2 as f64;

            let distance = (dx * dx + dy * dy + dz * dz).sqrt();

            PositionDistance {
                distance,
                start,
                end,
            }
        })
        .sorted_by(|a, b| f64::total_cmp(&a.distance, &b.distance))
        .collect_vec()
}

fn connect_circuit(circuits: &mut Vec<HashSet<(u64, u64, u64)>>, item: &PositionDistance) {
    let indices = circuits
        .iter()
        .positions(|circuit| circuit.contains(&item.start) || circuit.contains(&item.end))
        .sorted()
        .collect_vec();

    if indices.len() <= 1 {
        return;
    }

    for index in indices[1..].iter().rev() {
        let removed = circuits.swap_remove(*index);
        circuits[indices[0]].extend(removed);
    }
}

pub fn part2(input: &str) -> String {
    let positions = parse(input);

    let distances = calculate_distances(&positions);

    let mut circuits = positions
        .iter()
        .map(|pos| HashSet::from([*pos]))
        .collect_vec();

    for distance in distances {
        connect_circuit(&mut circuits, &distance);

        if circuits.len() == 1 {
            return (distance.start.0 * distance.end.0).to_string();
        }
    }

    panic!("Circuits not connected")
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
            162,817,812
            57,618,57
            906,360,560
            592,479,940
            352,342,300
            466,668,158
            542,29,236
            431,825,988
            739,650,466
            52,470,668
            216,146,977
            819,987,18
            117,168,530
            805,96,715
            346,949,466
            970,615,88
            941,993,340
            862,61,35
            984,92,344
            425,690,689
        "};

        // Example connects 10 circuits, real task 1000 times. This implementation uses lines.len() -> 20 in the example.
        assert_eq!(part1(input), "45");
    }

    #[test]
    fn test_part2() {
        let input = indoc! {"
            162,817,812
            57,618,57
            906,360,560
            592,479,940
            352,342,300
            466,668,158
            542,29,236
            431,825,988
            739,650,466
            52,470,668
            216,146,977
            819,987,18
            117,168,530
            805,96,715
            346,949,466
            970,615,88
            941,993,340
            862,61,35
            984,92,344
            425,690,689
        "};

        assert_eq!(part2(input), "25272");
    }
}
