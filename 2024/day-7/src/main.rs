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
        get_possible_sum(&tests, &tests_input, false)
    );
    println!(
        "Sum of possible tests with concat: {:?}",
        get_possible_sum(&tests, &tests_input, true)
    );
}

fn is_possible(current_sum: i64, test: &i64, test_input: &Vec<i64>, part2: bool) -> bool {
    // Base case: If no numbers are left and test is 0, it's possible
    if test_input.is_empty() {
        return *test == current_sum;
    }

    if current_sum > *test {
        return false;
    }
    // Check if addition causes a test case to pass
    if is_possible(
        current_sum + test_input[0],
        test,
        &test_input[1..].to_vec(),
        part2,
    ) {
        return true;
    }

    if part2 {
        // Check if concatenation causes a test case to pass
        // Caculcate concatenation new number
        let mut q: i64 = 10;
        let mut mul: i64 = 10;

        while q != 0 {
            q = test_input[0] / mul;
            if q > 0 {
                mul *= 10;
            }
        }

        if is_possible(
            (current_sum * mul) + test_input[0],
            test,
            &test_input[1..].to_vec(),
            part2,
        ) {
            return true;
        }
    }

    // Check if multiplication causes a test case to pass
    if current_sum == 0 {
        return is_possible(1 * test_input[0], test, &test_input[1..].to_vec(), part2);
    }
    return is_possible(
        current_sum * test_input[0],
        test,
        &test_input[1..].to_vec(),
        part2,
    );
}

fn get_possible_sum(tests: &Vec<i64>, tests_input: &Vec<Vec<i64>>, part2: bool) -> i64 {
    // iterate over tests and tests_input and then check if its possible,
    // if yes then add to sum and then return that sum
    let mut sum: i64 = 0;

    for (idx, value) in tests.iter().enumerate() {
        if is_possible(0, &value, &tests_input[idx], part2) {
            sum += value;
        }
    }
    sum
}
