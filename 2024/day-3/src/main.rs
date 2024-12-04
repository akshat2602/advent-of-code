use regex::Regex;

fn get_input(input: &str) -> Vec<&str> {
    println!("{:?}", input);
    // Go over the input and take out anything of the form mul(i32,i32)
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mut valid_input = Vec::new();
    for matc in re.find_iter(input) {
        valid_input.push(matc.as_str());
    }

    valid_input
}

fn main() {
    let path = std::path::Path::new("input.txt");
    let input = std::fs::read_to_string(path).unwrap();

    let valid_input = get_input(&input);

    let sum: i32 = valid_input.iter().map(|x| get_mul(*x)).sum();

    println!("Sum of valid muls are {:?}", sum);
}

fn get_mul(mul_string: &str) -> i32 {
    // Parse the mul string
    let start_idx = mul_string.find("(").unwrap();
    let mid_idx = mul_string.find(",").unwrap();
    let end_idx = mul_string.find(")").unwrap();

    let first_num: i32 = mul_string[start_idx + 1..mid_idx].parse().unwrap();
    let second_num: i32 = mul_string[mid_idx + 1..end_idx].parse().unwrap();

    first_num * second_num
}
