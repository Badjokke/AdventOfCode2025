use util::read_file_to_lines;
#[derive(Debug)]
struct Interval(u128, u128);

fn count_fresh_ingredients(intervals: &Vec<Interval>, ids: &Vec<u128>) -> u128 {
    let mut counter = 0_u128;
    for id in ids {
        for interval in intervals {
            if interval.0 <= *id && *id <= interval.1 {
                counter += 1;
                break;
            }
        }
    }
    return counter;
}

fn parse_lines(lines: Vec<String>) -> (Vec<Interval>, Vec<u128>) {
    let mut intervals = vec![];
    let mut ids = vec![];
    const ID_DELIMITER: &str = "";
    let mut id_start_index = 0;
    for i in 0..lines.len() {
        if lines[i] == ID_DELIMITER {
            id_start_index = i + 1;
            break;
        }
        let parts = lines[i].split("-").collect::<Vec<_>>();
        intervals.push(Interval(
            parts[0].parse::<u128>().unwrap(),
            parts[1].parse::<u128>().unwrap(),
        ));
    }
    for i in id_start_index..lines.len() {
        ids.push(lines[i].parse::<u128>().unwrap());
    }

    (intervals, ids)
}

fn main() {
    let lines = read_file_to_lines("input.txt");
    let (intervals, ids) = parse_lines(lines);
    let fresh_count = count_fresh_ingredients(&intervals, &ids);
    println!("Fresh: {fresh_count}");
}
