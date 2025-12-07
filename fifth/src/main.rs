use std::cmp::max;
use util::read_file_to_lines;
#[derive(Debug)]
struct Interval(u128, u128);

fn count_fresh_ingredients(mut intervals: Vec<Interval>) -> u128 {
    intervals.sort_by_key(|i| i.0);

    let mut merged: Vec<Interval> = Vec::new();
    println!("{intervals:?}");
    for interval in intervals {
        if let Some(last) = merged.last_mut() {
            println!("{interval:?} {last:?}");
            if interval.0 <= last.1 + 1 {
                last.1 = max(last.1, interval.1);
                continue;
            }
        }
        merged.push(interval);
    }
    println!("{merged:?}");
    merged.iter().map(|i| i.1 - i.0 + 1).sum()
}

fn parse_lines_to_intervals(lines: Vec<String>) -> Vec<Interval> {
    let mut intervals = vec![];
    const IDS_DELIMITER: &str = "";
    for i in 0..lines.len() {
        if lines[i] == IDS_DELIMITER {
            break;
        }
        let parts = lines[i].split("-").collect::<Vec<_>>();
        intervals.push(Interval(
            parts[0].parse::<u128>().unwrap(),
            parts[1].parse::<u128>().unwrap(),
        ));
    }
    intervals
}

fn main() {
    let lines = read_file_to_lines("input.txt");
    let intervals = parse_lines_to_intervals(lines);
    let fresh_count = count_fresh_ingredients(intervals);
    println!("Fresh: {fresh_count}");
}
