// src/components/navbar.rs
use yew::prelude::*;
use crate::hooks::use_theme::Theme; // Import Theme enum

// --- Component Properties ---
#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub current_theme: Theme,
    pub toggle_theme: Callback<MouseEvent>,
}

// --- Navbar Component ---
#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    let site_title = "Yunus Emre Aslan"; // Replace or make dynamic
    let cv_url = "cv.pdf"; // Assuming cv.pdf is in the root

    // --- Social Links ---
    let linkedin_url = "https://www.linkedin.com/in/yunus-emre-aslan-90895a175/";
    let github_url = "https://github.com/Dubium0"; 
    let email_address = "mailto:yunusemreaslan09@gmail.com";

    // --- SVG Icons ---
    let github_icon = html! { <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="icon-svg"><path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path></svg> };
    let linkedin_icon = html! { <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="icon-svg"><path d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z"></path><rect x="2" y="9" width="4" height="12"></rect><circle cx="4" cy="4" r="2"></circle></svg> };
    let mail_icon = html! { <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="icon-svg"><path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"></path><polyline points="22,6 12,13 2,6"></polyline></svg> };
    let moon_icon = Html::from_html_unchecked(r#"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="icon-svg"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path></svg>"#.into());
    let sun_icon = Html::from_html_unchecked(r#"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="icon-svg"><circle cx="12" cy="12" r="5"></circle><line x1="12" y1="1" x2="12" y2="3"></line><line x1="12" y1="21" x2="12" y2="23"></line><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line><line x1="1" y1="12" x2="3" y2="12"></line><line x1="21" y1="12" x2="23" y2="12"></line><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line></svg>"#.into());

    let (theme_icon, theme_aria_label) = match props.current_theme {
        Theme::Light => (moon_icon, "Activate dark mode"),
        Theme::Dark => (sun_icon, "Activate light mode"),
    };

    html! {
        <header class="site-navbar">
            <div class="navbar-container">
                <div class="navbar-brand">
                    // Link brand to top/hero section
                    <a href="#hero">{ site_title }</a>
                </div>
                <nav class="navbar-nav">
                    // --- Navigation Links ---
                    // Use anchor links pointing to section IDs
                    <a href="#about" class="nav-text-link">{"About"}</a>
                    <a href="#projects" class="nav-text-link">{"Projects"}</a>
                    <a href="#experience" class="nav-text-link">{"Experience"}</a>
                    <a href="#education" class="nav-text-link">{"Education"}</a>
                    <a href="#skills" class="nav-text-link">{"Skills"}</a>
                    <a href="#contact" class="nav-text-link">{"Contact"}</a>

                    // Separator (optional)
                    <span class="nav-separator"></span>

                    // --- Action Buttons/Icons ---
                    <a href={cv_url} target="_blank" rel="noopener noreferrer" class="nav-button-link">
                        { "Download CV" }
                    </a>
                     <a href={github_url} target="_blank" rel="noopener noreferrer" class="nav-icon-link" aria-label="My GitHub Profile" title="GitHub">
                       {github_icon}
                    </a>
                    <a href={linkedin_url} target="_blank" rel="noopener noreferrer" class="nav-icon-link" aria-label="My LinkedIn Profile" title="LinkedIn">
                        {linkedin_icon}
                    </a>
                    <a href={email_address} class="nav-icon-link" aria-label="Send me an email" title="Email">
                       {mail_icon}
                    </a>
                    <button
                        class="theme-toggle-button icon-button"
                        onclick={props.toggle_theme.clone()}
                        aria-label={theme_aria_label}
                        title={theme_aria_label}
                    >
                        { theme_icon }
                    </button>
                </nav>
            </div>
        </header>
    }
}
