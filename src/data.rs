use std::fs;

pub fn data(day: i8, file: &str) -> String {
    let path = format!("data/{}/{}.txt", day, file);
    fs::read_to_string(path).unwrap()
}