use yew::prelude::*;
use components::{
    work_experience, 
    education, 
    skills, 
    certifications, 
    projects, 
    contact
};

// Props define the data that will be passed to the app component
#[derive(Clone, Properties)]
pub struct Props {
    pub work_experiences: Vec<work_experience::WorkExperience>,
    pub education: Vec<education::Education>,
    pub skills: Vec<skills::Skills>,
    pub certifications: Vec<certifications::Certifications>,
    pub projects: Vec<projects::Projects>,
    pub contact: Vec<contact::Contact>,
}

// App component will be the parent component for all other components and will pass props to them as needed
#[function_component(App)]
pub fn app(props: &Props) -> Html {
    let Props { // destructure props
        work_experiences,
        education,
        skills,
        certifications,
        projects,
        contact,
    } = props;
    
    // return the html for the app component
    html! { 
        <>
            // pass props to each component
            <work_experience::WorkExperienceComponent work_experiences=work_experiences />
            <education::EducationComponent education=education />
            <skills::SkillsComponent skills=skills />
            <certifications::CertificationsComponent certifications=certifications />
            <projects::ProjectsComponent projects=projects />
            <contact::ContactComponent contact=contact />
        </>
    }
}

/*
TODO: Finish data module wtih read functions for all json
TODO: Finish app module with props for all
TODO: Create json files for all
TODO: Create UI
*/
fn main() {    
    // initialize the app component and render it to the DOM using the yew::Renderer trait and the render function
    yew::Renderer::<App>::new().render();
}