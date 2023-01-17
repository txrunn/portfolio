use yew::prelude::*;

mod components;
mod data;

#[function_component(App)]
fn app() -> Html {
    let work_experiences = data::read_work_experiences("work_experiences.json");
    let education = data::read_education("education.json");
    let skills = data::read_skills("skills.json");
    let certifications = data::read_certifications("certifications.json");
    let projects = data::read_projects("projects.json");
    let contact = data::read_contact("contact.json");
    
    html! { 
        <>
            <components::work_experience::WorkExperienceComponent />
            <components::education::EducationComponent />
            <components::skills::SkillsComponent />
            <components::certifications::CertificationsComponent />
            <components::projects::ProjectsComponent />
            <components::contact::ContactComponent />
        </>
    }
}

// TODO: Finish data module wtih read functions for all json
// TODO: Finish app module with props for all
// TODO: Create json files for all
// TODO: Create UI
fn main() {    
    yew::Renderer::<App>::new().render();
}