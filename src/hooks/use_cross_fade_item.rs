// src/hooks/use_cross_fade_item.rs
use yew::prelude::*;
use yew_hooks::use_timeout;


/// A custom hook to manage a cross-fade transition between items.
/// When the `target_item` dependency changes, it fades from the
/// currently active item to the new target item over `duration_ms`.
///
/// # Arguments
/// * `target_item` - The desired item to eventually display. The hook watches this for changes.
/// * `duration_ms` - The duration of the cross-fade transition in milliseconds.
///
/// # Returns
/// A tuple containing state handles:
/// * `UseStateHandle<T>`: The currently active (fully visible) item.
/// * `UseStateHandle<T>`: The next item that is fading in.
/// * `UseStateHandle<bool>`: A boolean indicating if the fade transition is currently active.
#[hook]
pub fn use_cross_fade_item<T>(target_item: T, duration_ms: u32) -> (UseStateHandle<T>, UseStateHandle<T>, UseStateHandle<bool>)
where
    T: PartialEq + Clone + 'static, // Item type must be comparable and cloneable
{
    // State for the currently *visible* item
    let active_item = use_state(|| target_item.clone());
    // State for the *next* item to fade in
    let next_item = use_state(|| target_item.clone());
    // State to control the fading CSS class
    let is_fading = use_state(|| false);

    // --- Timeout to complete the fade ---
    let timeout = {
        // Clone handles needed for the timeout closure
        let active_item_setter = active_item.setter();
        let next_item_val = (*next_item).clone(); // Clone value needed when timeout runs
        let is_fading = is_fading.clone();

        use_timeout(move || {
            // When timeout finishes:
            // 1. Make the 'next' item the 'active' one
            active_item_setter.set(next_item_val); // Use the captured next_item_val
            // 2. Turn off the fading class
            is_fading.set(false);
            log::trace!("Cross-fade transition complete.");
        }, duration_ms)
    };

    // --- Effect to initiate fade when target_item changes ---
    {
        // Clone handles/values needed for the effect closure
        let active_item_val = (*active_item).clone(); // Clone value for comparison
        let next_item_setter = next_item.setter();
        let is_fading = is_fading.clone();
        let timeout = timeout.clone();

        use_effect_with(target_item.clone(), move |current_target_item| { // Pass target_item as dependency
            // Run only if the target item is different from the currently active one
            if *current_target_item != active_item_val {
                log::trace!("Target item changed. Starting cross-fade.");
                // 1. Set the 'next' item path
                next_item_setter.set(current_target_item.clone());
                // 2. Trigger the fading class
                is_fading.set(true);
                // 3. Start the timeout to switch active item after fade duration
                timeout.reset();
            }
            // No cleanup needed here
            || ()
        });
    }

    // Return the state handles
    (active_item, next_item, is_fading)
}
