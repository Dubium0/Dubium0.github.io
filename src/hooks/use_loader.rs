// src/hooks/use_loader.rs
use yew::prelude::*;
use yew_hooks::use_effect_once;
use gloo_timers::callback::Timeout;
use web_sys::{window, Document};

// Helper function to get document
fn current_document() -> Option<Document> {
    window().and_then(|win| win.document())
}

// --- Custom Hook: use_loader ---
// Manages the state for the initial loading screen, ensuring a minimum display time.
// Returns `true` if the loader should still be visible, `false` otherwise.
#[hook]
pub fn use_loader(min_display_ms: u32) -> bool {
    // State
    let app_ready = use_state(|| false);      // Tracks if Yew app has mounted (component using the hook)
    let timer_elapsed = use_state(|| false); // Tracks if the minimum delay has passed
    let loader_hidden = use_state(|| false); // Tracks if the hidden class has been applied

    // --- Effect 1: Mark App as Ready on Mount ---
    {
        let app_ready = app_ready.clone();
        use_effect_once(move || {
            web_sys::console::log_1(&"[use_loader] Component mounted.".into());
            app_ready.set(true);
            || ()
        });
    }

    // --- Effect 2: Start Minimum Delay Timer ---
    {
        let timer_elapsed = timer_elapsed.clone();
        use_effect_once(move || {
            let timeout = Timeout::new(min_display_ms, move || {
                web_sys::console::log_1(&format!("[use_loader] Minimum loader time ({}ms) elapsed.", min_display_ms).into());
                timer_elapsed.set(true);
            });
            move || { drop(timeout); } // Cleanup: Cancel timer if component unmounts
        });
    }

    // --- Effect 3: Hide Loader When BOTH Conditions are Met ---
    // This effect runs when app_ready or timer_elapsed changes.
    {
        // Clone state needed for the effect's dependencies and logic
        let app_ready_val = *app_ready;
        let timer_elapsed_val = *timer_elapsed;
        let loader_hidden_handle = loader_hidden.clone(); // Clone handle to set state inside

        use_effect_with((app_ready_val, timer_elapsed_val), // Depend on both state values
            move |(current_app_ready, current_timer_elapsed)| {
            web_sys::console::log_1(&format!("[use_loader] Hide check: AppReady={}, TimerElapsed={}", current_app_ready, current_timer_elapsed).into());

            // Only proceed if loader isn't already hidden
            if !*loader_hidden_handle && *current_app_ready && *current_timer_elapsed {
                // Both conditions are true, hide the loader
                if let Some(document) = current_document() {
                    if let Some(loader_element) = document.get_element_by_id("loader") {
                         if loader_element.class_list().add_1("hidden").is_ok() {
                             web_sys::console::log_1(&"[use_loader] Loader hidden: App ready and timer elapsed.".into());
                             loader_hidden_handle.set(true); // Mark as hidden
                         } else {
                             web_sys::console::error_1(&"[use_loader] Failed to add hidden class to loader.".into());
                         }
                    } else {
                         web_sys::console::warn_1(&"[use_loader] Loader element (#loader) not found during hide attempt.".into());
                    }
                }
            }
             || () // No cleanup needed
        });
    }

    // Return whether the loader should *still* be visible
    // Loader is visible if it hasn't been marked as hidden yet.
    !*loader_hidden
}
