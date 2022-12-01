use std::fs;

pub fn select(level: i8) -> Result<String, ()> {
    let level_path = format!("./src/aoc/inputs/level_{}.txt", level);
    match fs::read_to_string(&level_path) {
        Ok(file) => Ok(file),
        Err(_) => panic!("Level input not found {}", &level_path)
    }
}
