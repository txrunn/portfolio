use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Debug, Clone)]
pub struct Certification {
    pub title: String,
    pub description: String,
}

pub fn read_certifications(path: &str) -> Vec<Certification> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let certifications: Vec<Certification> = serde_json::from_str(&contents).unwrap();
    certifications    
}