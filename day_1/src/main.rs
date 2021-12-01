use itertools::Itertools;
use std::fs;
use std::time::Instant;

fn d1_sum(nums: &Vec<u32>) -> u32 {
    nums.iter()
        .tuple_windows()
        .filter(|(f, s)| f < s)
        .count()
        .try_into()
        .unwrap()
}
#[allow(dead_code)]
fn d1_sum_noiter(nums: &Vec<u32>) -> u32 {
    let mut num = 0;
    for x in 1..nums.len() {
        if nums[x] > nums[x - 1] {
            num += 1
        }
    }
    num
}
fn d3_sum(nums: &Vec<u32>) -> u32 {
    nums.iter()
        .tuple_windows::<(_, _, _)>()
        .map(|(f, s, t)| f + s + t)
        .tuple_windows::<(_, _)>()
        .filter(|(f, s)| f < s)
        .count()
        .try_into()
        .unwrap()
}
fn bench(f: fn(&Vec<u32>) -> u32, args: &Vec<u32>, msg: &str) {
    let n = Instant::now();
    f(args);
    let time = n.elapsed().as_nanos();
    println!("time elapsed since function call - {} :{} ", msg, time);
}
fn main() {
    let inp = fs::read_to_string("./input.txt").unwrap();
    let nums: Vec<u32> = inp.lines().map(|v| v.parse::<u32>().unwrap()).collect();

    println!("part1: {}", d1_sum(&nums));
    println!("part2: {}", d3_sum(&nums));
    bench(d1_sum, &nums, "part 1 iter");
    bench(d3_sum, &nums, "part 2 iter");
}
