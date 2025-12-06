#!/usr/bin/env bash
set -eou pipefail

if [ -z "$1" ]; then
    echo "Usage: $0 <day_number>"
    exit 1
fi

DAY=$1

# Validate that the day is a number between 1 and 25
if ! [[ "$DAY" =~ ^[0-9]+$ ]] || [ "$DAY" -lt 1 ] ; then
    echo "Error: Day must be a number between 1 and 25"
    exit 1
fi

DAY_FILE="src/days/day${DAY}.rs"
MOD_FILE="src/days/mod.rs"

if [ -f "$DAY_FILE" ]; then
    echo "Error: $DAY_FILE already exists"
    exit 1
fi

cat > "$DAY_FILE" << 'EOF'
pub fn part1(input: &str) -> String {
    "TODO".to_string()
}

pub fn part2(input: &str) -> String {
    "TODO".to_string()
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
            TODO
        "};

        assert_eq!(part1(input), "TODO");
    }

    #[test]
    fn test_part2() {
        let input = indoc! {"
            TODO
        "};

        assert_eq!(part2(input), "TODO");
    }
}
EOF

echo "Created $DAY_FILE"

if ! grep -q "mod day${DAY};" "$MOD_FILE"; then
    sed -i "/^mod day[0-9]*;/{
        :a
        n
        /^mod day[0-9]*;/ba
        i mod day${DAY};
    }" "$MOD_FILE"

    if ! grep -q "mod day${DAY};" "$MOD_FILE"; then
        sed -i "1i mod day${DAY};" "$MOD_FILE"
    fi
fi

if ! grep -q "^[[:space:]]*${DAY} =>" "$MOD_FILE"; then
    sed -i "s/\([[:space:]]*\)_ => None,/\1${DAY} => Some(day${DAY}::run(input)),\n\1_ => None,/" "$MOD_FILE"
fi

echo "Updated $MOD_FILE"
echo "Day $DAY has been added successfully!"

