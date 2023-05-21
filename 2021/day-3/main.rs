use std::env;
use std::fs;

fn elem_sum(mut vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32> {

    for (idx, elem) in vec1.clone().iter().enumerate() {
        vec1[idx] += vec2[idx];
    }
    vec1
}

fn to_dec(vec1: Vec<i32>) -> i32 {
    let mut cum = 0;
    for (idx, elem) in vec1.iter().rev().enumerate() {
        cum += elem << (idx); 
    }

    cum
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1])
                    .unwrap();

    let v = contents.split("\n");
    
    let size = v.clone().collect::<Vec<_>>().len() as i32;

    let sums = v.map(|line| line.chars()
                                    .map(|ch| if ch == '1' {1} else {0})
                                    .collect::<Vec<i32>>()
                        )
                    .filter(|v| v.len() != 0)
                    .reduce(|acc, vec1| elem_sum(acc, vec1))
                    .unwrap();
    let most: Vec<i32> = sums.iter()
                .map(|&digit| if (size - digit) < digit {1} else {0})
                .collect();
                        
    
    let least: Vec<i32> = most.iter()
                                .map(|&digit| if digit == 1 {0} else {1})
                                .collect();

    
    let m_dec = to_dec(most);
    let l_dec = to_dec(least);
    println!("{m_dec}x{l_dec}={}",m_dec * l_dec);

    
}
