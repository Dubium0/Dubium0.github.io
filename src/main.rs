// src/main.rs
use yew::prelude::*;
// No longer need gloo_timers or yew_hooks::use_effect_once here
// No longer need web_sys here unless used elsewhere

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

// Import the custom hooks
use hooks::use_theme::use_theme;
use hooks::use_loader::use_loader;

// --- Main App Component ---
#[function_component(App)]
fn app() -> Html {
    // Theme state hook
    let (theme, toggle_theme) = use_theme();

    // --- Loader Hook ---
    let _loader_visible = use_loader(1000); // Keep the 3s delay for testing

    // --- Render the App ---
    html! {
        <>
            <Navbar current_theme={(*theme).clone()} toggle_theme={toggle_theme.clone()} />
            <Hero />
            <main class="main-content">
                <About />
                <Projects />
                <Contact />
            </main>
            <Footer />
        </>
    }
}

// --- Entry Point ---
fn main() {
    // Find the root element to mount the application into
    let document = web_sys::window().unwrap().document().unwrap();
    let root = document.get_element_by_id("app-root").unwrap();

    // Mount the Yew application into the <div id="app-root"></div>
    yew::Renderer::<App>::with_root(root).render(); // <-- Use with_root()

    // Previous method (mounts to body, potentially replacing loader):
    // yew::Renderer::<App>::new().render();
}

