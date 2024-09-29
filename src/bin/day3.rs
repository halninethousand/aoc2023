use std::{vec};

#[derive(Clone, Debug)]
struct Point(usize, usize);

fn main() {
    let input: Vec<String> = include_str!("../../data/day3.txt")
        .lines()
        .map(|x| x.to_string())
        .collect();

    let mut matrix: Vec<Vec<char>> = vec![];

    for line in &input {
        let char3: Vec<char> = line.chars().collect::<Vec<_>>();
        matrix.push(char3);
    }

    let mut num_start: Option<Point> = None;
    let mut locations: Vec<(Point, Point)> = vec![];
    let mut remember: usize = 0;

    for (y, row) in input.iter().enumerate() {
        for (x, item) in row.chars().enumerate() {
            // set x so we can use it outside of this block after the for x loop
            remember = x; 
            if !item.is_numeric() {
                if num_start.is_some() {
                    locations.push((num_start.clone().unwrap(), Point{0:y, 1:x}));
                    num_start = None;
                }                    
            } else {
                if num_start.clone().is_none() {
                    num_start = Some(Point(y, x));
                }
            }
        }
        if num_start.is_some() {
            locations.push((num_start.clone().unwrap(), Point{0:y, 1:remember+1}));
            num_start = None;
        }
    }

    let mut result = vec![];
    for location in locations {
        if is_symbol_adjacent(matrix.clone(), location.0.clone(),location.1.clone()) {
            result.push(read_number(matrix.clone(), location.0, location.1));
        }
    }
    println!("Part1: {}", result.iter().copied().sum::<u32>());

    // Part2
    let mut result_part2 = vec![];

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if is_symbol(matrix[y][x]) {
                let adjacent_numbers = get_adjacent_numbers(&matrix, y, x);
                if adjacent_numbers.len() == 2 {
                    result_part2.push(adjacent_numbers.iter().product::<u32>());
                }
            }
        }
    }

    println!("Part2: {}", result_part2.iter().copied().sum::<u32>());
}


fn read_number(matrix: Vec<Vec<char>>, start: Point, end: Point) -> u32 {

    let num: String = matrix.clone()[start.0][start.1..end.1]
        .iter()
        .collect();
    
    num.parse::<u32>().unwrap()
}
fn is_symbol_adjacent(matrix: Vec<Vec<char>>, start: Point, end: Point) -> bool {
    let offsets: [isize; 3] = [-1, 0, 1];
    for offset_i in offsets {
        let st: i32 = start.1 as i32 -1;
        let nd: i32 = end.1 as i32 +1;
        for current_j in st..nd {

            let current_i = start.0 as isize + offset_i;
            if in_bounds(&matrix, current_i, current_j.try_into().unwrap()) && is_symbol(matrix[current_i as usize][current_j as usize]){
                return true
            }
        }
    }
    
    false
}

fn in_bounds(matrix: &[Vec<char>], x: isize, y:isize) -> bool {
    (0 <= y && y < matrix.len() as isize) && (0 <= x && x < matrix[0].len() as isize)
}

fn is_symbol(item: char) -> bool {
    !item.is_numeric() && item != '.'
}  

fn get_adjacent_numbers(matrix: &[Vec<char>], i: usize, j: usize) -> Vec<u32> {
    let mut numbers = vec![];
    let directions: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),          (0, 1),
        (1, -1), (1, 0),  (1, 1) 
    ];

    let mut checked = vec![vec![false; matrix[0].len()]; matrix.len()];

    for &(dx, dy) in &directions {
        let y = i as isize + dy;
        let x = j as isize + dx;

        if in_bounds(matrix, y, x) && matrix[y as usize][x as usize].is_numeric() && !checked[y as usize][x as usize] {
            let number = extract_full_number(matrix, y as usize, x as usize, &mut checked);
            numbers.push(number);
        }
    }

    numbers
}

fn extract_full_number(matrix: &[Vec<char>], i: usize, j: usize, checked: &mut [Vec<bool>]) -> u32 {
    let mut number_str = String::new();
    let mut x = j;

    while x > 0 && matrix[i][x - 1].is_numeric() {
        x -= 1;
    }

    while x < matrix[i].len() && matrix[i][x].is_numeric() {
        number_str.push(matrix[i][x]);
        checked[i][x] = true;
        x += 1;
    }

    number_str.parse::<u32>().unwrap_or(0)
}
