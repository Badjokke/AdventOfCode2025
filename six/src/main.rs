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

fn collect_number_from_column(number_index: usize, lines: &Vec<String>) -> u128 {
    let mut result = 0_u128;
    let n = lines.len() - 1;
    let mut exp_walker = n;
    let mut i = n;
    while i != 0 {
        i -= 1;
        let mut chars = lines[i].chars();
        let char = chars.nth(number_index).unwrap();
        if char.is_whitespace() {
            continue;
        }
        let digit = char.to_digit(10).unwrap() as u128;
        result += digit * 10_u128.pow((n - exp_walker) as u32);
        exp_walker -= 1;
    }
    result
}

fn collect_numbers_from_vertical_list(path: &str) -> Vec<Operation> {
    let lines = read_file_to_lines(path);
    let mut result: Vec<Operation> = vec![];
    let row_count = lines.len();
    let mut walker = lines[row_count - 2].len();
    let mut result_pointer = 0;
    while walker != 0 {
        walker -= 1;
        if result_pointer == result.len() {
            result.push(Operation(vec![], Operator::NONE));
        }
        let number = collect_number_from_column(walker, &lines);
        if number == 0 {
            result_pointer += 1;
            continue;
        }
        result[result_pointer].0.push(number);
    }

    lines[row_count - 1]
        .split_whitespace()
        .map(|op| op.trim())
        .enumerate()
        .for_each(|(i, op)| result[result_pointer - i].1 = str_to_operator(op));
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
