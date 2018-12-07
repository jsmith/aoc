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

fn main() {
    // assert_eq!(part1("abcdef"), 0);
    // assert_eq!(part1("bababc"), 1);
    // assert_eq!(part1("abbcde"), 0);
    // assert_eq!(part1("abcccd"), 0);
    // assert_eq!(part1("aabcdd"), 0);
    assert_eq!(part1("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"), 12);
    let _contents = fs::read_to_string("src/input.txt").expect("Unable to read file");
    println!("Part 1: {}", part1(&_contents));
}
