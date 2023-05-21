use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();    
    
    let mut cnt = 0;
    let mut prev_val = 0;
    for s in contents.split("\n") {
        
        match s.parse() {
            Ok(val) => {
                if val > prev_val {
                    cnt+=1;
                }
                prev_val = val;
            },
            Err(_error) => () 
        };
    }
    
    println!("{}", cnt-1);
}
