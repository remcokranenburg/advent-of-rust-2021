use std::io;

fn main() {
    let mut numbers = vec![];

    loop {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(0) => {
                // Done
                break;
            }
            Ok(_) => {
                match buffer.trim().parse::<i64>() {
                    Ok(num) => numbers.push(num),
                    Err(_) => (),
                }
            }
            Err(error) => {
                println!("Error: {}", error);
                return;
            }
        }
    }

    let mut num_increases = 0;
    let mut sliding_increases = 0;

    for (i, num) in numbers.iter().enumerate() {
        if i > 0 && num > &numbers[i - 1] {
            num_increases += 1;
        }

        if i > 2 &&
                num + &numbers[i - 1] + &numbers[i - 2] >
                    &numbers[i - 1] + &numbers[i - 2] + &numbers[i - 3] {
            sliding_increases += 1;
        }
    }

    println!("{} {}", num_increases, sliding_increases);
}
