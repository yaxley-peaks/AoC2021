use itertools::Itertools;
use std::fs;

fn solve(inp: String) -> (u32, u32) {
    let inp = inp
        .lines()
        .map(|x| x.split(" ").collect_tuple::<(_, _)>().unwrap())
        .map(|(f, s)| (f, s.parse::<u32>().unwrap()))
        .collect::<Vec<(_, _)>>();

    let mut depth = 0;
    let mut length = 0;
    let mut aim = 0;
    let mut depth2 = 0;

    for mes in inp {
        match mes {
            ("forward", x) => {
                length += x;
                depth2 += aim * x;
            }
            ("up", x) => {
                depth -= x;
                aim -= x;
            }
            ("down", x) => {
                depth += x;
                aim += x;
            }
            _ => {}
        }
    }
    (length * depth, length * depth2)
}

fn bench<T, R>(f: fn(T) -> R, args: T, msg: &str) {
    use std::time::Instant;
    let n = Instant::now();
    f(args);
    let time = n.elapsed().as_nanos();
    println!("time elapsed since function call - {} :{} ns ", msg, time);
}

fn main() {
    let inp = fs::read_to_string("./input.txt").unwrap();
    let (x, y) = solve(inp.clone());

    bench(solve, inp.clone(), "part1");

    println!("{} {}", x, y);
}
