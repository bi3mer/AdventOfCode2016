use std::collections::HashSet;
use std::fs;

enum Direction {
    North,
    East,
    South,
    West
}

fn manhattan_distance(x0: i32, y0: i32, x1: i32, y1: i32) -> i32 {
    (x1 - x0).abs() + (y1 - y0).abs()
}

fn run(path: &str) {
    let mut x = 0;
    let mut y = 0;
    let mut current_direction = Direction::North;

    let mut visited = HashSet::new();
    let mut hq_location = (0, 0);
    let mut found = false;

    for cmd in path.split(", ") {
        let dir = cmd.chars().nth(0).unwrap();
        let forward_str = &cmd[1..cmd.chars().count()];
        let forward_amount = forward_str.parse::<i32>().unwrap();

        match current_direction {
            Direction::North => {
                if dir == 'R' {
                    current_direction = Direction::East;
                } else {
                    current_direction = Direction::West;
                }
            },
            Direction::East => {
                if dir == 'R' {
                    current_direction = Direction::South;
                } else {
                    current_direction = Direction::North;
                }
            },
            Direction::South => {
                if dir == 'R' {
                    current_direction = Direction::West;
                } else {
                    current_direction = Direction::East;
                }
            },
            Direction::West => {
                if dir == 'R' {
                    current_direction = Direction::North;
                } else {
                    current_direction = Direction::South;
                }
            }
        }
        
        if !found {
            for _ in 0..forward_amount {
                match current_direction {
                    Direction::North => y += 1,
                    Direction::East => x += 1,
                    Direction::South => y -= 1,
                    Direction::West => x -= 1
                }

                if !found {
                    let pos = (x ,y);
                    if visited.contains(&pos) {
                        hq_location = pos;
                        found = true;
                    } else {
                        visited.insert(pos);
                    }
                }
            }
        } else {
            match current_direction {
                Direction::North => y += forward_amount,
                Direction::East => x += forward_amount,
                Direction::South => y -= forward_amount,
                Direction::West => x -= forward_amount
            }
        }
    }

    println!("distance: {}", manhattan_distance(0, 0, x, y));
    println!("hq location: {}", manhattan_distance(0, 0, hq_location.0, hq_location.1));
}

fn main() {
    let content = fs::read_to_string("input/01.txt")
        .expect("Unable to read file for some reason");

    run(&content.to_string());
}
