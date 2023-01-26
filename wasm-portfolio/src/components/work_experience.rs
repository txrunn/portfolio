use yew::prelude::*;
use serde::{Serialize, Deserialize};

// Create a struct for each work experience
// Work experiences will be a vector of work_experiences read from a json file
#[derive(Clone, Properties, Serialize, Deserialize)]
pub struct work_experience {
    pub title: String,
    pub company: String,
    pub location: String,
    pub date: String,
    pub description: String,
}

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
            for work_experience.iter().map(|work_experience| { // iterate over the work experience vector and create a div for each work experience
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