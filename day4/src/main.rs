use std::collections::{HashSet, HashMap};

#[derive(Debug)]
struct ScratchCard {
    winning_numbers: HashSet<usize>,
    scratch_numbers: HashSet<usize>,
}

fn get_scratch_cards(input: &str) -> Vec<ScratchCard> {
    let mut scratch_cards: Vec<ScratchCard> = Vec::new();
    for line in input.lines() {
        let mut parts = line.split(":");
        let _id = parts.next().unwrap().replace("Card ", "").trim().parse::<usize>().unwrap();

        let mut numbers = parts.next().unwrap().trim().split(" | ");

        let winning_numbers = numbers
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .filter(|x| x.to_string() != "")
            .map(|x| x.trim().parse::<usize>()
            .unwrap())
            .collect::<HashSet<usize>>();
        
        let scratch_numbers = numbers
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .filter(|x| x.to_string() != "")
            .map(|x| x.trim().parse::<usize>()
            .unwrap())
            .collect::<HashSet<usize>>();

        scratch_cards.push(ScratchCard { winning_numbers, scratch_numbers });
    }

    scratch_cards
}

fn part_one(input: &str) {
    let scratch_cards = get_scratch_cards(input);
    
    let mut score = 0;
    for scratch_card in scratch_cards {

        let mut scratch_card_score = 0;

        for winning_number in scratch_card.winning_numbers {
            if scratch_card.scratch_numbers.contains(&winning_number) {
                if scratch_card_score == 0 {
                    scratch_card_score = 1;
                } else {
                    scratch_card_score *= 2;
                }
            }
        }

        score += scratch_card_score;

    }

    println!("Part One: {}", score);

}

fn part_two(input: &str) {
    let scratch_cards = get_scratch_cards(input);
    let mut card_counts: HashMap<usize, usize> = HashMap::new();

    for (i, card) in scratch_cards.iter().enumerate() {
        if !card_counts.contains_key(&i) {
            card_counts.insert(i, 1);
        }

        let winning_count = card.winning_numbers
            .iter()
            .filter(|x| card.scratch_numbers.contains(x))
            .count();

        for j in i+1..i+winning_count+1 {
            if !card_counts.contains_key(&j) {
                card_counts.insert(j, 1);
            }
            card_counts.insert(j, card_counts.get(&j).unwrap() + card_counts.get(&i).unwrap());
        };
    }

    let sum = card_counts.iter().fold(0, |acc, (_, v)| acc + v);
    println!("Part Two: {}", sum);
}


fn main() {
    let input = include_str!("input.txt");
    part_one(input);
    part_two(input);
}
