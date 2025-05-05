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
    let site_title = "Your Name"; // Replace or make dynamic

    // Define SVG icons as strings
    let moon_icon = Html::from_html_unchecked(r#"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-moon"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path></svg>"#.into());
    let sun_icon = Html::from_html_unchecked(r#"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-sun"><circle cx="12" cy="12" r="5"></circle><line x1="12" y1="1" x2="12" y2="3"></line><line x1="12" y1="21" x2="12" y2="23"></line><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line><line x1="1" y1="12" x2="3" y2="12"></line><line x1="21" y1="12" x2="23" y2="12"></line><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line></svg>"#.into());

    // Determine which icon and aria-label to use
    let (icon, aria_label) = match props.current_theme {
        Theme::Light => (moon_icon, "Activate dark mode"),
        Theme::Dark => (sun_icon, "Activate light mode"),
    };

    html! {
        <header class="site-navbar">
            <div class="navbar-container">
                <div class="navbar-brand">
                    // Link to the top of the page
                    <a href="#">{ site_title }</a>
                </div>
                <nav class="navbar-nav">
                    // Add other nav links here if needed (e.g., About, Projects)
                    // <a href="#about">{"About"}</a>
                    // <a href="#projects">{"Projects"}</a>
                    // <a href="#contact">{"Contact"}</a>

                    // Theme Toggle Button
                    <button
                        class="theme-toggle-button icon-button" // Add icon-button class
                        onclick={props.toggle_theme.clone()}
                        aria-label={aria_label}
                        title={aria_label} // Add title for tooltip on hover
                    >
                        { icon }
                    </button>
                </nav>
            </div>
        </header>
    }
}
