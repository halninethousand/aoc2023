fn main() {
    let input = include_str!("../data/day1.txt");
    for line in input.lines() {
        let mut parts = line.chars().map(|s| s.is_digit(10));
        let mut mask = parts.collect::<Vec<_>>();
        let 
        eprintln!("parts = {:?}", parts);
    }
    eprintln!("input = {:?}", input);
}
