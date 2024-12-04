use std::fs;
use std::time::SystemTime;

fn main() {

    let filename = "./input.txt";
    println!("reading input file: {filename}");
    let input = fs::read_to_string("./input.txt").expect(&format!("could not find file: {filename}"));

    let start_time = SystemTime::now();

    let reports = input.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|token| {
                    token.parse::<i32>()
                        .expect(&format!("Could not parse token: {token}"))
                })
                .collect::<Vec<i32>>()
        });

    let is_safe = |report: &Vec<i32>| -> bool {
        let greatest_difference = greatest_difference(report);
        is_monotonic(report) && greatest_difference <= 3 && greatest_difference > 0
    };

    let num_safe = reports.clone().fold(0, |acc, report| {
        if is_safe(&report) { acc + 1 } else { acc }
    });

    let part_1_duration = start_time.elapsed().unwrap();

    // CHECKPOINT: Part 2 begins here

    let num_safe_dampened = reports.fold(0, |acc, report| {
        // this could be faster if we did a sliding exclusion window with an iteratof filter rather than copying the data
        let dampened_reports = get_dampened_reports(&report);
        if is_safe(&report) || dampened_reports.iter().any(is_safe) { acc + 1 } else { acc }
    });

    let part_2_duration = start_time.elapsed().unwrap();

    println!("part 1 elapsed time: {:?}", part_1_duration);
    println!("part 2 elapsed time: {:?}", part_2_duration);

    println!("num safe is: {num_safe}");
    println!("num safe when dampened is: {num_safe_dampened}")




}

#[test]
fn test_is_monotonic_asc() {
    assert_eq!(true, is_monotonic(&vec![1, 2, 4]))
}
#[test]
fn test_is_monotonic_desc() {
    assert_eq!(true, is_monotonic(&vec![5, 3, 2]))
}
#[test]
fn test_is_monotonic_false() {
    assert_eq!(false, is_monotonic(&vec![1, 4, 3]))
}
fn is_monotonic(list: &Vec<i32>) -> bool {
    let sig = (list.last().unwrap() - list.first().unwrap()).signum();
    list.windows(2).all(|window| {
        (window[1] - window[0]).signum() == sig
    })
}

#[test]
fn test_greatest_difference() {
    assert_eq!(5, greatest_difference(&vec![1, 2, 7, 8, 9]))
}
fn greatest_difference(list: &Vec<i32>) -> u32 {
    list.windows(2)
        .map(|window| window[0].abs_diff(window[1]))
        .max()
        .unwrap_or(0)
}

#[test]
fn test_get_dampened_reports() {
    let report = vec![1, 2, 7, 8, 9];
    let dampened_reports = get_dampened_reports(&report);
    assert_eq!(vec![vec![2, 7, 8, 9], vec![1, 7, 8, 9], vec![1, 2, 8, 9], vec![1, 2, 7, 9], vec![1, 2, 7, 8]], dampened_reports);
}
fn get_dampened_reports(report: &Vec<i32>) -> Vec<Vec<i32>> {
    (0..report.len()).map(|n| {
        let mut permutation = report.clone();
        permutation.remove(n);
        permutation
    }).collect()
}
