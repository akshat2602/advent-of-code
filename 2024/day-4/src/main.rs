// Get input as string, convert the letters X, M, A, S to 1,2,3,4
// and remove the other letters and return a 2D matrix
fn get_input(input: &str) -> Vec<Vec<i32>> {
    let mut levels = Vec::new();
    for line in input.lines() {
        let mut level = Vec::new();
        for c in line.chars() {
            level.push(match c {
                'X' => 1,
                'M' => 2,
                'A' => 3,
                'S' => 4,
                _ => 0,
            });
        }
        levels.push(level);
    }
    levels
}

fn main() {
    let path = std::path::Path::new("input.txt");
    let input = std::fs::read_to_string(path).unwrap();

    let xmas = get_input(&input);

    let xmas = get_number_of_valid_xmas(xmas);

    println!("Number of valid xmas: {:?}", xmas);
}

fn check_for_increasing_num(
    xmas: &Vec<Vec<i32>>,
    i_dir: i32,
    j_dir: i32,
    starting_i: usize,
    starting_j: usize,
) -> bool {
    // given a starting position and i and j directions, check if I get 1,2,3,4 in that direction
    let mut start_i = starting_i as isize;
    let mut start_j = starting_j as isize;
    let mut prev_i = starting_i as isize;
    let mut prev_j = starting_j as isize;

    for _ in 0..3 {
        start_i += i_dir as isize;
        start_j += j_dir as isize;

        if start_i < 0
            || start_j < 0
            || start_i as usize >= xmas.len()
            || start_j as usize >= xmas[0].len()
        {
            return false;
        }
        if xmas[prev_i as usize][prev_j as usize] + 1 != xmas[start_i as usize][start_j as usize] {
            return false;
        }
        prev_i = start_i;
        prev_j = start_j
    }
    true
}

fn get_number_of_valid_xmas(xmas: Vec<Vec<i32>>) -> i32 {
    // Iterate over the matrix, anytime I encounter a 1, I check in all directions
    // whether I get a monotonically increasing counter, if yes then increase count by 1
    let mut count: i32 = 0;

    for (i, scam_v) in xmas.iter().enumerate() {
        for (j, _) in scam_v.iter().enumerate() {
            // If xmas[i][j] is 1, check in all directions
            if xmas[i][j] == 1 {
                let mut result = Vec::new();
                for i_dir in -1..2 {
                    for j_dir in -1..2 {
                        if i_dir == 0 && j_dir == 0 {
                            continue;
                        }
                        result.push(check_for_increasing_num(
                            &xmas,
                            i_dir,
                            j_dir,
                            i.try_into().unwrap(),
                            j.try_into().unwrap(),
                        ));
                    }
                }
                count += result.iter().filter(|x| **x).count() as i32;
            }
        }
    }

    count
}
