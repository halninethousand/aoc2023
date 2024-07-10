fn main() {
    let input: Vec<String> = include_str!("../data/current.txt")
        .lines()
        .map(|x| x.to_string())
        .collect();

    // Process seeds
    let seeds: Vec<u64> = input[0]
        .split_whitespace()
        .skip(1)  // Skip the "seeds:" label
        .filter_map(|s| s.parse().ok())
        .collect();

    // Process maps
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

    // Don't forget to push the last map
    if !current_map.is_empty() {
        maps.push(current_map);
    }
    println!("{:?}", maps);
    // Process each map
    for map in maps {
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

        println!("Map: {}", map_name);
        println!("Mappings: {:?}", mappings);
    }
}
