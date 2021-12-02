use itertools::Itertools;
use std::fs;
use std::time::Instant;

fn part_1(nums: &Vec<u32>) -> u32 {
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
fn part_2(nums: &Vec<u32>) -> u32 {
    nums.iter()
        .tuple_windows::<(_, _, _)>()
        .map(|(f, s, t)| f + s + t)
        .tuple_windows::<(_, _)>()
        .filter(|(f, s)| f < s)
        .count()
        .try_into()
        .unwrap()
}

fn part_2_noiter(nums: &Vec<u32>) -> u32 {
    let mut cnt = 0;
    for i in 3..nums.len() {
        //we can skip the middle two numbers because they cancel out
        if nums[i] > nums[i - 3] {
            cnt += 1;
        }
    }
    cnt
}

fn bench<T, R>(f: fn(T) -> R, args: T, msg: &str) {
    let n = Instant::now();
    f(args);
    let time = n.elapsed().as_nanos();
    println!("time elapsed since function call - {} :{} ns ", msg, time);
}
fn main() {
    let inp = fs::read_to_string("./input.txt").unwrap();
    let nums: Vec<u32> = inp.lines().map(|v| v.parse::<u32>().unwrap()).collect();

    println!("part1: {}", part_1(&nums));
    println!("part2: {}", part_2(&nums));
    
    bench(part_1, &nums, "part 1 iter");
    bench(d1_sum_noiter, &nums, "part 1 no iter");
    bench(part_2, &nums, "part 2 iter");
    bench(part_2_noiter, &nums, "part 2 no iter");
}
