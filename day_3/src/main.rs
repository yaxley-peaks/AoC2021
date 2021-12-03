use itertools::Itertools;
use std::fs;
use transpose::transpose;

fn part1(inp: &Vec<Vec<char>>) -> u128{
    let mut gamma = Vec::new();
    let mut epsilon = Vec::new();

    let mut ones = 0;
    let mut zeroes = 0;
    for x in 0..inp[0].len() {
        ones = 0;
        zeroes = 0;
        for elem in inp.iter() {
            if elem[x] == '1' {
                ones += 1;
            } else {
                zeroes += 1;
            }
        }
        if ones > zeroes {
            gamma.push('1');
            epsilon.push('0');
        } else {
            epsilon.push('1');
            gamma.push('0')
        }
    }

    let eps: String = epsilon.iter().collect();
    let eps = isize::from_str_radix(&eps, 2).unwrap();

    let gms: String = gamma.iter().collect();
    let gms = isize::from_str_radix(&gms, 2).unwrap();

    (gms*eps).try_into().unwrap()
}


fn main() {
    let inp = fs::read_to_string("./input.txt").unwrap();
    let inp: Vec<_> = inp
        .lines()
        .into_iter()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect();

    
    println!("{}", part1(&inp));
}
