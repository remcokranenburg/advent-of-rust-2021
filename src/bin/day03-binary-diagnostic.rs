use std::io;

fn to_number(v: &Vec<i64>) -> i64 {
    let mut out = 0;

    for (i, x) in v.iter().rev().enumerate() {
        //println!("v={:?},i={},x={} ({})", v, i, x, x * 2 << i);
        if *x == 1 {
            out += 1 << i;
        }
    }

    out
}

fn main() {
    let mut rows = vec![];

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
                    match c.parse::<i64>() {
                        Ok(num) => row.push(num),
                        Err(_) => continue,
                    }
                }

                rows.push(row);
            }
            Err(error) => {
                println!("Error: {}", error);
                return;
            }
        }
    }

    let mut gamma = vec![];
    let mut epsilon = vec![];

    for (i, _) in rows[0].iter().enumerate() {
        let mut zeros = 0;
        let mut ones = 0;

        for row in &rows {
            if row.len() <= i {
                continue
            }

            match row[i] {
                0 => zeros += 1,
                1 => ones += 1,
                x => panic!("Unexpected number: {}", x),
            }
        }

        if zeros > ones {
            gamma.push(0);
            epsilon.push(1);
        } else if ones > zeros {
            gamma.push(1);
            epsilon.push(0);
        } else {
            panic!("Error: same number of 0s and 1s");
        }
    }

    let mut oxygen = rows.to_vec();
    let mut co2 = rows.to_vec();

    for (i, _) in oxygen[0].to_vec().iter().enumerate() {
        if oxygen.len() == 1 {
            break;
        }

        let mut new_oxygen : Vec<Vec<i64>> = vec![];

        let mut zeros = 0;
        let mut ones = 0;

        for row in &oxygen {
            if row.len() <= i {
                continue
            }

            match row[i] {
                0 => zeros += 1,
                1 => ones += 1,
                x => panic!("Unexpected number: {}", x),
            }
        }

        for row in &oxygen {
            if row.len() <= i {
                continue
            }

            match row[i] {
                0 => if zeros > ones { new_oxygen.push(row.to_vec()) },
                1 => if ones >= zeros { new_oxygen.push(row.to_vec()) },
                x => panic!("Unexpected number: {}", x),
            }
        }

        oxygen = new_oxygen;
    }

    for (i, _) in co2[0].to_vec().iter().enumerate() {
        if co2.len() == 1 {
            break;
        }

        let mut new_co2 : Vec<Vec<i64>> = vec![];

        let mut zeros = 0;
        let mut ones = 0;

        for row in &co2 {
            if row.len() <= i {
                continue
            }

            match row[i] {
                0 => zeros += 1,
                1 => ones += 1,
                x => panic!("Unexpected number: {}", x),
            }
        }

        for row in &co2 {
            if row.len() <= i {
                continue
            }

            match row[i] {
                0 => if zeros <= ones { new_co2.push(row.to_vec()) },
                1 => if ones < zeros { new_co2.push(row.to_vec()) },
                x => panic!("Unexpected number: {}", x),
            }
        }
        //println!("{:?}", co2);
        co2 = new_co2;
    }

    //println!("gamma: {:?}, epsilon: {:?}", &gamma, &epsilon);
    //println!("gamma: {}, epsilon: {}", to_number(&gamma), to_number(&epsilon));
    println!("{}", to_number(&gamma) * to_number(&epsilon));
    //println!("{:?} {}", oxygen, to_number(&oxygen[0]));
    //println!("{:?} {}", co2, to_number(&co2[0]));
    println!("{}", to_number(&oxygen[0]) * to_number(&co2[0]));
}
