fn main() {
    let input: Vec<&str> = include_str!("../../data/day6.txt").lines().collect();
    let times: Vec<usize> = input[0]
        .split_whitespace()
        .filter_map(|word| word.parse::<usize>().ok())
        .collect();

    let distances: Vec<usize> = input[1]
        .split_whitespace()
        .filter_map(|word| word.parse::<usize>().ok())
        .collect();

    println!("time: {times:?}\nduration: {distances:?}");

    let winners: Vec<usize> = times.iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| {
            (0..=time).filter(|&button| {
                let left = time - button;
                let traveled = left * button;
                traveled > distance
            }).count()
        })
        .collect();

    println!("{}", winners.iter().product::<usize>());

    // Part2
    let time: usize = input[0]
        .chars()
        .filter(|c| c.is_digit(10))  
        .collect::<String>()
        .parse::<usize>()
        .unwrap();


    let distance: usize = input[1]
        .chars()
        .filter(|c| c.is_digit(10))  
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    
    let winners = 
            (0..=time).filter(|&button| {
                let left = time - button;
                let traveled = left * button;
                traveled > distance
            }).count();

    println!("winners: {}", winners);
}
