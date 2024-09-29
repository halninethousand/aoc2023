use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let input: Vec<String> = include_str!("../../data/day4.txt")
        .lines()
        .map(|x| x.to_string())
        .collect();

    let mut part1_ans = 0;
    
    // 1

    for line in input.clone() {
        let card: Vec<&str> = line.split(':').collect();
        let mut num: Vec<&str> = card[1]
            .split('|')
            .collect();

        let left: Vec<&str> = num[0].split_whitespace().collect();
        let right: Vec<&str> = num[1].split_whitespace().collect();
        let set1: HashSet<&str> = left.iter().cloned().collect();
        let set2: HashSet<&str> = right.iter().cloned().collect();
    
        let common_elements: HashSet<_> = set1.intersection(&set2).cloned().collect();

        if !common_elements.is_empty() {
            let mut c: usize = 0;
            for (i, _item) in common_elements.iter().enumerate() {
                match i {
                    0 => c += 1,
                    _ => c *= 2,
                }
            }
            part1_ans += c;
        }
    }

    println!("Part 1: {part1_ans:?}");

    // 2 

    let mut copies: HashMap<usize, u64> = HashMap::new();

    for (i, line) in input.iter().enumerate() {
        let card_number = i + 1;
        *copies.entry(card_number).or_insert(0) += 1;

        let card: Vec<&str> = line.split(':').collect();
        let num: Vec<&str> = card[1].split('|').collect();
        let left: HashSet<&str> = num[0].split_whitespace().collect();
        let right: HashSet<&str> = num[1].split_whitespace().collect();

        let common_elements: HashSet<_> = left.intersection(&right).cloned().collect();
        let copy_count = common_elements.len();

        let current_card_count = *copies.get(&card_number).unwrap();

        for x in card_number + 1..=card_number + copy_count {
            if x <= input.len() {
                *copies.entry(x).or_insert(0) += current_card_count;
            }
        }
    }

    let total_cards: u64 = copies.values().sum();
    println!("Part 2: {}", total_cards);
    println!("Card copies: {:?}", copies);

}

