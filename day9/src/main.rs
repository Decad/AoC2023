fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.lines()
    .map(|line| 
        line.split_ascii_whitespace()
        .map(|n| n.parse::<i32>().unwrap() )
        .collect()
    )
    .collect()
}

fn get_differences(input: Vec<i32>) -> Vec<i32> {
    // get the differences between each number
    let mut differences = Vec::new();
    for i in 0..input.len() - 1 {
        differences.push(input[i + 1] - input[i]);
    }
    differences
}

fn get_layers(histories: Vec<Vec<i32>>) -> Vec<Vec<Vec<i32>>> {
    return histories.iter().map(|history| {
        let mut layers = Vec::new();

        let mut differences = get_differences(history.clone());
        layers.push(differences.clone());

        loop {
            if differences.iter().all(|n| n == &(0 as i32)) {
                break;
            }

            differences = get_differences(differences.clone());
            layers.push(differences.clone());
        }
        
        layers
    })
    .collect();
}

fn part_one(input: &str) {
    let histories = parse_input(input);

    let growth_predictions: Vec<i32> = get_layers(histories.clone())
    .iter()
    .map(|history_layers| {
        return history_layers.iter().fold(0, |acc, layer| acc + layer.last().unwrap());
    })
    .collect();

    let sum = histories.iter().zip(growth_predictions)
        .fold(0, |acc, (history, prediction)| acc + history.last().unwrap() + prediction);

    println!("Part One: {:?}", sum);
    
}

fn part_two(input: &str) {
    let histories = parse_input(input);

    let growth_predictions: Vec<i32> = get_layers(histories.clone())
    .iter()
    .map(|history_layers| {
        return history_layers.iter().rev().fold(0, |acc, layer| return layer.first().unwrap() - acc);
    })
    .collect();

    let sum = histories.iter().zip(growth_predictions)
        .fold(0, |acc, (history, prediction)| acc + history.first().unwrap() - prediction);

    println!("Part Two: {:?}", sum);
}

fn main() {
    let input = include_str!("input.txt");

    part_one(input);
    part_two(input);
}
