use std::{fs, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut crabs: Vec<i32> = fs::read_to_string(&args[1])
                                .unwrap()
                                .split(",")
                                .map(|x| x.trim().parse::<i32>().unwrap())
                                .collect();
    
    crabs.sort();
    let size = crabs.len();

    let median = if size % 2 == 0 {
        let lm = crabs[size/2-1];
        let rm = crabs[size/2];
        (lm + rm) / 2
    }else{
        crabs[size/2]
    };

    let fuel = crabs.iter().fold(0, |acc, n| {
        acc + (n - median).abs()
    });
    println!("{fuel}");
}
