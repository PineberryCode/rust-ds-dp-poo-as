use chrono::{NaiveDate};

#[derive(Debug)]
pub struct Target {
    id: char,
    code_security: u8
}

#[derive(Debug)]
pub struct Person {
    id: char,
    targets: Vec<Target>,
    name: String,
    lastname: String,
    birthdate: NaiveDate
}

pub fn add_target(id: char, code: u8) -> Vec<Target> {
    let target = Target {id: id, code_security: code};

    let mut vector = Vec::new();
    vector.push(target);

    vector
}