use std::{fs, time::SystemTime, vec};
type Pattern = Vec<(i32, i32)>;

fn main() {
    let start_time = SystemTime::now();

    let file_read_start_time = SystemTime::now();
    let filename = "./input.txt";
    println!("reading input file: {filename}");
    let input = fs::read_to_string("./input.txt").expect(&format!("could not find file: {filename}"));
    let file_read_duration = file_read_start_time.elapsed().unwrap();

    // CHECKPOINT - PART 1
    let part_1_start_time = SystemTime::now();

    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    let grid = Grid::new(grid);

    let directions = vec![
        (1, 0),     // east
        (1, 1),     // northeast
        (0, 1),     // north
        (-1, 1),    // northwest
        (-1, 0),    // west
        (-1, -1),   // southwest
        (0, -1),    // south
        (1, -1),    //southeast
    ];

    let patterns: Vec<Pattern> = directions.iter().map(|dir| {
        (0..4).map(|magnitude| {
            (dir.0 * magnitude, dir.1 * magnitude)
        }).collect()
    }).collect();

    let mut xmas_count = 0;
    
    for row in 0..grid.rows {
        for col in 0..grid.cols {
            let count_for_offset = patterns.iter().fold(0, |acc, pattern| {
                match grid.get_word_by_pattern(pattern, (row, col)) {
                    Some(word) => if word == "XMAS" {acc + 1} else {acc},
                    _ => acc
                }
            });
            xmas_count += count_for_offset;
        }
    }

    let part_1_duration = part_1_start_time.elapsed().unwrap();

    // CHECKPOINT - PART 2
    let part_2_start_time = SystemTime::now();

    // M M
    //  A
    // S S
    let x_mas_pattern = vec![
        (-1,  1),   // M
        ( 1,  1),   // M
        ( 0,  0),   // A
        (-1, -1),   // S
        ( 1, -1),   // S
    ];

    // The word we want to match now is "MMASS"

    let x_mas_patterns = (0..=3)
        .map(|n| rotate_pattern_ccw_times(&x_mas_pattern, n))
        .collect::<Vec<Pattern>>();

    let mut x_mas_count = 0;

    for row in 0..grid.rows {
        for col in 0..grid.cols {
            let count_for_offset = x_mas_patterns.iter().fold(0, |acc, pattern| {
                match grid.get_word_by_pattern(pattern, (row, col)) {
                    Some(word) => if word == "MMASS" {acc + 1} else {acc}
                    _ => acc
                }
            });
            x_mas_count += count_for_offset;
        }
    }

    let part_2_duration = part_2_start_time.elapsed().unwrap();
    let total_duration = start_time.elapsed().unwrap();

    // CHECKPOINT - DONE!

    println!("xmas count: {xmas_count}");
    println!("x-mas count: {x_mas_count}");
    println!("TIME:");
    println!("file read - {:?}", file_read_duration);
    println!("part 1 - {:?}", part_1_duration);
    println!("part 2 - {:?}", part_2_duration);
    println!("total - {:?}", total_duration);
}

fn rotate_pattern_ccw(pattern: &Pattern) -> Pattern {
    pattern.iter().map(|coords| {
        (-coords.1, coords.0)
    }).collect()
}

fn rotate_pattern_ccw_times(pattern: &Pattern, rotations: usize) -> Pattern {
    (0..rotations).fold(pattern.clone(), |acc_pattern, _| rotate_pattern_ccw(&acc_pattern))
}


struct Grid {
    vector: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(vector: Vec<Vec<char>>) -> Self {
        let rows = vector.len();
        let cols = vector.get(0).map(|row| row.len()).unwrap_or(0);
        Self {
            rows,
            cols,
            vector,
        }
    }
    fn get(&self, row: i32, col: i32) -> Option<char> {
        let row: usize = row.try_into().ok()?;
        let col: usize = col.try_into().ok()?;
        if     row >= self.rows
            || col >= self.cols
        {
            None
        } else {
            Some(self.vector[row][col])
        }
    }
    fn get_word_by_pattern(&self, pattern: &Pattern, offset: (usize, usize)) -> Option<String> {
        pattern.iter()
            .map(|coords: &(i32, i32)| {
                let offset_row = coords.0 + i32::try_from(offset.0).ok()?;
                let offset_col = coords.1 + i32::try_from(offset.1).ok()?;
                self.get(offset_row, offset_col)
            })
            .try_fold(String::new(), |mut acc, maybe_c| {
                acc.push(maybe_c?);
                Some(acc)
            })
    }
}
