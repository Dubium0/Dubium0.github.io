// src/components/skills.rs
use yew::prelude::*;

// --- Data Structure for a Skill Category ---
#[derive(Clone, PartialEq)]
struct SkillCategory {
    name: String,
    items: Vec<String>,
}

// --- Data Structure for Hobbies/Interests ---
#[derive(Clone, PartialEq)]
struct Interest {
    name: String,
    // description: Option<String>, // Optional: if you want to add a short description
}

// --- Skills Section Component ---
#[function_component(Skills)]
pub fn skills() -> Html {
    // --- Define your skills data here ---
    let skill_categories: Vec<SkillCategory> = vec![
        SkillCategory {
            name: "Languages".to_string(),
            items: vec![
                "Turkish (Native)".to_string(),
                "English (Fluent)".to_string(),
                // Add other languages
            ],
        },
        SkillCategory {
            name: "Programming".to_string(),
            items: vec![
                "C++".to_string(),
                "C#".to_string(),
                "ASAP.NET".to_string(),
                "Python".to_string(),
                "GLSL/HLSL".to_string(),
                "Java".to_string(),
                "Rust (Learning)".to_string(),
                "Kotlin (Basics)".to_string(),
                "Flutter (Basics)".to_string(),
                // Add other programming languages/skills
            ],
        },
        SkillCategory {
            name: "Software & Tools".to_string(),
            items: vec![
                "Git & GitHub & Plastics SCM".to_string(),
                "Visual Studio".to_string(),
                "Unity Engine".to_string(),
                "Unreal Engine 5 (Learning)".to_string(),
                "Blender".to_string(),
                "VS Code".to_string(),
                "CMake".to_string(),
                "Premake5".to_string(),
                "PostgreSQL".to_string(),
                "LÖVE game engine".to_string(),
                "Microsoft Office programs".to_string(),
                "KiCad".to_string(),
                // Add other software/tools
            ],
        },
        SkillCategory {
            name: "Hardware".to_string(),
            items: vec![
                "PC Building".to_string(),
                "Arduino (Basic)".to_string(),
                "Electronics (Basic)".to_string(),
                
            ],
        },
    ];

    // --- Define your hobbies/interests data here ---
    let interests: Vec<Interest> = vec![
        Interest { name: "Game Development".to_string() },
        Interest { name: "Computer Graphics".to_string() },
        Interest { name: "Electric Guitars".to_string() },
        Interest { name: "Drawing".to_string() },
        Interest { name: "Movies and Anime".to_string() },
        Interest { name: "Music Composing".to_string() },
        Interest { name: "Playing Video Games".to_string() },
        // Add other interests
    ];

    html! {
        <section id="skills" class="content-section skills-section">
            <h2>{ "Skills & Interests" }</h2>

            <div class="skills-grid">
                {
                    skill_categories.iter().map(|category| {
                        html! {
                            <div class="skill-category">
                                <h3>{ &category.name }</h3>
                                <ul>
                                    { for category.items.iter().map(|item| html!{ <li>{item}</li> }) }
                                </ul>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>

            <div class="interests-section">
                <h3>{ "Areas of Interest" }</h3> // Or "Hobbies"
                <ul class="interests-list">
                     { for interests.iter().map(|interest| html!{ <li>{ &interest.name }</li> }) }
                </ul>
                // Alternatively, display as a paragraph:
                // <p class="interests-paragraph">
                //     { interests.iter().map(|i| i.name.clone()).collect::<Vec<String>>().join(", ") }
                // </p>
            </div>
        </section>
    }
}
