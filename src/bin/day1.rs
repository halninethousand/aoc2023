use std::collections::HashMap;

fn main() {
    let input: Vec<String> = include_str!("../../data/day1.txt")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let mut first = input.clone();
    let mut second = input.clone();

    find_first_last(&mut first);

    let digits = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let leet_map = HashMap::from([("one", "1e"), ("two", "2o"), ("three", "3e"), ("four", "4"), ("five", "5e"), ("six", "6"), ("seven", "7"), ("eight", "8t"), ("nine", "9e")]);

    for line in second.iter_mut() {
        let mut ret: Vec<(usize, &str)> = vec![];
        let match_idx = line.clone();
        for digit in &digits {
            let mut idcs: Vec<(usize, &str)> = match_idx.match_indices(digit).collect();
            if !idcs.is_empty() {
                idcs.sort();
                for item in idcs {
                    ret.push(item);
                }
            }
        }
        ret.sort();
        for item in ret.iter().enumerate() {
            let mut what = item.1.clone();
            *line = line.replace(&what, leet_map.get(what).unwrap());
        }
    }

    find_first_last(&mut second);
}

trait IsEmpty {
    fn is_empty(&self) -> bool;
}

trait Sort {
    fn sort(&mut self) -> &mut Self;
}

impl IsEmpty for (usize, &str) {
    fn is_empty(&self) -> bool {
        self.1.is_empty()
    }
}

impl Sort for Vec<(usize, &str)> {
    fn sort(&mut self) -> &mut Self {
        self.sort_by_key(|d| d.0);
        self 
    }
}

fn find_first_last(input: &mut Vec<String>) -> () {
    let mut sum: u32 = 0;
    for line in input.iter_mut() {
        if line.is_empty() {
            break;
        }
        let mut parts = line.chars().collect::<Vec<char>>();
        parts.retain(|part| part.is_digit(10));


        if parts.len() == 1 { 
            let first_last: u32 = (parts[0].to_string().as_str().to_owned() + parts[0].to_string().as_str()).parse().unwrap();
            sum += first_last;
        } else if parts.len() == 0 {
            continue
        } else {
            let first_last: u32 = (String::from(*parts.clone().first().unwrap()) + parts.clone().last().unwrap().to_string().as_str()).parse().unwrap();
            sum += first_last;
        }
    }
    println!("sum of calibration values = {:?}", sum); 
}
