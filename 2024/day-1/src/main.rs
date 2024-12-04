fn get_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut input1 = Vec::new();
    let mut input2 = Vec::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        input1.push(iter.next().unwrap().parse().unwrap());
        input2.push(iter.next().unwrap().parse().unwrap());
    }

    (input1, input2)
}

fn main() {
    let path = std::path::Path::new("input.txt");
    let input = std::fs::read_to_string(path).unwrap();

    let (input1, input2) = get_input(&input);

    let mut input1 = input1;
    input1.sort();
    let mut input2 = input2;
    input2.sort();

    println!("Distance: {}", calc_distance(&input1, &input2));
    println!("Similarity Score: {}", calc_sim_score(&input1, &input2));
}

fn calc_distance(input1: &Vec<i32>, input2: &Vec<i32>) -> i32 {
    let mut distance = 0;
    for i in 0..input1.len() {
        distance += (input1[i] - input2[i]).abs();
    }
    distance
}

fn calc_sim_score(input1: &Vec<i32>, input2: &Vec<i32>) -> i32 {
    let mut sim_score = 0;

    let mut input2_occurrence = std::collections::HashMap::new();
    for i in 0..input2.len() {
        let count = input2_occurrence.entry(input2[i]).or_insert(0);
        *count += 1;
    }

    for i in 0..input1.len() {
        let occur = match input2_occurrence.get(&input1[i]) {
            Some(o) => o,
            None => &0,
        };
        sim_score += occur * input1[i];
    }
    sim_score
}
