use yew::prelude::*;
use serde::{Serialize, Deserialize};

// Create a struct for the projects component
// Projects will be a vector of Projects read from a json file
#[derive(Clone, Properties, Serialize, Deserialize)]
pub struct Projects {
    pub title: String,
    pub description: String,
    pub link: String,
}

#[function_component]
pub fn ProjectsComponent(projects: Projects) -> Html {
    html! {
        <div class="projects">
            <h3>{ projects.title }</h3>
            <p>{ projects.description }</p>
        </div>
    }
}