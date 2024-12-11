use std::collections::HashMap;

fn get_input(input: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut constraints: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut input_strs: Vec<Vec<i32>> = Vec::new();
    let mut parsing_constraints = true;
    for line in input.lines() {
        if line == "" {
            parsing_constraints = false;
            continue;
        }
        if parsing_constraints {
            let constraint_on: i32 = line.split('|').collect::<Vec<_>>()[0]
                .parse::<i32>()
                .unwrap();
            let constraint_by: i32 = line.split('|').collect::<Vec<_>>()[1]
                .parse::<i32>()
                .unwrap();
            let mut_constraints_vec = constraints.get_mut(&constraint_on);
            if mut_constraints_vec.is_none() {
                let mut new_vec = Vec::new();
                new_vec.push(constraint_by);
                constraints.insert(constraint_on, new_vec);
            } else {
                mut_constraints_vec.unwrap().push(constraint_by);
            }
        } else {
            input_strs.push(line.split(',').map(|x| x.parse::<i32>().unwrap()).collect());
        }
    }

    (constraints, input_strs)
}

fn main() {
    let path = std::path::Path::new("input.txt");
    let input = std::fs::read_to_string(path).unwrap();

    let (constraints, input) = get_input(&input);

    println!(
        "Sum of middle elements of correctly ordered inputs is: {:?} and sum of fixed middle elements are: {:?}",
        get_sum_of_middle_elements(&constraints, &input).0,
        get_sum_of_middle_elements(&constraints, &input).1
    );
}

// This function will return the index of all correctly ordered inputs
fn correctly_ordered_check(constraints: &HashMap<i32, Vec<i32>>, input: &Vec<i32>) -> bool {
    for i in 0..input.len() - 1 {
        if !constraints.contains_key(&input[i]) {
            return false;
        }
        for j in i + 1..input.len() {
            let mut found = false;
            if let Some(elements) = constraints.get(&input[i]) {
                for &element in elements {
                    if element == input[j] {
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                return false;
            }
        }
    }
    true
}

fn fix_ordering(constraints: &HashMap<i32, Vec<i32>>, input: &Vec<i32>) -> Vec<i32> {
    let mut fixed_update = vec![0; input.len()];

    for i in 0..input.len() {
        let mut count_found = 0;

        match constraints.get(&input[i]) {
            None => {
                // means it should not come before any and should be last
                count_found = 0;
            }
            Some(rules) => {
                for j in 0..input.len() {
                    if i == j {
                        continue;
                    }

                    let found = rules.iter().any(|&element| element == input[j]);
                    if found {
                        count_found += 1;
                    }
                }
            }
        }

        // the count of numbers that are found is the count of numbers that should come after it
        // so the current number's position = length of update - number of characters found
        // this is such that all the found characters have space to come after it.
        fixed_update[input.len() - count_found - 1] = input[i];
    }
    fixed_update
}

// Iterate over list of correct_ordered_idxs, get that idx from the input and find the middle element and add it to the sum
fn get_sum_of_middle_elements(
    constraints: &HashMap<i32, Vec<i32>>,
    input: &Vec<Vec<i32>>,
) -> (i32, i32) {
    let mut sum = 0;
    let mut fixing_sum = 0;
    for i in input.iter() {
        if correctly_ordered_check(constraints, i) {
            sum += i[i.len() / 2];
        } else {
            let new_i = fix_ordering(constraints, i);
            fixing_sum += new_i[new_i.len() / 2];
        }
    }
    (sum, fixing_sum)
}
