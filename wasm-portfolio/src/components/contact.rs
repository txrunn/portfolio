use yew::prelude::*;
use serde::{Serialize, Deserialize};

// Create a struct for the contact component
// Contact will be read from a json file
#[derive(Clone, Properties, Serialize, Deserialize)]
pub struct Contact {
    pub email: String,
    pub phone: String,
    pub github: String,
    pub linkedin: String,
}

#[function_component]
pub fn ContactComponent(contact: Contact) -> Html {
    html! {
        <div class="contact">
            <h3>{ contact.email }</h3>
            <h3>{ contact.phone }</h3>
            <h3>{ contact.github }</h3>
            <h3>{ contact.linkedin }</h3>
        </div>
    }
}