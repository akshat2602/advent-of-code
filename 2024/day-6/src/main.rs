// Part 2 inspired by https://github.com/shraddhaag/aoc/blob/main/2024/day6/main.go

// 0 signifies a normal position
// 1 signifies an obstacle

use std::collections::{HashMap, HashSet};

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
    let path = std::path::Path::new("input.txt");
    let input = std::fs::read_to_string(path).unwrap();

    let (mut manufacturing_lab, guard_starting_pos, guard_starting_direction) = get_input(&input);

    let (count, path) = simulate_guard_path_and_count_visited(
        &manufacturing_lab,
        guard_starting_pos,
        guard_starting_direction,
    );
    println!(
        "Number of distinct positions touched by the guard: {:?}",
        count
    );
    println!(
        "Number of obstacles to create a loop: {:?}",
        add_obstacle_and_count_loops(
            &mut manufacturing_lab,
            guard_starting_direction,
            guard_starting_pos,
            path
        )
    );
}

fn get_next_step_path(
    manufacturing_lab: &Vec<Vec<i32>>,
    guard_curr_pos: (usize, usize),
    guard_curr_direction: i32,
) -> ((usize, usize), i32, bool) {
    let directions = [
        (-1, 0), // Up
        (0, 1),  // Right
        (1, 0),  // Down
        (0, -1), // Left
    ];
    let mut guard_new_direction = guard_curr_direction;
    let mut guard_new_pos = guard_curr_pos;
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
        return (guard_new_pos, guard_new_direction, false);
    }

    if manufacturing_lab[next_row as usize][next_col as usize] == 1 {
        // Change direction clockwise
        guard_new_direction = (guard_curr_direction + 1) % 4;
    } else {
        // Move to the next position
        guard_new_pos = (next_row as usize, next_col as usize);
    }
    (guard_new_pos, guard_new_direction, true)
}

fn detect_loop(
    manufacturing_lab: &Vec<Vec<i32>>,
    guard_starting_direction: i32,
    guard_starting_pos: (usize, usize),
) -> bool {
    let mut visited_positions: HashMap<(usize, usize), i32> = std::collections::HashMap::new();
    let mut guard_curr_pos = guard_starting_pos;
    let mut guard_curr_direction = guard_starting_direction;
    loop {
        if !visited_positions.contains_key(&guard_curr_pos) {
            visited_positions.insert(guard_curr_pos, guard_curr_direction);
        } else {
            if visited_positions.get(&guard_curr_pos).unwrap() == &guard_curr_direction {
                return true;
            }
        }

        let (guard_new_pos, guard_new_direction, valid) =
            get_next_step_path(&manufacturing_lab, guard_curr_pos, guard_curr_direction);
        if valid {
            guard_curr_direction = guard_new_direction;
            guard_curr_pos = guard_new_pos;
        } else {
            break;
        }
    }

    false
}

fn add_obstacle_and_count_loops(
    manufacturing_lab: &mut Vec<Vec<i32>>,
    guard_starting_direction: i32,
    guard_starting_pos: (usize, usize),
    path: Vec<(usize, usize)>,
) -> i32 {
    let mut count = 0;
    let mut obstacle_map = HashSet::new();
    // Iterate over the path
    for key in path.iter() {
        if key.0 == guard_starting_pos.0 && key.1 == guard_starting_pos.1 {
            continue;
        }
        if manufacturing_lab[key.0][key.1] == 0 {
            manufacturing_lab[key.0][key.1] = 1;
            if detect_loop(
                manufacturing_lab,
                guard_starting_direction,
                guard_starting_pos,
            ) {
                count += 1;
                obstacle_map.insert(key);
            }
            manufacturing_lab[key.0][key.1] = 0;
        }
    }

    count
}

fn simulate_guard_path_and_count_visited(
    manufacturing_lab: &Vec<Vec<i32>>,
    guard_starting_pos: (usize, usize),
    guard_starting_direction: i32,
) -> (i32, Vec<(usize, usize)>) {
    // Simulate guard path
    // Anytime the guard encounters an obstacle, change direction clockwise
    let mut count = 0;
    let mut guard_curr_pos = guard_starting_pos;
    let mut guard_curr_direction = guard_starting_direction;
    let mut visited_positions = Vec::new();

    loop {
        if !visited_positions.contains(&guard_curr_pos) {
            count += 1;
            visited_positions.push(guard_curr_pos);
        }

        let (guard_new_pos, guard_new_direction, valid) =
            get_next_step_path(&manufacturing_lab, guard_curr_pos, guard_curr_direction);
        if valid {
            guard_curr_direction = guard_new_direction;
            guard_curr_pos = guard_new_pos;
        } else {
            break;
        }
    }

    (count, visited_positions)
}
