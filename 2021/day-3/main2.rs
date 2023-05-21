use std::env;
use std::fs;

fn elem_sum(mut vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32> {

    for (idx, _elem) in vec1.clone().iter().enumerate() {
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

enum ReadingType {
    Oxy,
    Scrubber
}

fn filter_number(number: Vec<Vec<i32>>, reading: ReadingType) -> Option<Vec<i32>>{
    
    let mut result = number.clone();
    
    for idx in 0..number[0].len() {
        
        let filt: Vec<i32> = result.clone().into_iter()
                         .reduce(|acc, vec1| elem_sum(acc, vec1))
                         .unwrap()
                         .iter()
                         .map(|&digit| if (result.len() as i32 - digit) <= digit {1} else {0})
                         .collect();

        match reading {
            ReadingType::Oxy => result.retain(|n| n[idx] == filt[idx]),
            ReadingType::Scrubber => result.retain(|n| n[idx] != filt[idx])
        };

        if result.len() == 1 {
            break;
        }
    }

    result.last().cloned()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1])
                    .unwrap();

    let v = contents.split("\n");
    let numbers: Vec<Vec<i32>> = v.map(|line| line.chars()
                                    .map(|ch| if ch == '1' {1} else {0})
                                    .collect::<Vec<i32>>()
                    )
                    .filter(|v| v.len() != 0)
                    .collect();

    let oxy = to_dec(filter_number(numbers.clone(),ReadingType::Oxy).unwrap());
    let co2 = to_dec(filter_number(numbers,ReadingType::Scrubber).unwrap());
    println!("{oxy} x {co2} = {}",oxy*co2);
}
