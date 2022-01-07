use std::io;
use std::u64;
use std::usize;

fn print_field(field: &Vec<Vec<(u64, bool, u64)>>) {
    for row in field {
        print!("row: ");
        for node in row {
            if node.1 {
                print!("{}", node.2);
            } else {
                print!(".")
            }
        }
        println!("");
    }
}

fn copy_row(row: &Vec<(u64, bool, u64)>) -> Vec<(u64, bool, u64)> {
    let mut result = vec![];

    for n in row {
        result.push((n.0 % 9 + 1, n.1, n.2));
    }

    result
}

fn main() {
    let mut field = vec![];

    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => {
                // Done
                break;
            }
            Ok(_) => {
                let mut row = vec![];

                for c in line.trim().split("") {
                    match c.parse::<u64>() {
                        Ok(num) => row.push((num, false, u64::MAX)),
                        Err(_) => continue,
                    }
                }

                if row.len() > 0 {
                    field.push(row);
                }
            }
            Err(error) => {
                println!("Error: {}", error);
                return;
            }
        }
    }

    println!("field read");

    for row in &mut field {
        let row_length = row.len();
        for i in row_length..(row_length * 5) {
            row.push((row[i - row_length].0 % 9 + 1, false, u64::MAX));
        }
    }

    let field_length = field.len();

    for i in field_length..(field_length * 5) {
        field.push(copy_row(&field[i - field_length]));
    }

    println!("field expanded");

    field[0][0].2 = 0;

    print_field(&field);

    let mut current_y: usize = 0;
    let mut current_x: usize = 0;

    loop {
        // println!("current: {} {}", current_y, current_x);
        print_field(&field);

        let low_y = current_y.checked_sub(1).unwrap_or(0);
        let high_y = usize::min(current_y + 2, field.len());

        for y in low_y..high_y {
            if !field[y][current_x].1 {
                let new_distance = field[current_y][current_x].2 + field[y][current_x].0;

                if new_distance < field[y][current_x].2 {
                    field[y][current_x].2 = new_distance;
                }
            }
        }

        let low_x = current_x.checked_sub(1).unwrap_or(0);
        let high_x = usize::min(current_x + 2, field[current_y].len());

        for x in low_x..high_x {
            if !field[current_y][x].1 {
                let new_distance = field[current_y][current_x].2 + field[current_y][x].0;

                if new_distance < field[current_y][x].2 {
                    field[current_y][x].2 = new_distance;
                }
            }
        }

        field[current_y][current_x].1 = true;

        if current_y == field.len() - 1 && current_x == field[current_y].len() - 1 {
            println!("{}", field[current_y][current_x].2);
            break;
        }

        let mut lowest_distance_unvisited = u64::MAX;
        let mut has_unvisited = false;

        for (y, row) in field.iter().enumerate() {
            for (x, node) in row.iter().enumerate() {
                if !node.1 {
                    has_unvisited = true;

                    if node.2 < lowest_distance_unvisited {
                        lowest_distance_unvisited = node.2;
                        current_y = y;
                        current_x = x;
                    }
                }
            }
        }

        if !has_unvisited {
            println!("All nodes visited.");
            break;
        }
    }
}
