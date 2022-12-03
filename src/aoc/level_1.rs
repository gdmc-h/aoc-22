use crate::helpers;

pub fn solve() -> (i32, i32) {
    let elfs_input = helpers::load_input::select(1).unwrap();
    let mut elfs_kcal = elfs_input.split("\n\n")
        .map(|every_kcal| {
            every_kcal
                .split_whitespace()
                .fold(0, |acc, kcal| {
                    kcal.parse::<i32>().unwrap_or(0) + acc
                }) 
        })
        .collect::<Vec<i32>>();

    // AAAAAHHHHHHH, MUTABILITYYYYYYYYYY!!! THE HORROR!!!
    elfs_kcal.sort();
    elfs_kcal.reverse();

    (
        *elfs_kcal.first().unwrap(),
        elfs_kcal.get(..3).unwrap().iter().sum()
    )
}

