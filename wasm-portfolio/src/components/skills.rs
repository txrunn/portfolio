use yew::prelude::*;
use serde::{Serialize, Deserialize};

// Create a struct for the skills component
// Skills will be a vector of Skills read from a json file
#[derive(Clone, Properties, Serialize, Deserialize)]
pub struct Skills {
    pub title: String,
    pub description: String,
}

#[function_component]
pub fn SkillsComponent(skills: Skills) -> Html {
    html! {
        <div class="skills">
            <h3>{ skills.title }</h3>
            <p>{ skills.description }</p>
        </div>
    }
}