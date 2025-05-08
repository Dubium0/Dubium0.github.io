// src/components/experience_item.rs
use yew::prelude::*;

// --- Enum for Experience Category ---
#[derive(Clone, PartialEq)]
pub enum ExperienceCategory {
    Work,
    Extracurricular,
    // Add other categories if needed
}

// --- Data Structure for a Single Experience ---
#[derive(Clone, PartialEq)]
pub struct ExperienceData {
    pub title: String,
    pub company: String,        // Or Organization/Club Name
    pub company_url: Option<String>,
    pub dates: String,
    pub location: String,
    pub description_points: Vec<String>,
    pub technologies: Option<Vec<String>>,
    pub category: ExperienceCategory, // <-- Added category field
}

// --- Props for ExperienceItem Component ---
#[derive(Properties, Clone, PartialEq)]
pub struct ExperienceItemProps {
    pub experience: ExperienceData,
}

// --- ExperienceItem Component ---
#[function_component(ExperienceItem)]
pub fn experience_item(props: &ExperienceItemProps) -> Html {
    let exp = &props.experience;

    let company_html = if let Some(url) = &exp.company_url {
        html! { <a href={url.clone()} target="_blank" rel="noopener noreferrer">{ &exp.company }</a> }
    } else {
        html! { { &exp.company } }
    };

    html! {
        <div class="experience-item">
            <div class="experience-header">
                <h3 class="experience-title">{ &exp.title }</h3>
                <p class="experience-company">{ company_html } {" - "} {&exp.location}</p>
                <p class="experience-dates">{ &exp.dates }</p>
            </div>
            <ul class="experience-description">
                { for exp.description_points.iter().map(|point| html!{ <li>{ point }</li> }) }
            </ul>
            {
                if let Some(techs) = &exp.technologies {
                    if !techs.is_empty() {
                        html! {
                            <p class="experience-technologies">
                                <strong>{"Technologies: "}</strong>
                                { techs.join(", ") }
                            </p>
                        }
                    } else {
                        html! {}
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
