use util::read_file_to_matrix;
const PAPER_ROLL: char = '@';
const MAX_PAPERS_IN_RANGE: u32 = 4;
const SPIRAL_WIDTH: u8 = 1;

fn count_paper_rolls_in_spiral(
    matrix: &Vec<Vec<char>>,
    position: (isize, isize),
    spiral_width: isize,
) -> u32 {
    let bounds_guarded_paperoll_check = |x: &isize, y: &isize, counter: &mut u32| {
        if *x >= 0 && *y >= 0 && *x < matrix.len() as isize && *y < matrix[0].len() as isize {
            if matrix[*x as usize][*y as usize] == PAPER_ROLL {
                *counter += 1;
            }
        }
    };
    let mut counter: u32 = 0;
    for spiral_walker in 1..spiral_width + 1 {
        let mut i = position.0 - spiral_walker;
        let mut j = position.1 - spiral_walker;
        //row above
        for _ in 0..(2 * spiral_walker) as isize {
            bounds_guarded_paperoll_check(&i, &j, &mut counter);
            j += 1;
        }
        //column to the right
        for _ in 0..(2 * spiral_walker) as isize {
            bounds_guarded_paperoll_check(&i, &j, &mut counter);
            i += 1;
        }
        //row below
        for _ in 0..(2 * spiral_walker) as isize {
            bounds_guarded_paperoll_check(&i, &j, &mut counter);
            j -= 1;
        }
        //column to the left
        for _ in 0..(2 * spiral_walker) as isize {
            bounds_guarded_paperoll_check(&i, &j, &mut counter);
            i -= 1;
        }
    }
    return counter;
}
fn count_accessible_paper_rolls(matrix: &mut Vec<Vec<char>>) -> u32 {
    let row_count = matrix.len();
    let column_count = matrix[0].len();
    let mut accessible_paper_rolls = 0;
    loop {
        let mut roll_removed = false;
        for i in 0..row_count {
            for j in 0..column_count {
                if matrix[i][j] != PAPER_ROLL {
                    continue;
                }
                if count_paper_rolls_in_spiral(
                    matrix,
                    (i as isize, j as isize),
                    SPIRAL_WIDTH as isize,
                ) < MAX_PAPERS_IN_RANGE
                {
                    accessible_paper_rolls += 1;
                    matrix[i][j] = '.';
                    roll_removed = true;
                }
            }
        }
        if !roll_removed {
            break;
        }
    }
    accessible_paper_rolls
}

fn main() {
    let input_file = "input.txt";
    let mut matrix = read_file_to_matrix::<char>(input_file, |c| c);
    let accessible_rolls = count_accessible_paper_rolls(&mut matrix);
    println!("{accessible_rolls}");
}
