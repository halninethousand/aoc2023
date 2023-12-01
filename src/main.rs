fn main() {
    let input = include_str!("../data/day1.txt").lines().collect::<Vec<&str>>();
    let mut second = input.clone();

    find_first_last(&second.clone());

    let digits = ["zero", "one", "two", "there", "four", "five", "six", "seven", "eight", "nine"];
    static mut modified_line: String = String::new();
    unsafe {
    for line in second.iter_mut() {
            modified_line = line.to_string();  // Change type to String
            
            for _ in 0..5 {
                for (i, digit) in digits.iter().enumerate() {
                    if modified_line.contains(*digit) {
                            modified_line = modified_line.replace(digit, &i.to_string());
                            println!("modified_line = {:?}", modified_line);
                        }
                }
            }
            *line = modified_line.as_str();
        }
    find_first_last(&second);
    }
}

fn find_first_last(input: &Vec<&str>) -> u32 {
    let mut answer = 0;
    for _ in 0..5 {
        let mut parts: Vec<char> = vec![];
        for line in input {
            if line == &"" {
                break;
            }
            eprintln!("before {parts:?}");
            let mut parts = line.chars().collect::<Vec<char>>();
            parts.retain(|&part| part.is_digit(10));

            eprintln!("after {parts:?}");

        }
        if parts.len() == 1 { 
            let first = parts[0]; 
            let last = parts[0]; 
            let s = first + last;
        } else {
            let first = String::from(*parts.clone().first().unwrap());
            println!("{first:?}");
            let last = String::from(*parts.clone().last().unwrap());
            let s = first + &last;
        }
        answer += s.parse::<i32>().unwrap();
    }
    eprintln!("sum of calibration values = {:?}", answer); 
    answer as u32
}