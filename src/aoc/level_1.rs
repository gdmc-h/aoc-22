use crate::helpers;

pub fn solve() -> (i32, i32) {
    let elfs_input = helpers::load_input::select(1).unwrap();
    let mut elfs_kcal = elfs_input.split("\n\n")
        .map(|every_kcal| {
            every_kcal
                .split('\n') // found out that this split can be replaced by split_whitespace()
                             // not going to use it as it was not part of the original solution.
                             // still learned something new :D
                .fold(0, |acc, kcal| {
                    kcal.parse::<i32>().unwrap_or(0) + acc
                }) 
        })
        .collect::<Vec<i32>>();

    // AAAAAHHHHHHH, MUTABILITYYYYYYYYYY!!! THE HORROR!!!
    elfs_kcal.sort();
    elfs_kcal.reverse();

    (
        elfs_kcal.get(0).unwrap().to_owned(),
        elfs_kcal.get(0..3).unwrap().iter().sum()
    )
}

