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
    return;
}

fn part2(input: &str) {
   
}

fn main() {
    assert_eq!(part1("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"), 12);
    let _contents = fs::read_to_string("src/input.txt").expect("Unable to read file");
    println!("Part 1: {}", part1(&_contents));
    print!("Part 2: ");
    part2(&_contents);
    println!();
}
