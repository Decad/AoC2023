use std::{thread, sync::{Arc, Mutex}};


#[derive(Clone)]
#[derive(Debug)]
struct Conversion {
    source_start: usize,
    destination_start: usize,
    size: usize,
}

#[derive(Clone)]
#[derive(Debug)]
struct Map {
    from: String,
    to: String,
    conversions: Vec<Conversion>,
}

enum LineType {
    MapHeader,
    Map,
    None,
}

impl LineType {
    fn from(line: &str) -> LineType {
        if line.is_empty() {
            return LineType::None;
        }

        let first_char = line.chars().next().unwrap();
        match first_char.is_digit(10) {
            true => LineType::Map,
            false => LineType::MapHeader,
        }
    }
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<Map>) {
    let mut maps = Vec::new();

    let mut lines = input.lines();

    let seeds = lines.next()
        .unwrap()
        .replace("seeds: ", "")
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut current_map: Option<Map> = None;

    while let Some(line) = lines.next() {
        match LineType::from(line) {
            LineType::MapHeader => {
                if let Some(map) = current_map {
                    maps.push(map);
                }

                let modified_line = line.replace(" map:", "");
                let mut parts = modified_line.split("-to-");

                current_map = Some(Map {
                    from: parts.next().unwrap().trim().to_string(), 
                    to: parts.next().unwrap().trim().to_string(),
                    conversions: Vec::new(),
                });
            },
            LineType::Map => {
                let mut parts = line.split(" ");
                let destination_start = parts.next().unwrap().parse::<usize>().unwrap();
                let source_start = parts.next().unwrap().parse::<usize>().unwrap();
                let size = parts.next().unwrap().parse::<usize>().unwrap();

                current_map.as_mut().unwrap().conversions.push(Conversion {
                    source_start,
                    destination_start,
                    size,
                });
            },
            _ => continue
        }
    }

    if let Some(map) = current_map {
        maps.push(map);
    }

    (seeds, maps)
}

fn part_one(input: &str) {
    let (seeds, maps) = parse_input(input);
    let mut lowest_location = usize::max_value();

    for seed in seeds {
        let mut location = seed;

        for map in &maps {
            for conversion in &map.conversions {
                if location >= conversion.source_start && location < conversion.source_start + conversion.size {
                    location = conversion.destination_start + (location - conversion.source_start);
                    break;
                }
            }

        }
  
        if lowest_location > location {
            lowest_location = location;
        }

    }

    println!("Part one: lowest location is {}", lowest_location);
}

fn process_range(start: usize, size: usize, maps: &Vec<Map>) -> usize {
    let mut lowest_location = usize::max_value();
    for start in start..start + size {
        let mut location = start;
        for map in maps {
            for conversion in &map.conversions {
                if location >= conversion.source_start && location < conversion.source_start + conversion.size {
                    location = conversion.destination_start + (location - conversion.source_start);
                    break;
                }
            }
        }

        if lowest_location > location {
            lowest_location = location;
        }
    }

    lowest_location
}

// Bruteforce baby!!
// Could probably be optimized by taking the ranges and running them through the maps one at a time
// splitting the ranges as we go over any conversion boundaries
// at the end we will have a list of ranges and we can just find the smallest starting value
// BUT we can also just spawn a thread for each range and put the kettle on
fn part_two(input: &str) {
    let (seeds, maps) = parse_input(input);

    let seed_ranges = seeds.chunks(2);

    let maps_arc = Arc::new(maps);
    let results = Arc::new(Mutex::new(Vec::new()));

    thread::scope(|s| {
        for (i, seed_range) in seed_ranges.enumerate() {
            println!("Checking seed range {}", i);

            let maps_clone = Arc::clone(&maps_arc);
            let results_clone = Arc::clone(&results);

            s.spawn(move || {
                let best_in_range = process_range(seed_range[0], seed_range[1], &maps_clone);
                let mut results_guard = results_clone.lock().unwrap();
                results_guard.push(best_in_range);
            });
        }
    });

    let results_inner = Arc::try_unwrap(results).expect("Failed to unwrap Arc");
    let results_vec = results_inner.into_inner().expect("Failed to obtain inner value");

    let lowest_location = results_vec.iter().min().unwrap();

    println!("Part two: lowest location is {}", lowest_location);
}
fn main() {
    let input = include_str!("input.txt");
    part_one(input);
    part_two(input);
}
