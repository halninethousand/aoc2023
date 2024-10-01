fn main() {
    let data: String = include_str!("../../data/day5.txt").to_owned();
    let sections = parse_data(&data);
    let part1 = part1(&sections);

    println!("Part1 lowest location seed: {part1}");
}

fn parse_data(data: &str) -> Vec<Vec<usize>> {

    let ret = data.split("\r\n\r\n")
        .map(|x| x.split_once(':').unwrap().1.replace('\n', " ").split_whitespace()
            .map(|z| z.parse::<usize>().unwrap()).collect::<Vec<_>>()).collect();
    ret
}

fn part1(sections: &Vec<Vec<usize>>) -> usize {
    let mut seeds = sections[0].clone().to_owned();
    for section in sections[1..].iter() {
        'seed: for seed in seeds.iter_mut() {
            for chunk in section.chunks(3) {
                if let &[dest_start, source_start, map_range] = chunk {
                    if (source_start <= *seed) && ((source_start + map_range) >= *seed) {
                        *seed = dest_start + *seed - source_start;
                        continue 'seed
                    }
                }
            }
        }
    }
    *seeds.iter().min().unwrap()
}
