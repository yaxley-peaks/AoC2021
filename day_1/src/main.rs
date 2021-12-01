use std::fs;
use itertools::Itertools;

fn d1_sum(){
    let inp = fs::read_to_string("./input.txt").unwrap();
    let nums: Vec<u32> = inp.lines().map(|v| v.parse::<u32>().unwrap()).collect();
    let mut num = 0;
    for x in 1..nums.len(){
        if nums[x] > nums[x-1]{
            num += 1
        }
    }
    println!("part1: {}", num);
}

fn d3_sum(){
    let inp = fs::read_to_string("./input.txt").unwrap();
    let nums: Vec<u32> = inp.lines().map(|v| v.parse::<u32>().unwrap()).collect();

   let res =  nums
        .iter()
        .tuple_windows::<(_,_,_)>()
        .map(|(f,s,t)| f+s+t)
        .tuple_windows::<(_,_)>()
        .filter(|(f,s)| f < s)
        .count();
    println!("second part: {}", res);
}

fn main() {
    d1_sum();
    d3_sum();
}
