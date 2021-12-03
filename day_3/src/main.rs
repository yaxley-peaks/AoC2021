use std::fs;

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
    let inp = fs::read_to_string("./input.txt").unwrap();
    let inp = inp
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect::<Vec<_>>();

        println!("{}", part1(&inp));
        println!("{}", part2(&inp, 1, 0) * part2(&inp, 0, 1));
}
