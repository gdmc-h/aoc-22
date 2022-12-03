use crate::helpers::load_input;

pub fn solve() -> (i32, i32) {
    let priorities = ('a'..='z').chain('A'..='Z').into_iter().collect::<Vec<char>>();
    let level = load_input::select(3).unwrap();
    let lines = Box::new(level.lines().map(|l| l.to_string()));

    fn get_index(i: &[char], search: &char) -> i32 {
        i.iter().position(|&r| r == *search).unwrap() as i32
    }

    let get_single_sum = |source: String, comp: &[String]| {
        let t = source
            .chars()
            .collect::<Vec<char>>()
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
        lines.clone()
            .map(|sack| {
                let half_len = sack.len() / 2;
                let (comp1, comp2) = sack.split_at(half_len);

                get_single_sum(comp1.to_string(), &[comp2.to_string()])
            }).sum();

    let part_2: i32 = 
        lines.array_chunks::<3>()
            .map(|group| {
                let head = group.first().unwrap().to_owned();
                let tail = &group[1..];
        
                get_single_sum(head, tail)
            }).sum();


    (part_1, part_2)
}
