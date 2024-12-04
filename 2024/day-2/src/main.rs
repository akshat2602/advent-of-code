fn get_input(input: &str) -> Vec<Vec<i32>> {
    let mut levels = Vec::new();
    for line in input.lines() {
        let mut level = Vec::new();
        for num in line.split_whitespace() {
            level.push(num.parse().unwrap());
        }
        levels.push(level);
    }
    levels
}

fn main() {
    let path = std::path::Path::new("2024/day-2/input.txt");
    let input = std::fs::read_to_string(path).unwrap();

    let levels = get_input(&input);

    let count = levels
        .iter()
        .map(|level| check_level(&level))
        .filter(|x| *x)
        .count();

    println!("{:?}", count);
}

fn check_level(level_input: &Vec<i32>) -> bool {
    // Check if this level has strictly increasing or strictly decreasing and if the change is more than 0 and less than 4
    let mut inc_or_dec = 0;

    if level_input[0] > level_input[1] {
        inc_or_dec = 1;
    }

    for i in 1..level_input.iter().count() {
        let some_val = level_input[i] - level_input[i - 1];
        if some_val < 0 && inc_or_dec == 1 {
            // Check if some_val lies within 0 and 4 exclusively
            if ![1, 2, 3].contains(&some_val.abs()) {
                return false;
            }
        } else if some_val > 0 && inc_or_dec == 0 {
            if ![1, 2, 3].contains(&some_val.abs()) {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}