use std::cmp;

use crate::helpers::load_input;

pub fn solve() -> (i32, i32){
    let puzzle = load_input::select(4).unwrap();
    let input = puzzle.split_whitespace().collect::<Vec<&str>>();

    let (contains_count, overlap_counts) = 
        input.iter()
            .fold((0,0), |acc, pair_raw| {
                let r = pair_raw.split(|d| d == '-' || d == ',').map(|e| e.parse::<i32>().unwrap_or(0)).collect::<Vec<i32>>();
                let (head, tail) = r.split_at(2);
                let (mut c_c, mut o_c) = acc;

                let get_min_max = |x: &[i32], y: &[i32]| {
                    (
                        (*x.first().unwrap(), *x.last().unwrap()),
                        (*y.first().unwrap(), *y.last().unwrap())
                    )
                };

                let contains = |x: &[i32], y: &[i32]| {
                    let ((x1, x2), (y1, y2)) = get_min_max(x, y);

                    x1 <= y1 && x2>= y2
                };

                let overlaps = |x: &[i32], y: &[i32]| {
                    let ((x1, x2), (y1, y2)) = get_min_max(x, y);

                    cmp::max(x1, y1) <= cmp::min(x2, y2)
                };

                if contains(head, tail) || contains(tail, head) { c_c += 1; } 
                if overlaps(head, tail) || overlaps(tail, head) { o_c += 1; } 

                (c_c, o_c)
            });


    (contains_count, overlap_counts)
}
