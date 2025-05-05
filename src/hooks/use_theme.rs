// src/hooks/use_theme.rs
use yew::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use web_sys::{window, Window, Document, Element, MediaQueryList, console}; // Added console for logging

const THEME_KEY: &str = "portfolio-theme";

// Define Theme enum here or import if defined globally (e.g., in main.rs or lib.rs)
// For decoupling, defining it here makes the hook self-contained.
#[derive(Clone, PartialEq, Debug)]
pub enum Theme {
    Light,
    Dark,
}

// Helper to get window
fn current_window() -> Option<Window> {
    window()
}

// Helper to get document
fn current_document() -> Option<Document> {
    current_window().and_then(|win| win.document())
}

// Helper to get body element
fn body_element() -> Option<Element> {
    current_document().and_then(|doc| doc.body()).map(|body| body.into())
}


// Helper function to get MediaQueryList for prefers-color-scheme
fn get_media_query() -> Option<MediaQueryList> {
    current_window()
        .and_then(|win| win.match_media("(prefers-color-scheme: dark)").ok())
        .flatten()
}

// --- Custom Hook: use_theme ---
// Manages theme state, persistence, and side effects.
// Returns the current theme state handle and a callback to toggle the theme.
#[hook]
pub fn use_theme() -> (UseStateHandle<Theme>, Callback<MouseEvent>) {
    // State for the current theme
    let theme = use_state(|| {
        console::log_1(&"[use_theme] Initializing theme...".into());
        // 1. Check localStorage first
        match LocalStorage::get::<String>(THEME_KEY) {
            Ok(stored_theme) => {
                    console::log_1(&format!("[use_theme] Found in localStorage: {}", stored_theme).into());
                    if stored_theme == "dark" { Theme::Dark } else { Theme::Light }
            }
            Err(_) => {
                // 2. If no localStorage, check OS preference
                console::log_1(&"[use_theme] Not found in localStorage, checking OS preference.".into());
                get_media_query()
                    .map(|mql| {
                        let os_pref = if mql.matches() { Theme::Dark } else { Theme::Light };
                        console::log_1(&format!("[use_theme] OS preference: {:?}", os_pref).into());
                        os_pref
                        })
                    .unwrap_or_else(|| {
                        console::log_1(&"[use_theme] OS preference check failed, defaulting to Light.".into());
                        Theme::Light // Default to Light if media query fails
                    })
            }
        }
    });

    // Callback to toggle the theme
    let toggle_theme = {
        let theme = theme.clone();
        Callback::from(move |_event: MouseEvent| { // Explicitly type event
            let current_theme_str = format!("{:?}", *theme);
            let new_theme = match *theme {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::Light,
            };
                console::log_1(&format!("[use_theme] Toggle clicked! Current: {}, New: {:?}", current_theme_str, new_theme).into());
            theme.set(new_theme);
        })
    };

    // Effect to update body class and localStorage when theme changes
    // Use the theme state itself as the dependency for the effect
    {
        let theme_val = (*theme).clone(); // Clone the theme value for the effect
        use_effect_with(theme_val, move |current_theme| {
            let theme_str = match current_theme {
                Theme::Light => "light",
                Theme::Dark => "dark",
            };
            console::log_1(&format!("[use_theme] Effect running: Setting theme to {}", theme_str).into());

            // Update localStorage
            if let Err(e) = LocalStorage::set(THEME_KEY, theme_str) {
                    console::error_1(&format!("[use_theme] Failed to set localStorage: {:?}", e).into());
            }

            // Update body class
            if let Some(body) = body_element() {
                let class_list = body.class_list();
                match current_theme {
                    Theme::Light => {
                        class_list.remove_1("dark-mode").ok(); // Ignore potential error if class not present
                        class_list.add_1("light-mode").ok();
                    }
                    Theme::Dark => {
                        class_list.remove_1("light-mode").ok();
                        class_list.add_1("dark-mode").ok();
                    }
                }
                    console::log_1(&format!("[use_theme] Body class set to: {}", body.class_name()).into());
            } else {
                    console::error_1(&"[use_theme] Could not get body element to update class.".into());
            }

            // Return cleanup function (optional, not needed here)
            || ()
        });
    }

    (theme, toggle_theme) // Return the state handle and the callback
}
