// src/components/experiences.rs
use yew::prelude::*;
// Import the ExperienceData struct, ExperienceCategory enum, and ExperienceItem component
use super::experience_item::{ExperienceData, ExperienceCategory, ExperienceItem};

// --- Experiences Section Component ---
#[function_component(Experiences)]
pub fn experiences() -> Html {
    // --- Define your experience data here ---
    let all_experiences: Vec<ExperienceData> = vec![
        // --- Work Experience ---
        ExperienceData {
            title: "Software Engineer Intern".to_string(),
            company: "TaleWorlds Entertainment".to_string(),
            company_url: Some("https://www.taleworlds.com/".to_string()),
            dates: "June 2024 - September 2024".to_string(),
            location: "Ankara, Turkey".to_string(),
            description_points: vec![
                "Implemented advance rendering features; Transmission, Dispersion, Volume & IOR ,Dithered Transparency to in-house game engine. ".to_string(),
                "Implemented, a tool that automates Blender – Substance Painter – In-House Engine asset export and import operations for artists.".to_string(),
                "Experienced AA game studio workflow".to_string(),
                "Wrote QA tests and worked collaboratively with both QA and Art team".to_string(),
            ],
            technologies: Some(vec!["C++".to_string(), "Premake5".to_string(), "DirectX12".to_string(), 
            "C#".to_string(), "Python".to_string(), "Blender".to_string(), "Jira".to_string(), "Plastic SCM".to_string(), "Visual Studio".to_string()]),

            category: ExperienceCategory::Work, // Assign category
        },

        ExperienceData {
            title: "Game Developer Intern".to_string(),
            company: "Any Games".to_string(),
            company_url: Some("https://www.anygames.io/".to_string()),
            dates: "June 2023- September 2023".to_string(),
            location: "Istanbul, Turkey".to_string(),
            description_points: vec![
                "Designed & Developed modern hybrid-casual games with Unity.".to_string(),
                "Developed multiplayer features with Photon. ".to_string(),
                "Finished 3 different games.".to_string(),
                "Worked colloboratively with art team and leads.".to_string(),
            ],
            technologies: Some(vec!["Unity".to_string(), "Photon".to_string(), "C#".to_string(),]),

            category: ExperienceCategory::Work, // Assign category
        },
        ExperienceData {
            title: "Member - Ozu Racing".to_string(),
            company: "Ozyegin University".to_string(),
            company_url: None,
            dates: "December 2024 - Present".to_string(),
            location: "Istanbul, Turkey".to_string(),
            description_points: vec![
                "Contribute design of power-train component of car".to_string(),
                "Design and deployment of BSPD circuit. ".to_string(),
                
            ],
            technologies: Some(vec!["KiCad".to_string()]),
            category: ExperienceCategory::Extracurricular, // Assign category
        },
        ExperienceData {
            title: "MATH 212 (Differential Equations) Course T.A.".to_string(),
            company: "Ozyegin University".to_string(),
            company_url: None,
            dates: "September 2024 - January 2025".to_string(),
            location: "Istanbul, Turkey".to_string(),
            description_points: vec![
                "Assisted students throughout the semester with homework and other questions.".to_string(),
            ],
            technologies: None,//Some(vec!["C++".to_string(), "OpenGL".to_string(), "GLSL".to_string(), 
            category: ExperienceCategory::Extracurricular, // Assign category
        },
        // --- Extracurricular Experience ---
        ExperienceData {
            title: "Co-Founder Game Development Club".to_string(),
            company: "Ozyegin University".to_string(),
            company_url: None,
            dates: "December 2022 - December 2024".to_string(),
            location: "Istanbul, Turkey".to_string(),
            description_points: vec![
                "Organizing events related to game development.".to_string(),
                "Management of financials".to_string(),
                
            ],
            technologies: None,//Some(vec!["C++".to_string(), "OpenGL".to_string(), "GLSL".to_string(),
            category: ExperienceCategory::Extracurricular, // Assign category
        },
        ExperienceData {
            title: "Member - Ozu Rover".to_string(),
            company: "Ozyegin University".to_string(),
            company_url: None,
            dates: "December 2021 - December 2022".to_string(),
            location: "Istanbul, Turkey".to_string(),
            description_points: vec![
                "Developed a software that connects to in-house ‘Science Box’ to read and interpret sensor data ".to_string(),
                "Assisted to construction of the in-house science box that is specifically designed for science task 
of URC competition. ".to_string(),
                "Developed a GUI to visualize sensor data and control the science box. ".to_string(),

            ],
            technologies:Some(vec!["Python".to_string(), "Ros".to_string(), "Linux".to_string(), "Nvidia Jetson".to_string()]),
            category: ExperienceCategory::Extracurricular, // Assign category
        },
       
        // Add more ExperienceData structs here for other roles
    ];

    // Filter experiences into separate lists
    let work_experiences: Vec<ExperienceData> = all_experiences
        .iter()
        .filter(|e| e.category == ExperienceCategory::Work)
        .cloned()
        .collect();

    let extracurricular_experiences: Vec<ExperienceData> = all_experiences
        .iter()
        .filter(|e| e.category == ExperienceCategory::Extracurricular)
        .cloned()
        .collect();

    // Helper function to render a list of experiences
    let render_experience_list = |experiences_to_render: Vec<ExperienceData>| -> Html {
        if experiences_to_render.is_empty() {
            html! {} // Don't render the list div if empty
        } else {
            html! {
                <div class="experience-list">
                    {
                        experiences_to_render.iter().map(|exp_item| {
                            // Use title and company as part of the key for better uniqueness
                            let key = format!("{}-{}", exp_item.title, exp_item.company);
                            html! {
                                <ExperienceItem key={key} experience={exp_item.clone()} />
                            }
                        }).collect::<Html>()
                    }
                </div>
            }
        }
    };

    html! {
        <section id="experience" class="content-section experiences-section-categorized"> // Added new class
            <h2>{ "Experience" }</h2> // Main section title

            // --- Work Experience Subsection ---
            if !work_experiences.is_empty() {
                <div class="experience-category-group"> // Wrapper for category
                    <h3 class="experience-category-title">{ "Work Experience" }</h3>
                    { render_experience_list(work_experiences) }
                </div>
            }

            // --- Extracurricular Experience Subsection ---
             if !extracurricular_experiences.is_empty() {
                <div class="experience-category-group"> // Wrapper for category
                    <h3 class="experience-category-title">{ "Leadership & Extracurricular" }</h3> // Changed title slightly
                    { render_experience_list(extracurricular_experiences) }
                </div>
            }

            // Add more subsections here if you have other categories
        </section>
    }
}
