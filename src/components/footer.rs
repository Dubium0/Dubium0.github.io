// src/components/footer.rs
use yew::prelude::*;
// No longer need to import Theme or use props for it

// --- Footer Component ---
#[function_component(Footer)]
pub fn footer() -> Html { // No props needed anymore
    let current_year = 2025;
    let your_name = "Yunus Emre Aslan"; // Replace

    html! {
        <footer class="site-footer">
            // Removed the extra wrapper div, adjust if layout needs it
            <p>{ format!("© {} {}", current_year, your_name) }</p>
            // Theme toggle button removed
        </footer>
    }
}
