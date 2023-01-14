use yew::prelude::*;

#[derive(Clone, Properties)]
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