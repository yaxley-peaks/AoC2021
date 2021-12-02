use itertools::Itertools;
use std::fs;
fn main() {
    let inp = fs::read_to_string("./input.txt").unwrap();
    let inp = inp
        .lines()
        .map(|x| x.split(" ").collect_tuple::<(_, _)>().unwrap())
        .map(|(f,s)| (f , s.parse::<u32>().unwrap()))
        .collect::<Vec<(_, _)>>();

    let mut depth = 0;
    let mut length = 0;

    for mes in inp{
        match mes {
            ("forward" , x) => {length +=x;},
            ("up" , x) => {depth -=x;},
            ("down" , x) => {depth +=x;},
            _ =>{}
        }
    }
    println!("{}", length*depth);
}
