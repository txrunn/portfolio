use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Debug, Clone)]
pub struct Education {
    pub school: String,
    pub degree: String,
    pub location: String,
    pub date: String,
    pub description: String,
}

pub fn read_education(path: &str) -> Vec<Education> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let education: Vec<Education> = serde_json::from_str(&contents).unwrap();
    education
}