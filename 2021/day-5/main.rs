use std::{fs, env};
use std::collections::HashMap;

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
                let (xs, xe) = if x1 <= x2 {
                    (x1, x2)
                }else{
                    (x2, x1)
                };

                let (ys, ye) = if y1 <= y2 {
                    (y1, y2)
                }else{
                    (y2, y1)
                };
                    for xdx in xs..=xe {
                        for ydx in ys..=ye {
                            *map.entry((xdx,ydx)).or_insert(0) += 1;
                        }
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
