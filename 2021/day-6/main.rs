use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let content: String = fs::read_to_string(&args[1]).unwrap();
    let mut fishes: Vec<_> = content.split(",")
                                    .fold(vec![0;9], |mut acc, x| {
                                        acc[x.trim().parse::<usize>().unwrap()] += 1;
                                        acc 
                                        });
    let days: i32 = args[2].parse().unwrap();

    println!("Simulating {days} days!");

    for day in 1..days {
       fishes[(day as usize + 7)%9] += fishes[day as usize %9]; 
    }

        
    println!("{} fishes!", fishes.iter().sum::<usize>());
}
