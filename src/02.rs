use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let file = File::open("input/02.txt").unwrap();
    let reader = BufReader::new(file);
    
    let key_pad_1 = [['1','2','3'], ['4','5','6'], ['7','8','9']];
    let mut x_1 = 1;
    let mut y_1 = 1;

    let key_pad_2 = [
        [' ', ' ', '1', ' ', ' '],
        [' ', '2', '3', '4', ' '],
        ['5', '6', '7', '8', '9'],
        [' ', 'A', 'B', 'C', ' '],
        [' ', ' ', 'D', ' ', ' ']];
    let mut x_2 = 0;
    let mut y_2 = 2;

    let mut result_1: String = "".to_owned();
    let mut result_2: String = "".to_owned();

    for line in reader.lines() {
        let line = line.unwrap(); 
        for c in line.chars() {
            match c {
                'U' => {
                    if y_1 > 0 {
                        y_1 -= 1;
                    }

                    if y_2 > 0 && key_pad_2[y_2 - 1][x_2] != ' ' {
                        y_2 -= 1;
                    }
                },
                'R' => {
                    if x_1 < 2 {
                        x_1 += 1;
                    }

                    if x_2 < 4 && key_pad_2[y_2][x_2 + 1] != ' ' {
                        x_2 += 1;
                    }
                },
                'D' => {
                    if y_1 < 2 {
                        y_1 += 1;
                    }
                    
                    if y_2 < 4 && key_pad_2[y_2 + 1][x_2] != ' ' {
                        y_2 += 1;
                    }
                },
                'L' => {
                    if x_1 > 0 {
                        x_1 -= 1;
                    }

                    if x_2 > 0 && key_pad_2[y_2][x_2 - 1] != ' ' {
                        x_2 -= 1;
                    }
                },
                _ => {
                    println!("{} is not handled!", c);
                }
            }
        }
        
        // set number
        result_1.push_str(&key_pad_1[y_1][x_1].to_string());
        result_2.push_str(&key_pad_2[y_2][x_2].to_string());
    }

    println!("Part 1: {}", result_1);
    println!("Part 2: {}", result_2);
}