use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Debug, Clone)]
pub struct Skill {
    pub name: String,
    pub level: String,
}

pub fn read_skills(path: &str) -> Vec<Skill> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let skills: Vec<Skill> = serde_json::from_str(&contents).unwrap();
    skills
}