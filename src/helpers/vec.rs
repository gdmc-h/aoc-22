pub fn get_index(i: &Vec<&str>, search: &str) -> i32 {
    i.iter().position(|&r| r == search.to_lowercase()).unwrap() as i32
}



