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

fn part2(input: &str) -> usize {
    let counts: Vec<_> = input
        .lines()
        .collect();

    let length = counts[0].len();
    return length;
}

fn main() {
    assert_eq!(part1("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"), 12);
    let _contents = fs::read_to_string("src/input.txt").expect("Unable to read file");
    println!("Part 1: {}", part1(&_contents));
    println!("Part 2: {}", part2(&_contents));
}
