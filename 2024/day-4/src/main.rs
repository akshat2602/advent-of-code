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
    let path = std::path::Path::new("test.txt");
    let input = std::fs::read_to_string(path).unwrap();

    let levels = get_input(&input);

    println!("{:?}", levels);
}
