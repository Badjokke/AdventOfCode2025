use util::read_file_delimited;
#[derive(Debug)]
struct Interval(i64, i64);
fn collect_intervals(intervals: Vec<String>) -> Vec<Interval> {
    let mut result = vec![];
    let interval_delimiter = "-";
    for interval in intervals {
        let tmp = interval.split(interval_delimiter).collect::<Vec<_>>();
        result.push(Interval(
            tmp[0].parse::<i64>().unwrap(),
            tmp[1].parse::<i64>().unwrap(),
        ));
    }
    result
}

fn is_invalid_id(id: i64) -> bool {
    let tmp = id.to_string();
    let mut window: Vec<u8> = vec![];
    let digit_bytes = tmp.as_bytes();
    window.push(digit_bytes[0]);
    for i in 0..tmp.len() / 2 {
        if is_window_sequence(&window, digit_bytes, i + 1) {
            return true;
        }
        window.push(digit_bytes[i + 1]);
    }
    return false;
}

fn is_window_sequence(window: &Vec<u8>, digit: &[u8], start_index: usize) -> bool {
    let mut walker = 0;
    let window_size = window.len();
    if window_size % 2 == 0 && digit.len() % 2 != 0 {
        return false;
    }
    for i in start_index..digit.len() {
        if window[walker] != digit[i] {
            return false;
        }
        walker = (walker + 1) % window_size;
    }
    return walker == 0;
}

fn find_all_invalid_pairs(intervals: &Vec<Interval>) -> Vec<i64> {
    let mut result = vec![];
    for interval in intervals {
        for i in interval.0..interval.1 + 1 {
            if !is_invalid_id(i) {
                continue;
            }
            result.push(i);
        }
    }
    result
}

fn main() {
    let intervals = collect_intervals(read_file_delimited("input.txt", ","));
    let invalid_ids = find_all_invalid_pairs(&intervals);
    println!("Invalids: {:?}", invalid_ids);
    println!("invalid id sum: {:?}", invalid_ids.iter().sum::<i64>());
}
