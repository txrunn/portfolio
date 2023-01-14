use yew::prelude::*;

#[derive(Clone, Properties)]
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