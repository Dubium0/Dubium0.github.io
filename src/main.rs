// src/main.rs
use yew::prelude::*;

// Declare modules
mod components;
mod hooks;

// Import components
use components::about::About;
use components::contact::Contact;
use components::footer::Footer;
use components::hero::Hero;
use components::projects::Projects;
use components::navbar::Navbar;
use components::experiences::Experiences;
use components::education::Education;
use components::skills::Skills; 
use components::spotify_embed::SpotifyEmbed;
// Import custom hooks
use hooks::use_theme::use_theme;
use hooks::use_loader::use_loader;

// --- Main App Component ---
#[function_component(App)]
fn app() -> Html {
    let (theme, toggle_theme) = use_theme();
    let _loader_visible = use_loader(1000);

    html! {
        <>
            <SpotifyEmbed />
            <Navbar current_theme={(*theme).clone()} toggle_theme={toggle_theme.clone()} />
            <Hero />
            <main class="main-content">
                <About />
                <Projects />
                <Experiences />
                <Education />
                <Skills /> // <-- Add Skills section here
                <Contact />
            </main>
            <Footer />
        </>
    }
}

// --- Entry Point ---
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    let document = web_sys::window().unwrap().document().unwrap();
    let root = document.get_element_by_id("app-root").unwrap();
    yew::Renderer::<App>::with_root(root).render();
}
