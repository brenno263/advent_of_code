use std::fs;
use regex::Regex;

fn main() {

    let filename = "./input.txt";
    println!("reading input file: {filename}");
    let input = fs::read_to_string("./input.txt").expect(&format!("could not find file: {filename}"));

    // parens denote a capture group, so I'm capturing (\d+) and (\d+) out of this matching string.
    // \d is any digit, and \d+ is one or more digits.
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let sum = input.lines().fold(0, |acc, line| {
        let line_sum = pattern.captures_iter(line)
            .fold(0, |acc, capture| {
                // the 0th result of a capture is always the full string match, capture groups come after.
                let arg1 = capture[1].parse::<i32>().unwrap();
                let arg2 = capture[2].parse::<i32>().unwrap();
                acc + (arg1 * arg2)
            });
        acc + line_sum
    });

    println!("part 1 answer: {sum}");

    // CHECKPOINT - PART 2

    let pattern = Regex::new(r"(do\(\)|don't\(\)|mul\((\d+),(\d+)\))").unwrap();

    let mut enabled = true;
    let sum = input.lines().fold(0, |acc, line| {
        let line_sum = pattern.captures_iter(line).fold(0, |acc, capture| {
            // here the outer capture is at index 1
            let matched_str = &capture[1];
            if matched_str == "do()" {
                enabled = true;
                acc
            } else if matched_str == "don't()" {
                enabled = false;
                acc
            } else if enabled { // we've got a mul() on our hands
                // the 0th result of a capture is always the full string match.
                // 1 here is the outer capture group, inner capture groups come after.
                let arg1 = capture[2].parse::<i32>().unwrap();
                let arg2 = capture[3].parse::<i32>().unwrap();
                acc + (arg1 * arg2)
            } else {
                acc
            }
        });
        acc + line_sum
    });

    println!("part 2 answer: {sum}");
}


#[test]
fn test_get_mult_strings() {

}