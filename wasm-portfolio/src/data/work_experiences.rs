use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Debug, Clone)]
pub struct WorkExperience {
    pub title: String,
    pub company: String,
    pub location: String,
    pub date: String,
    pub description: String,
}

pub fn read_work_experiences(path: &str) -> Vec<WorkExperience> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let work_experiences: Vec<WorkExperience> = serde_json::from_str(&contents).unwrap();
    work_experiences
}
