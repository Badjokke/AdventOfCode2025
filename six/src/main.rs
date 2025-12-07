use util::read_file_to_lines;
const MULTIPLICATION_OPERATION: &str = "*";
const ADDITION_OPERATION: &str = "+";

#[derive(Debug, PartialEq)]
enum Operator {
    MULTIPLICATION,
    ADDITION,
    NONE,
}

#[derive(Debug)]
struct Operation(Vec<u128>, Operator);
fn str_to_operator(str: &str) -> Operator {
    match str {
        MULTIPLICATION_OPERATION => Operator::MULTIPLICATION,
        ADDITION_OPERATION => Operator::ADDITION,
        _ => Operator::NONE,
    }
}

fn collect_numbers_from_vertical_list(path: &str) -> Vec<Operation> {
    let lines = read_file_to_lines(path);
    let mut result: Vec<Operation> = vec![];
    let row_count = lines.len();
    for i in 0..row_count - 1 {
        lines[i].split_whitespace().enumerate().for_each(|(i, x)| {
            if i == result.len() {
                result.push(Operation(vec![], Operator::NONE));
            }
            result[i].0.push(x.trim().parse::<u128>().unwrap());
        });
    }
    lines[row_count - 1]
        .split_whitespace()
        .map(|op| op.trim())
        .enumerate()
        .for_each(|(i, op)| result[i].1 = str_to_operator(op));
    result
}

fn execute_operation(operation: &Operation) -> u128 {
    if operation.1 == Operator::ADDITION {
        return operation.0.iter().sum();
    }
    return operation.0.iter().product();
}

fn main() {
    let operations = collect_numbers_from_vertical_list("input.txt");
    let sum = operations.iter().map(execute_operation).sum::<u128>();
    println!("Final sum : {sum}");
}
