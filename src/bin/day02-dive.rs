use std::io;

use regex::Regex;

fn main() {
    let pattern = Regex::new(r"([a-zA-Z]*) (\d*)").unwrap();

    let mut horizontal = 0;
    let mut depth = 0;

    let mut aim = 0;
    let mut aim_horizontal = 0;
    let mut aim_depth = 0;

    loop {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(0) => {
                // Done
                break;
            }
            Ok(_) => {
                let captures = match pattern.captures(&buffer) {
                    Some(c) => c,
                    None => continue,
                };

                let num = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();

                match captures.get(1).unwrap().as_str() {
                    "forward" => {
                        horizontal += num;
                        aim_horizontal += num;
                        aim_depth += aim * num;
                    },
                    "down" => {
                        depth += num;
                        aim += num;
                    },
                    "up" => {
                        depth -= num;
                        aim -= num;
                    },
                    cmd => println!("Unknown command: {}", cmd),
                }
            }
            Err(error) => {
                println!("Error: {}", error);
                return;
            }
        }
    }

    println!("{} {}", horizontal * depth, aim_horizontal * aim_depth);
}
