fn parse_input_part_one(input: &str) -> Vec<(f64, f64)> {
    let mut lines = input.lines();

    let time_line = lines.next().unwrap().replace("Time:", "");
    let times = time_line
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<f64>().unwrap());

    let distance_line = lines.next().unwrap().replace("Distance:", "");
    let distances = distance_line
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<f64>().unwrap());

    return times.zip(distances).collect();
}

fn parse_input_part_two(input: &str) -> (f64, f64) {
    let mut lines = input.lines();

    let time_line = lines.next().unwrap().replace("Time:", "");
    let time = time_line
        .trim()
        .replace(" ", "")
        .parse::<f64>()
        .unwrap();

    let distance_line = lines.next().unwrap().replace("Distance:", "");
    let distance = distance_line
        .trim()
        .replace(" ", "")
        .parse::<f64>()
        .unwrap();

    return (time, distance);
}


fn part_one(input: &str) {
    let tds = parse_input_part_one(input);

    let margin = tds.iter()
        .map(|(race_time, record_distance)| {
            let discriminant = race_time.powi(2) - 4.0 * -1.0 * -(record_distance + 0.1);
            let min = (-race_time + (discriminant).sqrt()) / -2.0;
            let max = (-race_time - (discriminant).sqrt()) / -2.0;
            let min = min.ceil();
            let max = max.floor();
            return max - min + 1.0;
        })
        .fold(1.0, |acc, x| acc * x);

    println!("Part one: {} ", margin);
}

fn part_two(input: &str) {
    let (race_time, record_distance) = parse_input_part_two(input);

    let discriminant = race_time.powi(2) - 4.0 * -1.0 * -(record_distance + 0.1);
    let mut min = (-race_time + (discriminant).sqrt()) / -2.0;
    let mut max = (-race_time - (discriminant).sqrt()) / -2.0;
    min = min.ceil();
    max = max.floor();
    let ways = max - min + 1.0;

    println!("Part two: {}", ways);
}

fn main() {
    let input = include_str!("./input.txt");
    part_one(input);
    part_two(input);
}
