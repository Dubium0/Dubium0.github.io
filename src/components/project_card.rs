// src/components/project_card.rs
use yew::prelude::*;
use super::projects::Project; // Import the Project struct (assuming it's in projects.rs)

#[derive(Properties, PartialEq, Clone)]
pub struct ProjectCardProps {
    pub project: Project, // The component receives a Project struct as a property
}

// --- Project Card Component ---
// This component displays a single project item.
#[function_component(ProjectCard)]
pub fn project_card(props: &ProjectCardProps) -> Html {
    let project = &props.project; // Get a reference to the project data

    // Helper to conditionally render links if they exist
    let render_link = |label: &str, url: &Option<String>| -> Html {
        match url {
            Some(href) if !href.is_empty() => html! {
                <a href={href.clone()} target="_blank" rel="noopener noreferrer">{ label }</a>
            },
            _ => html! {}, // Render nothing if the URL is None or empty
        }
    };

    html! {
        <div class="project-item">
            // Optional: Add image if needed, e.g., from project.image_url
            // if let Some(img_src) = &project.image_url {
            //     <img src={img_src.clone()} alt={format!("{} Thumbnail", project.title)}/>
            // }

            <h3>{ &project.title }</h3>
            <p>{ &project.description }</p>
            // Only render the technologies line if the list is not empty
            if !project.technologies.is_empty() {
                <p><strong>{ "Technologies: " }</strong> { project.technologies.join(", ") }</p>
            }
            <div class="project-links">
                { render_link("Live Demo", &project.live_url) }
                { render_link("GitHub Repo", &project.repo_url) }
            </div>
        </div>
    }
}
