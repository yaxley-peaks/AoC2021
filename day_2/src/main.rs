use itertools::Itertools;
use std::fs;

fn part1() {
    let inp = fs::read_to_string("./input.txt").unwrap();
    let inp = inp
        .lines()
        .map(|x| x.split(" ").collect_tuple::<(_, _)>().unwrap())
        .map(|(f, s)| (f, s.parse::<u32>().unwrap()))
        .collect::<Vec<(_, _)>>();

    let mut depth = 0;
    let mut length = 0;

    for mes in inp {
        match mes {
            ("forward", x) => {
                length += x;
            }
            ("up", x) => {
                depth -= x;
            }
            ("down", x) => {
                depth += x;
            }
            _ => {}
        }
    }
    println!("{}", length * depth);
}

fn part2() {
    let inp = fs::read_to_string("./input.txt").unwrap();
    let inp = inp
        .lines()
        .map(|x| x.split(" ").collect_tuple::<(_, _)>().unwrap())
        .map(|(f, s)| (f, s.parse::<u32>().unwrap()))
        .collect::<Vec<(_, _)>>();

    let mut length = 0;
    let mut depth = 0;
    let mut aim = 0;

    for mes in inp {
        match mes {
            ("forward", x) => {
                length += x;
                depth += aim * x;
            }
            ("up", x) => {
                aim -= x;
            }
            ("down", x) => {
                aim += x;
            }
            _ => {}
        }
    }
    println!("{}" , length*depth);
}
fn main() {
    part1();
    part2();
}
