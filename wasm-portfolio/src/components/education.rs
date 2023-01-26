use yew::prelude::*;
use serde::{Serialize, Deserialize};

// Create a struct for the education component
// Education will be a vector of Education read from a json file
#[derive(Clone, Properties, Serialize, Deserialize)]
pub struct Education {
    pub school: String,
    pub location: String,
    pub degree: String,
    pub date: String,
    pub description: String,
}

#[function_component]
pub fn EducationComponent(education: Education) -> Html {
    html! {
        <div class="education">
            <h3>{ education.school }</h3>
            <h4>{ education.location }</h4>
            <h5>{ education.degree }</h5>
            <h5>{ education.date }</h5>
            <p>{ education.description }</p>
        </div>
    }
}