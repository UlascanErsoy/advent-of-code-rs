use std::{fs, env, cmp};
use std::collections::HashMap;

fn clamp(lim1: i32, lim2:i32, val:i32) -> i32 {
    let min = cmp::min(lim1,lim2);
    let max = cmp::max(lim1,lim2);
    if val <= cmp::min(lim1,lim2) {
        min
    }else if val >= cmp::max(lim1,lim2) {
        max 
    }else{
        val
    }
}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).unwrap();
    
    let mut map: HashMap<(i32,i32),i32> = HashMap::new();

    for line in contents.split("\n") {
        let v: Vec<_> = line.split("->")
                            .flat_map(|arr| arr.split(",").collect::<Vec<_>>())
                            .map(|num| num.trim().parse::<i32>())
                            .collect::<Vec<_>>();

        match v[..] {
            [Ok(x1),Ok(y1),Ok(x2),Ok(y2)] => {
                
                let steps = cmp::max(
                                (x2-x1).abs(),
                                (y2-y1).abs()
                                );
                let ydir = if y2 >= y1 {1} else {-1};
                let xdir = if x2 >= x1 {1} else {-1};

                for step in 0..=steps {
                        *map.entry((clamp(x1,x2,x1 + xdir * step),clamp(y1,y2,y1 + ydir * step))).or_insert(0) += 1;
                    }
            },
            _ => ()
        }
    }

    let result: Vec<_> = map.into_iter()
                    .filter(|((_x,_y),value)| value > &1)
                    .collect();

    println!("Score is {:?}",result.len());
}
