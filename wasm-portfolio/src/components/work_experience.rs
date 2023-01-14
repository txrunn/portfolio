use yew::prelude::*;

pub struct Props {
    pub work_experiences: Vec<WorkExperience>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            work_experiences: vec![],
        }
    }
}

#[function_component]
pub fn WorkExperienceComponent(work_experience: Vec<WorkExperience>) -> Html {
    html! {
        <> {
            for work_experience.iter().map(|work_experience| {
                html! {
                    <div class="work-experience">
                        <h3>{ work_experience.title }</h3>
                        <h4>{ work_experience.company }</h4>
                        <h5>{ work_experience.location }</h5>
                        <h5>{ work_experience.date }</h5>
                        <p>{ work_experience.description }</p>
                    </div>
                }
            })
        }
        </>
    }
}