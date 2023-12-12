use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Map {
    route: String,
    nodes: HashMap<String, (String, String)>,
    current_node: String,
    current_step: usize,
}

impl Map {
    fn steps_to_end(&mut self) -> usize {
        loop {
            if self.current_node == "ZZZ" {
                return self.current_step;
            }

            self.next();
        }
    }

    fn steps_to_cycle(&mut self, starting_point: String) -> usize {
        let mut iter_counts: Vec<usize> = Vec::new();
        let mut starting_step = 0;

        self.current_node = starting_point;

        loop {
            if self.current_node.ends_with("Z") {
                let steps_taken = self.current_step - starting_step;
                if iter_counts.contains(&(&steps_taken)) {
                    return steps_taken; 
                } else {
                    iter_counts.push(steps_taken);
                    starting_step = self.current_step;
                }
            }

            self.next();
        }
    }

    fn next(&mut self) {
        let (left, right) = self.nodes.get(&self.current_node).unwrap();
        let current_dir = self.route.chars().nth(self.current_step % self.route.len()).unwrap();

        self.current_node = match current_dir {
            'L' => left.to_string(),
            'R' => right.to_string(),
            _ => panic!("Unknown direction {}", current_dir),
        };
        self.current_step = self.current_step + 1;
    }

    fn start_nodes(&self) -> Vec<String> {
        let mut nodes = Vec::new();
        for (node, ..) in self.nodes.iter() {
            if node.ends_with("A") {
                nodes.push(node.to_string());
            }
        }
        nodes
    }
}

fn parse_input(input: &str) -> Map {
    let mut lines = input.lines();
    let route = lines.next().unwrap().to_string();

    // Skip empty line
    lines.next();

    let mut nodes = HashMap::new();
    for line in lines {
        let mut parts = line.split(" = ");
        let node = parts.next().unwrap().to_string();
        let left_right_part_str = parts.next().unwrap();
        let mut left_right_parts = left_right_part_str[1..left_right_part_str.len() - 1].split(", ");
        let left = left_right_parts.next().unwrap().to_string();
        let right = left_right_parts.next().unwrap().to_string();
        nodes.insert(node, (left, right));
    }

    Map { route, nodes, current_node: "AAA".to_string(), current_step: 0 }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn part_one(input: &str) {
    let mut map = parse_input(input);
    println!("Part one: {}", map.steps_to_end());
}

fn part_two(input: &str) {
    let map = parse_input(input);

    let start_nodes = map.start_nodes();

    let mut cycles = Vec::new();
    for start_node in start_nodes {
        let mut map = map.clone();
        cycles.push(map.steps_to_cycle(start_node));
    }

    let smallest_cycle = cycles.iter().fold(cycles[0], |acc, &x| lcm(acc, x));

    println!("Part two: {}", smallest_cycle);
}

fn main() {
    let input = include_str!("input.txt");
    part_one(input);
    part_two(input);
}
