use std::fs;

fn main() {
    let contents: Vec<_> = fs::read_to_string("input.txt")
                    .unwrap()
                    .split("\n")
                    .flat_map(|item| item.parse::<i32>())
                    .collect();    

    let wins = contents.windows(3) 
                       .map(|win| (win[0] + win[1] + win[2]));

    let mut cnt = 0;
    let mut prev_val = 0;
    for win in wins {
        if win > prev_val {
            cnt+=1;
        }
        prev_val = win;
    }
    println!("{}", cnt-1);
}
