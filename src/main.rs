use std::str::FromStr;

fn main() {
    let input = include_str!("../data/day2.txt");
    let mut answer: u32 = 0;
    for item in input.lines() {
        //println!("{item:?}");
        let game: Vec<&str> = item.split(":").collect();
        //println!("{game:?}");
        let mut attrs: Vec<&str> = game[1].clone().split(";").collect();
        let hand: Vec<&str> = attrs.clone();
        //println!("{attrs:?}");
        

        let mut ID: u32;
        let mut viable: bool = false;

        'outer: for item in hand {
            let mut colors: Vec<&str> = item.split(",").collect();
            let mut colors: Vec<String> = colors.iter().map(|X| X.to_string()).collect();
            println!("======");
            for color in colors.iter_mut() {
                if let Some(red) = color.trim().strip_suffix(" red") {
                    //println!("{red:?}");
                    //println!("{item:?}");

                    let red: u32 = FromStr::from_str(red).unwrap();

                    if red <= 12 {
                        println!("Red in check");
                        viable = true;
                    } else {
                        println!("red overflow");
                        viable = false;
                        break 'outer;
                    }
                } 
                if let Some(blue) = color.trim().strip_suffix(" blue") {
                    //println!("{blue:?}");
                    //println!("{item:?}");
                    let blue: u32 = FromStr::from_str(blue).unwrap();

                    if blue <= 14 {
                        println!("Blue in check");
                        viable = true;
                    } else {
                        println!("blue overflow");
                        viable = false;
                        break 'outer;
                    }
                } 
                if let Some(green) = color.trim().strip_suffix(" green") {
                    //println!("{green:?}");
                    //println!("{item:?}");

                    let green: u32 = FromStr::from_str(green).unwrap();

                    if green <= 13 {
                        println!("Green in check");
                        viable = true;
                    } else {
                        println!("green overflow");
                        viable = false;
                        break 'outer;
                    }
                } 
            }
            if viable == false {
                continue;
            }
        }

        if viable == true {
            ID = FromStr::from_str(game[0].strip_prefix("Game ").unwrap()).unwrap(); 
            answer += ID; 

            println!("Adding game {ID:?}\n{item:?}");
        }
    }
    println!("{answer:?}");
}

struct Game {
    red: u32,
    blue: u32,
    green: u32,
}

