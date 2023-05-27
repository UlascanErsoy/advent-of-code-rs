use std::{fs, env};

#[derive(Debug)]
struct Board {
    lines: Vec<Vec<i32>>
}

impl Board {
    fn new() -> Board  {
        Board { lines: Vec::new() }
    }
    fn new_line(&mut self, line: Vec<i32>) {
        self.lines.push(line);
    }
    fn empty(&self) -> bool {
        self.lines.len() == 0
    }
    fn mark(&mut self,  number: i32) -> i32 {

        let mut score = 0;
        let mut winning = false;
        let mut col_sums: Vec<i32> = vec![0; self.lines[0].len()];
        for line in self.lines.iter_mut() {
           for (cdx, val) in line.iter_mut().enumerate() {
               if *val == number {
                    *val = 0;
               }
               col_sums[cdx] += *val;
           }

           //check row
           let row_sum: i32 = line.iter().sum();
           score += row_sum;
           if row_sum == 0 {
               winning = true;
           }

           
        }
        
        if col_sums.contains(&0) {
            winning = true;
        }
        if winning {score * number} else {-1}
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).unwrap();
    let mut numbers: Vec<i32> = Vec::new();
    let mut boards: Vec<Board> = Vec::new();
    let mut n: usize = 0;

    for (idx, line) in contents.split("\n").enumerate() {
        if idx == 0 {
            numbers = line.split(",").map(|item| item.parse().unwrap())
                          .collect();
        } else if line.len() > 0 {
            boards[n-1].new_line(
                line.split_whitespace().map(|item| item.parse().unwrap())
                               .collect()
                               );
        } else {
            boards.push(Board::new());
            n += 1;
        }
    }
    
    if boards[boards.len() - 1].empty() {
        boards.pop();
    }

    'outer: for number in numbers {
        for board in boards.iter_mut() {
            match board.mark(number) {
                -1 => 1,
                 i => {
                        println!("Winning {i}!");
                        break 'outer
                    }
            };
            }
        }
}
