// src/components/projects.rs
use yew::prelude::*;
// Import the ProjectCard component from the sibling module
use super::project_card::ProjectCard;

// --- Project Data Structure ---
// Define a struct to hold all the information for a single project.
// `Clone` is needed to easily pass copies if necessary.
// `PartialEq` is needed because Yew components require props to implement it.
#[derive(Clone, PartialEq)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub technologies: Vec<String>, // A list of technologies used
    pub live_url: Option<String>,  // Optional link to a live demo
    pub repo_url: Option<String>,  // Optional link to the code repository
    // pub image_url: Option<String>, // Optional: Add an image URL field
}

// --- Projects Section Component ---
// This component now manages a list of Project data and renders ProjectCard components.
#[function_component(Projects)]
pub fn projects() -> Html {
    // --- Define your project data here ---
    // This is where you'll add new projects in the future.
    let project_data: Vec<Project> = vec![
        Project {
            title: "My First Cool Project".to_string(),
            description: "A description explaining what this project does, the problems it solves, and perhaps your role in creating it.".to_string(),
            technologies: vec!["Rust".to_string(), "Yew".to_string(), "Trunk".to_string(), "SCSS".to_string()],
            live_url: Some("https://example.com/live-demo".to_string()), // Replace with actual URL or None
            repo_url: Some("https://github.com/yourusername/project-repo".to_string()), // Replace with actual URL or None
            // image_url: Some("project1-thumb.png".to_string()), // Example image path
        },
        Project {
            title: "Another Interesting App".to_string(),
            description: "This app explores using WebAssembly for complex calculations in the browser.".to_string(),
            technologies: vec!["Rust".to_string(), "WASM".to_string(), "JavaScript".to_string()],
            live_url: None, // No live demo for this one
            repo_url: Some("https://github.com/yourusername/another-repo".to_string()),
            // image_url: None,
        },
        // --- Add new projects here ---
        // Project {
        //     title: "Future Project".to_string(),
        //     description: "Details about the next amazing thing.".to_string(),
        //     technologies: vec!["Rust".to_string()],
        //     live_url: None,
        //     repo_url: None,
        //     // image_url: None,
        // },
    ];

    // --- Render the component ---
    html! {
        <section id="projects" class="content-section alt-bg">
            <h2>{ "Projects" }</h2>
            <p>{ "Here's a selection of work I'm proud of." }</p> // You might adjust this intro text

            <div class="project-list">
                {
                    // Iterate over the project_data vector
                    project_data.iter().map(|project_item| {
                        // For each item, render a ProjectCard component,
                        // passing the current project_item as a property.
                        // The `key` prop helps Yew efficiently update the list.
                        // Using the title as a key assumes titles are unique.
                        html! {
                            <ProjectCard key={project_item.title.clone()} project={project_item.clone()} />
                        }
                    }).collect::<Html>() // Collect the generated HTML fragments
                }
            </div>
        </section>
    }
}
