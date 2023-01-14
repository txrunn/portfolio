use yew::prelude::*;
use components::{
    work_experience, 
    education, 
    skills, 
    certifications, 
    projects, 
    contact
};

#[derive(Clone, Properties)]
pub struct Props {
    pub work_experiences: Vec<work_experience::WorkExperience>,
    pub education: Vec<education::Education>,
    pub skills: Vec<skills::Skills>,
    pub certifications: Vec<certifications::Certifications>,
    pub projects: Vec<projects::Projects>,
    pub contact: Vec<contact::Contact>,
}

#[function_component(App)]
fn app(props: Props) -> Html {
    let Props {
        work_experiences,
        education,
        skills,
        certifications,
        projects,
        contact,
    } = props;
    
    html! { 
        <>
            <work_experience::WorkExperienceComponent work_experiences=work_experiences />
            <education::EducationComponent education=education />
            <skills::SkillsComponent skills=skills />
            <certifications::CertificationsComponent certifications=certifications />
            <projects::ProjectsComponent projects=projects />
            <contact::ContactComponent contact=contact />
        </>
    }
}
