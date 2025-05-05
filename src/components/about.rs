// src/components/about.rs
use yew::prelude::*;

// --- About Section Component ---
#[function_component(About)]
pub fn about() -> Html { // Make the function public (`pub fn`)
    html! {
        <section id="about" class="content-section">
            <h2>{ "About Me" }</h2>
            <p>{ "Hello! I'm a dedicated developer with a strong focus on performance and safety, which naturally led me to Rust and WebAssembly." }</p>
            <p>{ "Here, you can talk about your journey, your key skills (e.g., specific Rust crates, frontend frameworks, backend experience), your approach to development, and perhaps a bit about your interests outside of coding." }</p>
            <p>{ "My goal is to build efficient, maintainable, and user-friendly applications." }</p>
            // Add more paragraphs as needed
        </section>
    }
}
