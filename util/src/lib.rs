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

pub fn read_file_to_matrix<T>(path: &str, mapping_function: fn(char) -> T) -> Vec<Vec<T>> {
    let mut matrix = vec![];
    let content = read_to_string(path).expect("where input brotha");
    let lines = content.lines();
    for line in lines {
        let mut row = vec![];
        line.chars().for_each(|c| row.push(mapping_function(c)));
        matrix.push(row);
    }
    matrix
}
