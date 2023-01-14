use yew::prelude::*;

#[derive(Clone, Properties)]
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