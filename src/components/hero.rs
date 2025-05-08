// src/components/hero.rs
use yew::prelude::*;

use crate::hooks::use_random_interval_item::use_random_interval_item; 
use crate::hooks::use_cross_fade_item::use_cross_fade_item; 
use std::rc::Rc; 
// --- Hero Section Component ---
#[function_component(Hero)]
pub fn hero() -> Html {
    // Placeholder values - replace with your actual info
    let your_name = "Yunus Emre Aslan";
    let your_tagline = "Software Engineer";
    let your_brief_intro = "Hi, I'm Yunus Emre ASLAN, a senior Computer Science student at Ozyegin University here in Istanbul. I'm currently focusing on Game Development (specifically game engines) and Graphics Programming. My go-to programming languages are C++ and C#, and I love using the Unity game engine to bring my ideas to life. It's probably no surprise that I'm a huge gaming enthusiast!";
    let profile_image_path = "images/profile_image.png";
    let background_gifs: Rc<Vec<&'static str>> = Rc::new(vec![
        "images/battleship.gif",
        "images/jester.gif",
        "images/OceanSimulation.gif",
        "images/openglTemplate.gif",
        "images/marchingCubes.gif",
        // Add more GIF paths here
    ]);

    const CHANGE_INTERVAL_MS: u32 = 4000;
    const FADE_DURATION_MS: u32 = 750;

    // --- Use the random item hook to get the target GIF ---
    let target_gif = use_random_interval_item(background_gifs, CHANGE_INTERVAL_MS);

    // --- Use the cross-fade hook ---
    // Pass the target GIF value (cloned) and the duration.
    // It returns handles for the active item, the next item, and the fading state.
    let (active_gif, next_gif, is_fading) = use_cross_fade_item(*target_gif, FADE_DURATION_MS);

    html! {
        <section id="hero" class="hero-section">
             // Add fading class to the container when transition is active
             <div class={classes!("hero-background-media", (*is_fading).then_some("is-fading"))}>
                // Image for the currently active GIF
                <img
                    src={*active_gif} // Use active_gif handle
                    alt="Animated background (active)"
                    class="hero-background-gif active-gif"
                    key={format!("{}-active", *active_gif)}
                />
                // Image for the next GIF (fades in)
                 <img
                    src={*next_gif} // Use next_gif handle
                    alt="Animated background (next)"
                    class="hero-background-gif next-gif"
                    key={format!("{}-next", *next_gif)}
                />
                <div class="hero-overlay"></div>
             </div>

            // --- Foreground Content ---
            <div class="hero-content">
                <img
                    src={profile_image_path}
                    alt={format!("{} - Profile Picture", your_name)}
                    class="profile-photo"
                 />
                <h1>{ your_name }</h1>
                <h2>{ your_tagline }</h2>
                <p>{ your_brief_intro }</p>
                <div class="hero-buttons">
                    <a href="#projects" class="cta-button primary">{ "View My Work" }</a>
                </div>
            </div>
        </section>
    }
}
