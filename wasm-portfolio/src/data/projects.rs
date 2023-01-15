use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Debug, Clone)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub github: String,
    pub demo: String,
}

pub fn read_projects(path: &str) -> Vec<Project> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let projects: Vec<Project> = serde_json::from_str(&contents).unwrap();
    projects
}