use std::{vec, ops::Deref};

fn main() {
    let mut input: Vec<String> = include_str!("../data/day3.txt")
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

    let mut hehe: usize = 0;

    for (y, row) in input.iter().enumerate() {
        for (x, item) in row.chars().enumerate() {
            hehe = x; 
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
            locations.push((num_start.clone().unwrap(), Point{0:y, 1:hehe+1}));
            num_start = None;
        }
    }
    println!(" LOCATIONS :\n{locations:?} len {}", locations.len());


    let mut result = vec![];
    for location in locations {
        //println!("calling symbol_adj for {:?} {:?} {:?}", matrix.clone(), location.0.clone(), location.1.clone());
        if is_symbol_adjacent(matrix.clone(), location.0.clone(),location.1.clone()) {
            result.push(read_number(matrix.clone(), location.0, location.1));
        }
    }
    println!("answer {:?}", result.iter().map(|&x| x).sum::<u32>());
}

#[derive(Clone, Debug)]
struct Point(usize, usize);

fn read_number(matrix: Vec<Vec<char>>, start: Point, end: Point) -> u32 {

    let gg: String = matrix.clone()[start.0][start.1..end.1]
        .iter()

        .collect();
        //.map(|ch| ch.to_digit(10))
        //.flatten()
        //.collect();
    //println!("{:?}", gg.iter().sum::<u32>());
    
    gg.parse::<u32>().unwrap()
}
fn is_symbol_adjacent(matrix: Vec<Vec<char>>, start: Point, end: Point) -> bool {
    let offsets: [isize; 3] = [-1, 0, 1];
    //println!("start: {:?} | end: {:?}", start, end);
    for offset_i in offsets {
        let st: i32 = start.1 as i32 -1;
        let nd: i32 = end.1 as i32 +1;
        for current_j in st..nd {

            let current_i = start.0 as isize + offset_i;
            if in_bounds(matrix.clone(), current_i, current_j.try_into().unwrap()) && is_symbol(matrix[current_i as usize][current_j as usize]){
                //println!("symbol {:?} adjacent ", matrix[current_i as usize][current_j as usize]);
                return true
            }
        }
    }
    
    false
}

fn in_bounds(matrix: Vec<Vec<char>>, x: isize, y:isize) -> bool {
    (0 <= y && y < matrix.len() as isize) && (0 <= x && x < matrix[0].len() as isize)
}

fn is_symbol(item: char) -> bool {
    !item.is_numeric() && item != '.'
}  