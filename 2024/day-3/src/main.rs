use regex::Regex;

fn get_input(input: &str) -> Vec<&str> {
    // Go over the input and take out anything of the form mul(i32,i32)
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
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

    let mut sum: i32 = 0;
    let mut active = true;
    for input in valid_input.iter() {
        if input.contains("don") {
            active = false;
        } else if input.contains("do(") {
            active = true;
        } else {
            if active {
                sum += get_mul(*input);
            }
        }
    }

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
