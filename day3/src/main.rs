use std::collections::{HashSet, HashMap};

#[derive(Debug)]
struct EnginePart {
    id: String,
    line: usize,
    start: usize,
    end: usize,
    value: usize,
}

impl PartialEq for EnginePart {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

static SYMBOLS: [&'static str; 10] = ["*", "#", "/", "$", "+", "=", "-", "@", "&", "%"];

fn index_line(line: &str, line_number: usize) -> (Vec<EnginePart>, Vec<(char, usize)>) {
    let mut parts: Vec<EnginePart> = Vec::new();
    let mut symbols: Vec<(char, usize)> = Vec::new();

    let mut start: isize = -1;
    let mut end: usize = 0;

    for (i, c) in line.chars().enumerate() {
        if c.is_numeric() {
            if start == -1 {
                start = i as isize;
            }
            end = i;

        } else {
            if start != -1 {
                parts.push(EnginePart {
                    id: format!("{}-{}", line_number, start).to_string(),
                    line: line_number,
                    start: start as usize,
                    end,
                    value: line[start as usize..end + 1].parse().unwrap(),
                });
                start = -1;
                end = 0;
            }

            if SYMBOLS.contains(&c.to_string().as_str()) {
                symbols.push((c,i));
            }
        }
    }

    if start != -1 {
        parts.push(EnginePart {
            id: format!("{}-{}", line_number, start).to_string(),
            line: line_number,
            start: start as usize,
            end,
            value: line[start as usize..end + 1].parse().unwrap(),
        });
    }

    (parts, symbols)
}

fn is_adjacent(engine_part: &EnginePart, symbols: &HashSet<String>) -> bool {
    let directions: [(i32, i32); 8] = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1)
    ];

    for direction in directions.iter() {
        let y = engine_part.line as i32 + direction.0;
        let start_x = engine_part.start as i32 + direction.1;
        let end_x = engine_part.end as i32 + direction.1;

        if symbols.contains(&format!("{}-{}", y, start_x).to_string()) {
            return true;
        }

        if symbols.contains(&format!("{}-{}", y, end_x).to_string()) {
            return true;
        }
    }

    return false;
}

fn find_adjacent<'a>(engine_part: &'a EnginePart, gears: &mut HashMap<String, Vec<&'a EnginePart>>) {
    let directions: [(i32, i32); 8] = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1)
    ];

    for direction in directions.iter() {
        let y = engine_part.line as i32 + direction.0;
        let start_x = engine_part.start as i32 + direction.1;
        let end_x = engine_part.end as i32 + direction.1;

        if let Some(vec) = gears.get_mut(&format!("{}-{}", y, start_x).to_string()) {
            if !vec.contains(&engine_part) {
                vec.push(engine_part);
            }
        }
        
        if let Some(vec) = gears.get_mut(&format!("{}-{}", y, end_x).to_string()) {
            if !vec.contains(&engine_part) {
                vec.push(engine_part);
            }
        }
    }
}

fn part_one(input: &str) {
    let mut symbols: HashSet<String> = HashSet::new();
    let mut parts: Vec<EnginePart> = Vec::new();
    
    for (i, line) in input.lines().enumerate() {
        let (pts, syms) = index_line(line, i);

        parts.extend(pts);

        for symbol in syms {
            symbols.insert(format!("{}-{}", i, symbol.1).to_string());
        }
    }

    let sum = parts
        .iter()
        .filter(|p| is_adjacent(&p, &symbols))
        .map(|p| p.value)
        .sum::<usize>();

    println!("Part 1: {}", sum);
}

fn part_two(input: &str) {
    let mut gears: HashMap<String, Vec<&EnginePart>> = HashMap::new();
    let mut parts: Vec<EnginePart> = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let (pts, syms) = index_line(line, i);

        parts.extend(pts);

        for symbol in syms.iter().filter(|s| s.0 == '*') {
            gears.insert(format!("{}-{}", i, symbol.1).to_string(), Vec::new());
        }
    }

    parts
        .iter()
        .for_each(|p| find_adjacent(&p, &mut gears));

    let sum = gears.iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v[0].value * v[1].value)
        .sum::<usize>();

    println!("Part 2: {}", sum);
}

fn main() {
    let input = include_str!("./input.txt");
    part_one(input);
    part_two(input);
}
