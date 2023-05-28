use std::{fs, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let crabs: Vec<i32> = fs::read_to_string(&args[1])
                                .unwrap()
                                .split(",")
                                .map(|x| x.trim().parse::<i32>().unwrap())
                                .collect();
    
    let m = crabs.iter().sum::<i32>() / crabs.len() as i32;
    let fuel: i32 = (m..=m+1).map(|mean| {
        crabs.iter().map(|n| {
            let dist = (n - mean).abs();
            dist * (dist + 1) / 2
        }).sum::<i32>()
        }).min().unwrap();
    println!("{:?}", fuel);
}
