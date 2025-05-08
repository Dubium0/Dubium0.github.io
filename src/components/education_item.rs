// src/components/education_item.rs
use yew::prelude::*;

// --- Data Structure for a Single Education Entry ---
#[derive(Clone, PartialEq)]
pub struct EducationData {
    pub institution: String,      // e.g., "Istanbul Technical University"
    pub institution_url: Option<String>, // Optional link to institution website
    pub degree: String,           // e.g., "Bachelor of Science in Computer Science"
    pub dates: String,            // e.g., "Sep 2020 - June 2024 (Expected)"
    pub location: String,         // e.g., "Istanbul, Turkey"
    pub details: Option<Vec<String>>, // Optional: e.g., GPA, relevant coursework, thesis title
    pub logo_url: Option<String>, // Optional: URL to the institution's logo
}

// --- Props for EducationItem Component ---
#[derive(Properties, Clone, PartialEq)]
pub struct EducationItemProps {
    pub education: EducationData,
}

// --- EducationItem Component ---
#[function_component(EducationItem)]
pub fn education_item(props: &EducationItemProps) -> Html {
    let edu = &props.education;

    let institution_html = if let Some(url) = &edu.institution_url {
        html! { <a href={url.clone()} target="_blank" rel="noopener noreferrer">{ &edu.institution }</a> }
    } else {
        html! { { &edu.institution } }
    };

    html! {
        <div class="education-item">
            {
                if let Some(logo_src) = &edu.logo_url {
                    html! { <img src={logo_src.clone()} alt={format!("{} logo", edu.institution)} class="education-logo" /> }
                } else {
                    html! {}
                }
            }
            <div class="education-content">
                <div class="education-header">
                    <h3 class="education-institution">{ institution_html }</h3>
                    <p class="education-degree">{ &edu.degree }</p>
                    <p class="education-dates">{ &edu.dates } {" - "} {&edu.location}</p>
                </div>
                {
                    if let Some(details_list) = &edu.details {
                        if !details_list.is_empty() {
                            html! {
                                <ul class="education-details">
                                    { for details_list.iter().map(|point| html!{ <li>{ point }</li> }) }
                                </ul>
                            }
                        } else {
                            html!{}
                        }
                    } else {
                        html!{}
                    }
                }
            </div>
        </div>
    }
}
