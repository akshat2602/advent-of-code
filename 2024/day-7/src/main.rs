fn get_input(input: &str) -> (Vec<Vec<i64>>, Vec<i64>) {
    let mut tests_input = Vec::new();
    let mut tests = Vec::new();
    for line in input.lines() {
        let mut test_input = Vec::new();
        tests.push(
            line.split(':').collect::<Vec<&str>>()[0]
                .parse::<i64>()
                .unwrap(),
        );
        for num in line.split(':').collect::<Vec<&str>>()[1].split_whitespace() {
            test_input.push(num.parse().unwrap());
        }
        tests_input.push(test_input);
    }
    (tests_input, tests)
}

fn main() {
    let path = std::path::Path::new("input.txt");
    let input = std::fs::read_to_string(path).unwrap();

    let (tests_input, tests) = get_input(&input);
    println!(
        "Sum of possible tests: {:?}",
        get_possible_sum(tests, tests_input)
    );
}

fn is_possible(current_sum: i64, test: &i64, test_input: &Vec<i64>) -> bool {
    // Base case: If no numbers are left and test is 0, it's possible
    if test_input.is_empty() {
        return *test == current_sum;
    }

    if current_sum > *test {
        return false;
    }
    if is_possible(current_sum + test_input[0], test, &test_input[1..].to_vec()) {
        return true;
    }

    if current_sum == 0 {
        return is_possible(1 * test_input[0], test, &test_input[1..].to_vec());
    }
    return is_possible(current_sum * test_input[0], test, &test_input[1..].to_vec());
}

fn get_possible_sum(tests: Vec<i64>, tests_input: Vec<Vec<i64>>) -> i64 {
    // iterate over tests and tests_input and then check if its possible,
    // if yes then add to sum and then return that sum
    let mut sum: i64 = 0;

    for (idx, value) in tests.iter().enumerate() {
        if is_possible(0, &value, &tests_input[idx]) {
            sum += value;
        }
    }
    sum
}
