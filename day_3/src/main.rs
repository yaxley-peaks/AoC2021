use std::fs;

fn part1_bad(inp: &Vec<Vec<char>>) -> u128 {
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

    (gms * eps).try_into().unwrap()
}

fn count_bits_at_pos(nums: &[u32], pos: usize) -> (usize, usize) {
    let mut res = [0, 0];
    for &num in nums {
        res[(num as usize >> pos) & 1] += 1;
    }
    (res[0], res[1])
}

fn part1(nums: &[u32]) -> u32 {
    let (mut x, mut y) = (0, 0);

    for i in 0..12 {
        let (zero, one) = count_bits_at_pos(&nums, i);
        let temp = if one > zero { &mut x } else { &mut y };
        *temp += 1 << i;
    }

    x * y
}

fn part2(nums: &[u32], a: u32, b: u32) -> u32 {
    let mut nums = nums.to_vec();
    for i in (0..12).rev() {
        let (zero, one) = count_bits_at_pos(&nums, i);
        let keep = if one >= zero { a } else { b };
        nums.retain(|x| (x >> i) & 1 == keep);
        if nums.len() == 1 {
            break;
        }
    }
    nums[0]
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let input: Vec<_> = input
        .lines()
        .into_iter()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect();

    let inp = fs::read_to_string("./input.txt").unwrap();
    let inp = inp
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect::<Vec<_>>();

    println!("{}", part1(&inp));
    println!("{}", part2(&inp, 1, 0) * part2(&inp, 0, 1));

    println!("{}", part1_bad(&input));
}
