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
    if tmp.len() % 2 != 0 {
        return false;
    }
    let halves = tmp.split_at(tmp.len() / 2);
    println!("Halves: {:?}", halves);
    return halves.0 == halves.1;
}

fn find_all_invalid_pairs(intervals: &Vec<Interval>) -> Vec<i64> {
    let mut result = vec![];
    for interval in intervals {
        for i in interval.0..interval.1 {
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
