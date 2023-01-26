use yew::prelude::*;
use serde::{Serialize, Deserialize};

// Create a struct for the certification component
// Certifications will be a vector of Certifications read from a json file
#[derive(Clone, Properties, Serialize, Deserialize)]
pub struct Certifications {
    pub title: String,
    pub description: String,
}

#[function_component]
pub fn CertificationsComponent(certifications: Certifications) -> Html {
    html! {
        <div class="certifications">
            <h3>{ certifications.title }</h3>
            <p>{ certifications.description }</p>
        </div>
    }
}