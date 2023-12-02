fn main() {
    let input: Vec<String> = include_str!("../data/current.txt")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let mut first = input.clone();
    let mut second = input.clone();

    find_first_last(&mut first);

    let digits = ["zero", "one", "two", "there", "four", "five", "six", "seven", "eight", "nine"];

    for line in second.iter_mut() {
        for _ in 0..5 {
            for (i, digit) in digits.iter().enumerate() {
                if line.contains(*digit) {
                    *line = line.replace(digit, &i.to_string());
                }
            }
        }
    }

    find_first_last(&mut second);
}

fn find_first_last(input: &mut Vec<String>) -> () {
    let mut sum: u32 = 0;

    for line in input.iter_mut() {
        if line.is_empty() {
            break;
        }
        let mut parts = line.chars().collect::<Vec<char>>();
        parts.retain(|&part| part.is_digit(10));


        println!("{line:?}");
        println!("{parts:?}");

        if parts.len() == 1 { 
            let first_last: u32 = (parts[0].to_string().as_str().to_owned() + parts[0].to_string().as_str()).parse().unwrap();
            println!("first_last = {:?}", first_last);
            sum += first_last;
        } else if parts.len() == 0 {
            continue
        } else {
            let first_last: u32 = (String::from(*parts.clone().first().unwrap()) + parts.clone().last().unwrap().to_string().as_str()).parse().unwrap();
            println!("first_last = {:?}", first_last);
            sum += first_last;
        }
    }
    eprintln!("sum of calibration values = {:?}", sum); 
}