use crate::helpers::load_input;

pub fn solve() -> (i32, i32) {
    let priorities = ('a'..='z').chain('A'..='Z').into_iter().collect::<Vec<char>>();
    let level = load_input::select(3).unwrap();
    let lines_part_1 = level.lines();
    let lines_vec = lines_part_1.clone().map(|l| l.to_string()).collect::<Vec<String>>();

    let lines_part_2 = (0..lines_vec.len() / 3).map(|idx| {
        let current_index = idx * 3;

        lines_vec.get(current_index..current_index+3).unwrap().to_owned()
    });

    fn get_index(i: &[char], search: &char) -> i32 {
        i.iter().position(|&r| r == *search).unwrap() as i32
    }

    let get_single_sum = |source: &[char], comp: &[String]| {
        let t = source
            .iter()
            .fold(vec![], |mut acc, item| {
                let filter_me_daddy = comp.iter().filter(|s| s.contains(*item));

                if filter_me_daddy.count() == comp.len() && !acc.contains(item) {
                    acc.push(*item);
                }

                acc
            });

        t.iter().map(|item| get_index(&priorities, item) + 1).sum::<i32>()
    };

    let part_1: i32 = 
        lines_part_1.map(|sack| {
            let items_in_sack = sack.chars();
            let items_len = items_in_sack.clone().count() / 2;
            let comp1 = items_in_sack.clone().collect::<Vec<char>>().get(..items_len).unwrap().to_owned();
            let comp2 = items_in_sack.clone().collect::<Vec<char>>().get(items_len..).unwrap().to_owned();

            get_single_sum(&comp1, &[comp2.iter().collect()])
        }).sum();

    let part_2: i32 = 
        lines_part_2.map(|group| {
            let head = &group.first().unwrap().chars().collect::<Vec<char>>();
            let tail = &group[1..];
        
            get_single_sum(head, tail)
        }).sum();


    (part_1, part_2)
}
