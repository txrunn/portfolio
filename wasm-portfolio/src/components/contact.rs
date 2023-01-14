use yew::prelude::*;

#[derive(Clone, Properties)]
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