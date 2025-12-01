use std::fs::read_to_string;

#[derive(Debug)]
enum Direction {
    LEFT,
    RIGHT,
}

impl Direction {
    pub fn from_char(c: char) -> Self {
        if c == 'L' {
            return Direction::LEFT;
        }
        return Direction::RIGHT;
    }
}

#[derive(Debug)]
struct Rotation {
    direction: Direction,
    power: i32,
}

fn read_file_to_lines(path: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::with_capacity(100);
    let content = read_to_string(path).expect("where input brotha");
    content
        .lines()
        .for_each(|line| result.push(String::from(line)));
    result
}

fn count_dial_at_zero_occurrences(dial_rotations: &Vec<Rotation>) -> u32 {
    let mut occurrences: u32 = 0;
    let mut position: i32 = 50;
    const MAX_NUM: i32 = 100;
    for rotation in dial_rotations {
        position =
            ((position + get_rotation_direction_and_click_count(rotation)) + MAX_NUM) % MAX_NUM;
        if position == 0 {
            occurrences += 1;
        }
    }
    occurrences
}

fn get_rotation_direction_and_click_count(rotation: &Rotation) -> i32 {
    match rotation.direction {
        Direction::LEFT => -rotation.power,
        Direction::RIGHT => rotation.power,
    }
}

fn lines_to_rotations(file_lines: &mut Vec<String>) -> Vec<Rotation> {
    let mut result: Vec<Rotation> = Vec::with_capacity(100);
    for line in file_lines {
        let direction: char = line.chars().next().unwrap();
        let power: i32 = line.split_off(1).parse::<i32>().unwrap();
        result.push(Rotation {
            direction: Direction::from_char(direction),
            power,
        });
    }
    result
}

fn main() {
    println!("Hello, world!");
    let file_content = &mut read_file_to_lines("input.txt");
    let rotations = lines_to_rotations(file_content);
    let pswd = count_dial_at_zero_occurrences(&rotations);
    println!("{pswd}")
}
