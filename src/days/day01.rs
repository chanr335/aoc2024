use std::fs;

pub fn solve() {
    //create an array for the left and right side of the puzzle
    //32 bit integer dynamic lists stored in heap
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    //not setting fs to a variable
    fs::read_to_string("inputs/day01.txt")
        .expect("ERROR: could not find input file")
        .lines()
        //iterate through every line
        .for_each(|line| {
            //expect() -> return error, ok() -> ignore error and keep going, unwrap -> return's OK
            //but may panic
            let mut parts = line.split_whitespace();
            let left_num = parts.next().expect("ERROR: could not parse input");
            let right_num = parts.next().expect("ERROR: could not parse input");
            left.push(left_num.parse::<i32>().unwrap());
            right.push(right_num.parse::<i32>().unwrap());
            println!("{left_num}, {right_num}")
        });

    left.sort();
    right.sort();

    //iterate through every value in left
    //zip with every right iterator (linking them up into a tuple)
    //map them with their absolute difference
    let res = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum::<u32>();

    println!("ANSWER: {res}")
}
