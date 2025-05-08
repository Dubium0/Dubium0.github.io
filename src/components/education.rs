// src/components/education.rs
use yew::prelude::*;
// Import the EducationData struct and EducationItem component from the sibling module
use super::education_item::{EducationData, EducationItem};

// --- Education Section Component ---
#[function_component(Education)]
pub fn education() -> Html {
    // --- Define your education data here ---
    let education_list: Vec<EducationData> = vec![
        EducationData {
            institution: "Ozyegin University".to_string(), // Replace with your institution
            institution_url: Some("https://www.ozyegin.edu.tr/en".to_string()), // Replace
            degree: "Bachelor of Science Degree in Computer Science".to_string(), // Replace
            dates: "September 2020 - January 2026 (Expected)".to_string(), // Replace
            location: "Istanbul, Turkey".to_string(),
            details: Some(vec![
                "Current GPA: 3.09 / 4.00".to_string(), // Replace
                "Relevant Coursework: Data Structures, Algorithms, Operating Systems, Game Development, Advance C++".to_string(),
                // "Thesis: Title of Your Thesis (if applicable)".to_string(),
            ]),
            logo_url: Some("images/ozyegin_logo.png".to_string()), // Example: Place 'itu_logo.png' in 'assets' folder
        },
        EducationData {
            institution: "Terme Science High School".to_string(), // Replace with your institution
            institution_url: Some("https://termefenlisesi.meb.k12.tr/".to_string()), // Replace
            degree: "".to_string(),
            dates: "September 2016 - June 2020".to_string(), // Replace
            location: "Samsun, Turkey".to_string(),
            details: None,
            logo_url: None,
        },
        // Add more EducationData structs here for other degrees or institutions
        // EducationData {
        //     institution: "Another University/High School".to_string(),
        //     institution_url: None,
        //     degree: "Degree or Diploma".to_string(),
        //     dates: "Year - Year".to_string(),
        //     location: "City, Country".to_string(),
        //     details: None,
        //     logo_url: None,
        // },
    ];

    html! {
        <section id="education" class="content-section education-section">
            <h2>{ "Education" }</h2>
            <div class="education-list">
                {
                    education_list.iter().map(|edu_item| {
                        html! {
                            <EducationItem education={edu_item.clone()} />
                        }
                    }).collect::<Html>()
                }
            </div>
        </section>
    }
}
