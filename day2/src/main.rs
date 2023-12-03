fn part_one(input: &str) {
    let red_count = 12;
    let green_count = 13;
    let blue_count = 14;

    let possible_games = input.lines().map(|line| {
        let mut line_parts = line.split(":");
        let game_id: u16 = line_parts.next().unwrap().replace("Game ", "").parse().unwrap();
        let mut game_parts = line_parts.next().unwrap().split(";");

        if game_parts.all(|game_part| {

            let mut possible = true;
            
           game_part.trim().split(", ").for_each(|pull| {

                let mut red: u16 = 0;
                let mut green: u16 = 0;
                let mut blue: u16 = 0;

                let pull_parts: Vec<&str> = pull.split(' ').collect();

                match pull_parts[1] {
                    "red" => red += pull_parts[0].parse::<u16>().unwrap(),
                    "green" => green += pull_parts[0].parse::<u16>().unwrap(),
                    "blue" => blue += pull_parts[0].parse::<u16>().unwrap(),
                    _ => println!("Unknown color")
                }
                
                if (red > red_count) || (green > green_count) || (blue > blue_count) {
                    println!("Game {} is not possible", game_id);
                    possible = false;
                }
            });

            return possible;
        })  {
            return game_id
        }
        
        return 0;
    }).sum::<u16>();

    println!("Sum of possible game id's {}", possible_games);
}

fn part_two(input: &str) {
    let power: u32 = input.lines().map(|line| {
        let mut line_parts = line.split(":");
        let game_parts = line_parts.nth(1).unwrap().split(";");
        let mut game_max_red: u16 = 0;
        let mut game_max_green: u16 = 0;
        let mut game_max_blue: u16 = 0;

        game_parts.for_each(|game_part| {

           game_part.trim().split(", ").for_each(|pull| {

                let mut red: u16 = 0;
                let mut green: u16 = 0;
                let mut blue: u16 = 0;

                let pull_parts: Vec<&str> = pull.split(' ').collect();

                match pull_parts[1] {
                    "red" => red += pull_parts[0].parse::<u16>().unwrap(),
                    "green" => green += pull_parts[0].parse::<u16>().unwrap(),
                    "blue" => blue += pull_parts[0].parse::<u16>().unwrap(),
                    _ => println!("Unknown color")
                }

                game_max_blue = game_max_blue.max(blue);
                game_max_green = game_max_green.max(green);
                game_max_red = game_max_red.max(red);
            });
        });

        return game_max_blue as u32 * game_max_green as u32 * game_max_red as u32;

    }).sum::<u32>();

    println!("Sum of the games power is {}", power);
}


fn main() {
    let input = include_str!("./input.txt");
    part_one(input);
    part_two(input);
}
