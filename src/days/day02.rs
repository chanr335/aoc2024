use std::fs;

pub fn solve() {
    let result: i32 = fs::read_to_string("inputs/day02.txt")
        .expect("ERROR: Could not find input file")
        .lines()
        .map(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            let first_num = nums[0];
            let second_num = nums[1];

            let mut increasing = true;
            if first_num > second_num {
                increasing = false;
            }

            let is_valid = nums.windows(2).all(|pair| match pair {
                [x, y] => {
                    let diff = (x - y).abs();
                    if diff < 1 || diff > 3 {
                        false
                    } else {
                        match increasing {
                            true => x <= y,
                            false => x >= y,
                        }
                    }
                }
                _ => false,
            });

            match is_valid {
                true => 1,
                false => 0,
            }
        })
        .sum();

    println!("Result: {}", result);
}

