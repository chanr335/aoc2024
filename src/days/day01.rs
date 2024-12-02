use std::{collections::HashMap, fs};

pub fn solve() {
    part_one();
    part_two();
}

fn parse_file() -> Result<(Vec<i32>, Vec<i32>), ()> {
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
        });
    Ok((left, right))
}

//sort then find abs difference of each side of list
fn part_one() {
    let (mut left, mut right) = parse_file().expect("ERROR: could not parse file");
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

fn part_two() {
    let (left, right) = parse_file().expect("ERROR: could not parse file");
    //count the frequencies of every number in the right side
    let right_freq = right
        .iter()
        .copied()
        .fold(HashMap::<i32, i32>::new(), |mut f, num| {
            //modify the value at the key num
            f.entry(num).and_modify(|freq| *freq += 1).or_insert(1);
            //return f
            f
        });

    //iterate through the left list and multiply by it's key in the hashmap
    let ans: i32 = left
        .iter()
        .map(|num| {
            let freq = *right_freq.get(num).unwrap_or(&0);
            num * freq
        })
        .sum();

    println!("ANSWER: {ans}");
}
