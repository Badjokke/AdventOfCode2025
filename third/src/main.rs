use util::read_file_to_lines;
#[derive(Debug)]
struct Pair(usize, u32);

fn string_banks_to_int(banks: Vec<String>) -> Vec<Vec<u32>> {
    let mut result: Vec<Vec<u32>> = vec![];
    for bank in banks {
        let mut bank_vector = vec![];
        bank.chars()
            .for_each(|c| bank_vector.push(c.to_digit(10).unwrap()));
        result.push(bank_vector);
    }
    return result;
}

fn find_max_num_in_bank(bank: &Vec<u32>, starting_index: usize, end_index: usize) -> Pair {
    let mut max: u32 = bank[starting_index];
    let mut index: usize = starting_index;
    for i in starting_index..end_index {
        if bank[i] > max {
            max = bank[i];
            index = i;
        }
    }
    Pair(index, max)
}
fn resolve_pair_value(first: &Pair, second: &Pair) -> u16 {
    if first.0 < second.0 {
        return (10 * first.1 + second.1) as u16;
    }
    return (10 * second.1 + first.1) as u16;
}
fn find_pairs_in_banks(banks: Vec<Vec<u32>>) -> u16 {
    let mut sum: u16 = 0;
    for bank in banks {
        let mut starting_index: usize = 0;
        let mut end_index: usize = bank.len();

        let first_max = find_max_num_in_bank(&bank, starting_index, end_index);
        if first_max.0 == end_index - 1 {
            end_index -= 1;
        } else {
            starting_index = first_max.0 + 1;
        }
        let second_max = find_max_num_in_bank(&bank, starting_index, end_index);
        sum += resolve_pair_value(&first_max, &second_max)
    }
    return sum;
}
fn main() {
    let lines = read_file_to_lines("input.txt");
    let sum = find_pairs_in_banks(string_banks_to_int(lines));
    println!("{sum}");
}
