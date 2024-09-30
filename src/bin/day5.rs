use std::collections::HashMap;

fn main() {
    let input: Vec<String> = include_str!("../../data/day5.txt")
        .lines()
        .map(|x| x.to_string())
        .collect();
    
    let seeds: Vec<u64> = input[0]
        .split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .collect();
    
    let mut current_map = Vec::new();
    let mut maps = Vec::new();

    for line in &input[1..] {
        if line.is_empty() {
            if !current_map.is_empty() {
                maps.push(current_map);
                current_map = Vec::new();
            }
        } else {
            current_map.push(line);
        }
    }

    if !current_map.is_empty() {
        maps.push(current_map);
    }

    println!("Seeds: {:?}", seeds);
 
    let processed_maps: Vec<HashMap<u64, u64>> = maps.iter().map(|map| {
        let map_name = map[0].trim_end_matches(" map:");
        let mappings: Vec<(u64, u64, u64)> = map[1..]
            .iter()
            .map(|line| {
                let nums: Vec<u64> = line
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                (nums[0], nums[1], nums[2])
            })
            .collect();
        
        let mut hmap: HashMap<u64, u64> = HashMap::new();
        
        for &(dest_start, source_start, length) in &mappings {
            for i in 0..length {
                hmap.insert(source_start + i, dest_start + i);
            }
        }

        fill_default_mappings(&mut hmap);
        
        println!("Map: {}", map_name);
        println!("Mappings: {:?}", mappings);
        
        hmap

    }).collect();

    let final_locations: Vec<u64> = seeds.iter().map(|&seed| {
        processed_maps.iter().fold(seed, |value, map| {
            *map.get(&value).unwrap_or(&value)
        })
    }).collect();

    println!("Final locations: {:?}", final_locations);
    
    if let Some(&lowest_location) = final_locations.iter().min() {
        println!("Lowest location number: {}", lowest_location);
    } else {
        println!("No valid locations found.");
    }
}

fn fill_default_mappings(map: &mut HashMap<u64, u64>) {
    let max_key = *map.keys().max().unwrap_or(&0);
    for i in 0..=max_key {
        map.entry(i).or_insert(i);
    }
}
