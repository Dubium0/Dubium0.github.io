// src/components/contact.rs
use yew::prelude::*;

// --- Contact Section Component ---
#[function_component(Contact)]
pub fn contact() -> Html { // Make the function public (`pub fn`)
     let email_address = "mailto:emre.aslan.24628@ozu.edu.tr"; // Replace
     // Make sure the CV file exists in your project root or update the path
     let cv_url = "images/YunusEmreAslanCV.pdf"; // Example: Assuming cv.pdf is in the root

    html! {
        <section id="contact" class="content-section">
            <h2>{ "Get In Touch" }</h2>
            <p>{ "I'm always open to discussing new projects, creative ideas, or opportunities to be part of your visions." }</p>
            <p>{ "Feel free to reach out via email: "} <a href={email_address}>{ "emre.aslan.24628@ozu.edu.tr" }</a></p>
            // Optional: Add a contact form component here later
            <p>{ "You can also find my CV here: "} <a href={cv_url} target="_blank" rel="noopener noreferrer">{ "Download CV" }</a></p>
        </section>
    }
}
