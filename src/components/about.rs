// src/components/about.rs
use yew::prelude::*;

// --- About Section Component ---
#[function_component(About)]
pub fn about() -> Html { // Make the function public (`pub fn`)
    html! {
        <section id="about" class="content-section">
            <h2>{ "About Me" }</h2>
            <p class="centered-text">{ "Born in Samsun, Turkey" }</p>
            <p class="centered-text">{ "Lives & studying in Istanbul, Turkey" }</p>
            <p>{ "  I'm a senior computer science student who enjoys creating visually impressive software such as games, and renderers. 
                    I'm trying to learn more about computer graphics, software development, and modern game engines such as Unity and Unreal Engine 5. 
                    For now, my favorite programming language is C++, and my favorite game engine is Unity. 
                    I'm open to everything interesting about game development and graphics programming so don't hesitate to contact me if you have things to talk about. " }</p>
          
            // Add more paragraphs as needed
        </section>
    }
}
