
fn part_one(input: &str) { 
    let sum = input.lines().map(|input| find_and_sum(input)).sum::<u32>();
    println!("Part one: {}", sum);
}

fn part_two(input: &str) {

    // Not very efficient, but it works. 
    // Replace the words with numbers, then find the first two in the same way we did in part one.
    let replacements = [
        ("zero", "z0o"),
        ("one", "o1e"),
        ("two", "t2w"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];

    let sum = input.lines().map(|l| {
        let line = replacements.iter().fold(l.to_string(), |acc, (from, to)| {
            acc.replace(from, to)
        });
        return find_and_sum(&line);
    }).sum::<u32>();

    println!("Part two: {}", sum);
}

fn find_and_sum(line: &str) -> u32 {
    let mut left_idx = 0;
    let mut right_idx = line.len().saturating_sub(1);

    let mut left_number = None;
    let mut right_number = None;

        while left_idx <= right_idx {
            left_number = line.chars().nth(left_idx).and_then(|c| c.to_digit(10));
            right_number = line.chars().nth(right_idx).and_then(|c| c.to_digit(10));

            if left_number.is_none() {
                left_idx += 1;
            }

            if right_number.is_none() {
                right_idx = right_idx.saturating_sub(1);
            }

            if left_number.is_some() && right_number.is_some() {
                break;
            }
        }

        if let (Some(left), Some(right)) = (left_number, right_number) {
            return format!("{}{}", left, right).parse::<u32>().unwrap();
        } else {
            panic!("Could not find two numbers in line: {}", line);
        }
}

fn main() {
    let input = include_str!("./input.txt");
    part_one(input);
    part_two(input);
}
