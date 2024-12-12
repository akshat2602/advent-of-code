// 0 signifies a normal position
// 1 signifies an obstacle

// 0 signifies up direction
// 1 signifies right direction
// 2 signifies down direction
// 3 signifies left direction
fn get_input(input: &str) -> (Vec<Vec<i32>>, (usize, usize), i32) {
    let mut manufacturing_lab_layout = Vec::new();
    let mut guard_starting_pos = (0, 0);
    let mut guard_starting_direction = 0;
    for (line_idx, line) in input.lines().enumerate() {
        let mut level = Vec::new();
        for (idx, c) in line.chars().enumerate() {
            level.push(match c {
                '.' => 0,
                '#' => 1,
                '^' | '<' | '>' | 'v' => 0,
                _ => unimplemented!(),
            });
            if c != '.' && c != '#' {
                guard_starting_pos = (line_idx, idx);
            }
            if c == '^' {
                guard_starting_direction = 0;
            } else if c == '>' {
                guard_starting_direction = 1;
            } else if c == 'v' {
                guard_starting_direction = 2;
            } else if c == '<' {
                guard_starting_direction = 3;
            }
        }
        manufacturing_lab_layout.push(level);
    }
    (
        manufacturing_lab_layout,
        guard_starting_pos,
        guard_starting_direction,
    )
}

fn main() {
    let path = std::path::Path::new("test.txt");
    let input = std::fs::read_to_string(path).unwrap();

    let (manufacturing_lab, guard_starting_pos, guard_starting_direction) = get_input(&input);

    // println!("Lab layout: {:?}", manufacturing_lab);
    // println!("Guard starting position: {:?}", guard_starting_pos);
    // println!("Guard starting direction: {:?}", guard_starting_direction);
    println!(
        "Number of distinct positions touched by the guard: {:?}",
        simulate_guard_path_and_count_visited(
            &manufacturing_lab,
            guard_starting_pos,
            guard_starting_direction
        )
    )
}

fn simulate_guard_path_and_count_visited(
    manufacturing_lab: &Vec<Vec<i32>>,
    guard_starting_pos: (usize, usize),
    guard_starting_direction: i32,
) -> i32 {
    // Simulate guard path
    // Anytime the guard encounters an obstacle, change direction clockwise
    let mut count = 1;
    let mut guard_curr_pos = guard_starting_pos;
    let mut guard_curr_direction = guard_starting_direction;
    let mut visited_positions = std::collections::HashSet::new();
    visited_positions.insert(guard_curr_pos);

    let directions = [
        (-1, 0), // Up
        (0, 1),  // Right
        (1, 0),  // Down
        (0, -1), // Left
    ];

    loop {
        // figure out guard_direction and then move guard in the matrix
        // breaking condition should be when you go out of bounds
        let next_row = guard_curr_pos.0 as isize + directions[guard_curr_direction as usize].0;
        let next_col = guard_curr_pos.1 as isize + directions[guard_curr_direction as usize].1;

        // Check boundaries
        if next_row < 0
            || next_row > (manufacturing_lab.len() - 1) as isize
            || next_col < 0
            || next_col > (manufacturing_lab[0].len() - 1) as isize
        {
            break; // Out of bounds
        }

        if manufacturing_lab[next_row as usize][next_col as usize] == 1 {
            // Change direction clockwise
            guard_curr_direction = (guard_curr_direction + 1) % 4;
        } else {
            // Move to the next position
            guard_curr_pos = (next_row as usize, next_col as usize);
            if !visited_positions.contains(&guard_curr_pos) {
                count += 1;
                visited_positions.insert(guard_curr_pos);
            }
        }
    }

    count
}
