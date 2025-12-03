use std::fs::read_to_string;

pub fn read_file_to_lines(path: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::with_capacity(100);
    let content = read_to_string(path).expect("where input brotha");
    content
        .lines()
        .for_each(|line| result.push(String::from(line)));
    result
}

pub fn read_file_delimited(path: &str, delimiter: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::with_capacity(100);
    let content = read_to_string(path).expect("where input brotha");
    content
        .split(delimiter)
        .for_each(|token| result.push(String::from(token)));
    result
}
