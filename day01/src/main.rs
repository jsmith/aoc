use std::fs;
use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<isize> {
    input
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .collect()
}

fn part1(input: &[isize]) -> isize {
    return input.iter().sum();
}

fn part2(input: &[isize]) -> isize {
    let mut seen = HashSet::new();
    let mut sum = 0;
    seen.insert(sum);

    for freq in input.iter().cycle() {
        sum += freq;
        if !seen.insert(sum) {
            break;
        }
    }

    return sum;
}

fn main() {
    println!("Hello, world!");
    assert_eq!(part1(&[1, -2, 3, 1]), 3);
    assert_eq!(part1(&[1, 1, 1]), 3);
    assert_eq!(part1(&[1, 1, -2]), 0);
    assert_eq!(part1(&[-1, -2, -3]), -6);
    assert_eq!(part2(&[1, -2, 3, 1]), 2);
    assert_eq!(part2(&[1, -1]), 0);
    assert_eq!(part2(&[3, 3, 4, -2, -4]), 10);
    assert_eq!(part2(&[-6, 3, 8, 5, -6]), 5);
    assert_eq!(part2(&[7, 7, -2, -7, -4]), 14);

    let contents = fs::read_to_string("src/day1.txt").expect("Unable to read file");
    let input = parse_input(&contents);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
