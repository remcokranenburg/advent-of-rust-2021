use std::io;
use std::io::Read;

fn find_fish(fish: &Vec<(i64, i64)>, age: i64) -> i64 {
    for (i, f) in fish.iter().enumerate() {
        if f.1 == age {
            return i as i64
        }
    }

    -1
}

fn merge_fishes(fish: &mut Vec<(i64, i64)>, new_fish: Vec<(i64, i64)>) {
    for nf in new_fish {
        for mut f in &mut *fish {
            if nf.1 == f.1 {
                f.0 += nf.0;
                break;
            }
        }

        fish.push(nf);
    }
}

fn main() {
    let mut text = String::new();

    io::stdin().read_to_string(&mut text).unwrap();

    let fish: Vec<(i64, i64)> = text.trim().split(",")
        .map(|x| { (1, x.parse().unwrap()) })
        .collect();

    let mut generations = vec![fish];

    println!("Initial state: {:?}", generations[0]);

    for day in 0..257 {
        let mut new_fish = vec![];

        for i in 0..generations[day].len() {
            if generations[day][i].1 == 0 {
                let new_fish_index = find_fish(&new_fish, 6);

                if new_fish_index > -1 {
                    new_fish[new_fish_index as usize].0 += generations[day][i].0;
                } else {
                    new_fish.push((generations[day][i].0, 6));
                }

                let extra_fish_index = find_fish(&new_fish, 8);

                if extra_fish_index > -1 {
                     new_fish[extra_fish_index as usize].0 += generations[day][i].0;
                } else {
                     new_fish.push((generations[day][i].0, 8));
                }
            } else {
                let new_fish_index = find_fish(&new_fish, generations[day][i].1 - 1);

                if new_fish_index > -1 {
                    new_fish[new_fish_index as usize].0 += generations[day][i].0;
                } else {
                    new_fish.push((generations[day][i].0, generations[day][i].1 - 1));
                }
            }
        }

        generations.push(new_fish);

        let mut num_fish = 0;

        for f in &generations[day] {
            num_fish += f.0;
        }

        println!("After {} day: {:?}", day + 1, generations[day]);
        println!("After {} day: {}", day + 1, num_fish);
    }

}
