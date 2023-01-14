use yew::prelude::*;

#[derive(Clone, Properties)]
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