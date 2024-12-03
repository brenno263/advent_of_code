use std::fs;

fn main() {

    let filename = "./input.txt";
    println!("reading input file: {filename}");
    let input = fs::read_to_string("./input.txt").expect(&format!("could not find file: {filename}"));

    let input_tuples = input
        .lines()
        .map(|line| {
            let ids = line
                .split_whitespace()
                .map(|id_str| {
                    id_str.parse::<u32>().expect("Failed to parse integer! {id_str}")
                })
                .collect::<Vec<_>>();

            assert!(ids.len() == 2);
            (ids[0], ids[1])
        });

    let (mut left_list, mut right_list): (Vec<u32>,Vec<u32>) = {
        let mut left_list = Vec::<u32>::new();
        let mut right_list = Vec::<u32>::new();
        input_tuples.for_each(|tuple| {
            left_list.push(tuple.0);
            right_list.push(tuple.1);
        });
        (left_list, right_list)
    };

    left_list.sort();
    right_list.sort();

    let difference_sum = difference_sum(&left_list, &right_list);

    println!("sum of differences: {difference_sum}");

    // CHECKPOINT: Part 2 starts here

    let similarity_score = similarity_score(&left_list, &right_list);

    println!("similarity score: {similarity_score}");
}

#[test]
fn test_difference_sum() {
    // taken from example for problem 1
    let mut left_list = vec![3, 4, 2, 1, 3, 3];
    let mut right_list = vec![4, 3, 5, 3, 9, 3];
    left_list.sort();
    right_list.sort();
    assert_eq!(11, difference_sum(&left_list, &right_list))

}
fn difference_sum(left_list: &Vec<u32>, right_list: &Vec<u32>) -> u32 {
    let differences = left_list.iter()
        .zip(right_list.iter())
        .map(|pair| {
            // convert to distances
            u32::abs_diff(*pair.0, *pair.1)
        });

    differences.sum()
}

#[test]
fn test_similarity_score() {
    // taken from example for problem 2
    let left_list = vec![3, 4, 2, 1, 3, 3];
    let right_list = vec![4, 3, 5, 3, 9, 3];
    assert_eq!(31, similarity_score(&left_list, &right_list));
}
fn similarity_score(left_list: &Vec<u32>, right_list: &Vec<u32>) -> u32 {
    left_list.iter()
        .fold(0, |acc, left_id| {
            let count_in_right = freq_in_list(*left_id, &right_list);
            acc + left_id * count_in_right
        })
}

#[test]
fn test_freq_in_list() {
    assert_eq!(3, freq_in_list(1, &vec![0, 1, 2, 1, 3, 1]));
}
fn freq_in_list(needle: u32, haystack: &Vec<u32>) -> u32 {
    haystack.iter()
        .fold(0, |acc: u32, n: &u32| if *n == needle { acc + 1 } else { acc })
}



