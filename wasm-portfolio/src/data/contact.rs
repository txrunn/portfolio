use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Debug, Clone)]
pub struct Contact {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub github: String,
    pub linkedin: String,
}

pub fn read_contact(path: &str) -> Vec<Contact> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let contact: Vec<Contact> = serde_json::from_str(&contents).unwrap();
    contact
}