use crate::helpers;

fn get_all_calories(elfs: &str) -> impl Iterator<Item=i32> + '_ {
    elfs
        .split("\n\n")
        .map(|every_kcal| {
            every_kcal
                .split('\n')
                .fold(0, |acc, kcal| {
                    kcal.parse::<i32>().unwrap_or(0) + acc
                })
        })
}

pub async fn level01_part1() -> Result<String, ()> {
    let res = helpers::load_input::select(1).unwrap();
    let elfs_kcal = get_all_calories(&res);

    let split_elfs = elfs_kcal
        .reduce(|acc, kcal| {
            if kcal > acc { kcal } else { acc }
        })
        .unwrap();

    Ok(split_elfs.to_string())
}

pub async fn level01_part2() -> Result<String, ()> {
    let res = helpers::load_input::select(1).unwrap();
    let mut elfs_kcal = get_all_calories(&res).collect::<Vec<i32>>();

    // AAAAAHHHHHHH, MUTABILITYYYYYYYYYY!!! THE HORROR!!!
    elfs_kcal.sort();
    elfs_kcal.reverse();

    let split_elfs = elfs_kcal
        .get(0..2)
        .unwrap()
        .iter()
        .fold(0, |acc, kcal| acc + kcal); // can be refactor with a reduce, but I don't have the
                                          // time to fight with the borrow checker right now.

    Ok(split_elfs.to_string())
}
