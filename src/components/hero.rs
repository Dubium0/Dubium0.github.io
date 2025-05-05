// src/components/hero.rs
use yew::prelude::*;

// --- Hero Section Component ---
#[function_component(Hero)]
pub fn hero() -> Html { // Make the function public (`pub fn`)
    // Placeholder values - replace with your actual info
    let your_name = "Your Name";
    let your_tagline = "Rust & WebAssembly Developer | Building Fast & Reliable Web Apps";
    let your_brief_intro = "Passionate about leveraging Rust's power for modern web experiences.";
    // Make sure the CV file exists in your project root or update the path
    let cv_url = "cv.pdf"; // Example: Assuming cv.pdf is in the root alongside index.html
    let linkedin_url = "https://linkedin.com/in/yourprofile"; // Replace
    let github_url = "https://github.com/yourusername"; // Replace
    let email_address = "mailto:your.email@example.com"; // Replace

    html! {
        <section id="hero" class="hero-section">
            <div class="hero-content">
                // Use a placeholder image service or add your actual image to the project root
                <img src="https://placehold.co/150x150/a3a3a3/ffffff?text=Your\\nPhoto" alt="Your Name - Profile Picture" class="profile-photo"/>
                <h1>{ your_name }</h1>
                <h2>{ your_tagline }</h2>
                <p>{ your_brief_intro }</p>
                <div class="hero-buttons">
                    // Primary CTA linking to the projects section
                    <a href="#projects" class="cta-button primary">{ "View My Work" }</a>
                    // Secondary button for CV download
                    // `target="_blank"` opens the link in a new tab
                    // `rel="noopener noreferrer"` is important for security when using target="_blank"
                    <a href={cv_url} target="_blank" rel="noopener noreferrer" class="cta-button secondary">{ "Download CV" }</a>
                </div>
                <div class="social-links">
                    // Simple text links for now - consider using icons (e.g., Font Awesome) later
                    <a href={linkedin_url} target="_blank" rel="noopener noreferrer">{ "LinkedIn" }</a>
                    <a href={github_url} target="_blank" rel="noopener noreferrer">{ "GitHub" }</a>
                    <a href={email_address}>{ "Email" }</a>
                </div>
            </div>
        </section>
    }
}
