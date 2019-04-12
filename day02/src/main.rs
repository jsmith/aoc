use std::fs;
use std::collections::HashMap;

fn count(line: &str) -> HashMap<char, isize> {
    let mut counts = HashMap::new();
    for c in line.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    return counts;
}

fn part1(input: &str) -> isize {
    let counts: Vec<_> = input
        .lines()
        .map(count)
        .collect();
    
    let mut threes = 0;
    let mut twos = 0;
    for map in counts.iter() {
        if map.values().any(|&val| val == 2) {
            twos += 1;
        }
        if map.values().any(|&val| val == 3) {
            threes += 1;
        }
    }

    return twos * threes;
}

// fn count_set_bits(i: u32) -> u32 {
//     i = i - (i.right_shift(1) & 0x55555555);
//     i = (i & 0x33333333) + ((i.shift_right(2)) & 0x33333333);
//     return (((i + (i.shift_right(4))) & 0x0F0F0F0F) * 0x01010101).right_shift(24);
// }

fn part2(input: &str) {
    let lines: Vec<_> = input
        .lines()
        .collect();

    for line1 in &lines {
        for line2 in &lines {
            if line1 == line2 {
                continue;
            }

            let mut different = 0;
            let mut found = 0;
            for (i, (c1, c2)) in line1.chars().zip(line2.chars()).enumerate() {
                if c1 != c2 {
                    different = i;
                    found += 1;
                    if found > 1 {
                        break;
                    }
                }
            }

            if found == 1 {
                for (i, c) in line1.chars().enumerate() {
                    if i == different { continue; }
                    print!("{}", c);
                }
                return;
            }
        }
    }
}

fn main() {
    assert_eq!(part1("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"), 12);
    let _contents = fs::read_to_string("src/input.txt").expect("Unable to read file");
    println!("Part 1: {}", part1(&_contents));
    print!("Part 2: ");
    part2(&_contents);
    println!();
}
