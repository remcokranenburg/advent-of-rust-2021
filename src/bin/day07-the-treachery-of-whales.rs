use std::io;
use std::io::Read;
use std::i64;

fn calculate_cost(numbers: &Vec<i64>, position: i64) -> i64 {
    let mut cost = 0;

    for n in numbers {
        cost += (n - position).abs();
    }

    cost
}

fn calculate_cost2(numbers: &Vec<i64>, position: i64) -> i64 {
    let mut cost = 0;

    for n in numbers {
        let diff = (n - position).abs();

        cost += diff * (diff + 1) / 2;
    }

    cost
}

fn main() {
    let mut text = String::new();

    io::stdin().read_to_string(&mut text).unwrap();

    let numbers: Vec<i64> = text.trim().split(",").map(|c| c.parse().unwrap()).collect();

    let mut min = i64::MAX;
    let mut max = i64::MIN;

    for n in &numbers {
        if *n < min {
            min = *n;
        }
        if *n > max {
            max = *n;
        }
    }

    let mut min_cost = i64::MAX;
    let mut min_i = i64::MAX;

    for i in min..max {
        let cost = calculate_cost(&numbers, i);

        if cost < min_cost {
            min_cost = cost;
            min_i = i;
        }
    }

    let mut min_cost2 = i64::MAX;
    let mut min_i2 = i64::MAX;

    for i in min..(max + 1) {
        let cost = calculate_cost2(&numbers, i);

        if cost < min_cost2 {
            min_cost2 = cost;
            min_i2 = i;
        }

    }

    println!("{} {}", min_i, min_cost);
    println!("{} {}", min_i2, min_cost2);
}
