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

fn resolve_pair_value(pair: &Pair, battery_position: u32) -> u64 {
    (10 as u64).pow(battery_position - 1) * pair.1 as u64
}

fn find_pairs_in_banks(banks: Vec<Vec<u32>>) -> u64 {
    let mut sum: u64 = 0;
    const BATTERIES_NEEDED: u16 = 12;

    for bank in banks {
        let mut batteries_acq: u16 = 0;
        let element_count = (bank.len() + 1) as u16;
        let mut i: u16 = 0;
        while batteries_acq != BATTERIES_NEEDED {
            let batteries_left = BATTERIES_NEEDED - batteries_acq;
            let j = element_count - (i as u16) - batteries_left;
            let pair = find_max_num_in_bank(&bank, i as usize, (j + i) as usize);
            i = pair.0 as u16 + 1;
            batteries_acq += 1;
            sum += resolve_pair_value(&pair, batteries_left as u32);
        }
    }
    return sum;
}

fn main() {
    let lines = read_file_to_lines("input.txt");
    let sum = find_pairs_in_banks(string_banks_to_int(lines));
    println!("{sum}");
}
