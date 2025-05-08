// src/components/project_card.rs
use yew::prelude::*;
use super::projects::Project;

#[derive(Properties, PartialEq, Clone)]
pub struct ProjectCardProps {
    pub project: Project,
}

#[function_component(ProjectCard)]
pub fn project_card(props: &ProjectCardProps) -> Html {
    let project = &props.project;

    // GitHub SVG Icon
    let github_icon = html! { /* ... (same as before) ... */
        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="icon-svg project-link-icon">
            <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>
        </svg>
    };

    // Helper to conditionally render the GitHub icon button
    let render_github_link = |url: &Option<String>| -> Html { /* ... (same as before) ... */
        match url {
            Some(href) if !href.is_empty() => html! {
                <a href={href.clone()} target="_blank" rel="noopener noreferrer" class="project-icon-button project-link-hover" aria-label="View GitHub Repository" title="GitHub Repository"> // Added project-link-hover class
                    {github_icon.clone()}
                </a>
            },
            _ => html! {},
        }
    };

    // Render image/gif if URL is provided
    let project_media = match &project.image_url {
        Some(url) if !url.is_empty() => html! {
            <div class="project-media">
                <img src={url.clone()} alt={format!("Preview for {}", project.title)} loading="lazy" />
            </div>
        },
        _ => html! {},
    };

    html! {
        <div class="project-item project-item-interactive">
            { project_media }

            // Content visible initially
            <div class="project-visible-content">
                <h3>{ &project.title }</h3>
                // Render technology tags (summary)
                if !project.technologies.is_empty() {
                    <div class="project-tech-tags-summary">
                        { for project.technologies.iter().take(3).map(|tech| html!{ <span class="tech-tag">{tech}</span> }) }
                        { if project.technologies.len() > 3 { html!{<span class="tech-tag more-tag">{"..."}</span>} } else { html!{} } }
                    </div>
                }
                 // GitHub link removed from here
                 // <div class="project-links-visible"> ... </div>
            </div>

            // Content revealed on hover
            <div class="project-hover-content">
                <p class="project-description">{ &project.description }</p>
                // Render full technology tags list on hover
                if !project.technologies.is_empty() {
                     <div class="project-tech-tags-full">
                        <strong>{"Technologies: "}</strong>
                        { for project.technologies.iter().map(|tech| html!{ <span class="tech-tag">{tech}</span> }) }
                     </div>
                }
                // --- Links shown on hover ---
                 <div class="project-hover-links"> // New wrapper for links
                     // Live Demo link
                     if let Some(href) = &project.live_url {
                         if !href.is_empty() {
                             <a href={href.clone()} target="_blank" rel="noopener noreferrer" class="project-text-link project-link-hover">{ "Live Demo" }</a>
                         }
                     }
                     // GitHub link moved here
                     { render_github_link(&project.repo_url) }
                 </div>
            </div>
        </div>
    }
}
