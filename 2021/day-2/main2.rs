use std::fs;
use std::env;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let content = fs::read_to_string(&args[1])
                    .unwrap();

    let (mut vert, mut horz) = (0,0);
    let mut aim = 0;

    for line in content.split("\n") {
        let v: Vec<_> = line.split(" ").collect();
        if v.len() == 2 {
            match (v[0] , v[1].parse().unwrap_or(0)) {
                ("forward", m) => {
                    horz += m;
                    vert += aim * m;
                },
                ("down",m) => aim += m,
                ("up",m) => aim -= m,
                _ => println!("nah")
            };
        }
    }

    println!("Vertical: {vert}, Horizontal: {horz}");
    println!("Result: {}", vert * horz);
}
