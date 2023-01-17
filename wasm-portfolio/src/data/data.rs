use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Debug, Clone)]
pub struct Certification {
    pub title: String,
    pub description: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Contact {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub github: String,
    pub linkedin: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Education {
    pub school: String,
    pub degree: String,
    pub location: String,
    pub date: String,
    pub description: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub github: String,
    pub demo: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Skill {
    pub name: String,
    pub level: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WorkExperience {
    pub title: String,
    pub company: String,
    pub location: String,
    pub date: String,
    pub description: String,
}

pub fn read_certifications(path: &str) -> Vec<Certification> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let certifications: Vec<Certification> = serde_json::from_str(&contents).unwrap();
    certifications    
}

pub fn read_contact(path: &str) -> Vec<Contact> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let contact: Vec<Contact> = serde_json::from_str(&contents).unwrap();
    contact
}

pub fn read_education(path: &str) -> Vec<Education> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let education: Vec<Education> = serde_json::from_str(&contents).unwrap();
    education
}

pub fn read_projects(path: &str) -> Vec<Project> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let projects: Vec<Project> = serde_json::from_str(&contents).unwrap();
    projects
}

pub fn read_skills(path: &str) -> Vec<Skill> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let skills: Vec<Skill> = serde_json::from_str(&contents).unwrap();
    skills
}

pub fn read_work_experiences(path: &str) -> Vec<WorkExperience> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let work_experiences: Vec<WorkExperience> = serde_json::from_str(&contents).unwrap();
    work_experiences
}
