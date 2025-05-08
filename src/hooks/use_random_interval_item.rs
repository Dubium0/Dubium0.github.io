// src/hooks/use_random_interval_item.rs
use yew::prelude::*;
use rand::seq::SliceRandom;
use gloo_timers::callback::Interval;
use yew_hooks::use_effect_once;
use std::rc::Rc;

/// A custom hook that selects a random item from a list at a specified interval.
///
/// # Arguments
/// * `items` - An `Rc<Vec<T>>` containing the list of items to choose from.
///             Using Rc allows sharing the list without cloning the whole Vec.
/// * `interval_ms` - The interval in milliseconds at which to change the item.
///
/// # Returns
/// * `UseStateHandle<T>` - A Yew state handle holding the currently selected item.
///                         The initial value is randomly selected.
#[hook]
pub fn use_random_interval_item<T>(items: Rc<Vec<T>>, interval_ms: u32) -> UseStateHandle<T>
where
    T: Clone + PartialEq + 'static, // Item type must be Cloneable and PartialEq
{
    // State for the currently selected item
    let current_item = use_state(|| {
        let mut rng = rand::thread_rng();
        // Clone the item when setting initial state
        items.choose(&mut rng).cloned().unwrap_or_else(|| items[0].clone())
    });

    // State to hold the interval handle
    let interval_handle_state: UseStateHandle<Option<Interval>> = use_state(|| None);

    // Effect to set up and clean up the interval timer
    {
        let current_item_setter = current_item.setter();
        let items_clone = items.clone(); // Clone Rc for the closure
        let interval_handle_state = interval_handle_state.clone();
        let current_item_value = (*current_item).clone(); // Clone current value for comparison

        use_effect_once(move || {
            // Only set up interval if there's more than one item
            if items_clone.len() > 1 {
                log::info!("Setting up random item interval ({}ms).", interval_ms);
                let interval = Interval::new(interval_ms, move || {
                    let mut rng = rand::thread_rng();
                    // Choose a new random item, trying not to repeat immediately
                    let next_item = loop {
                        let chosen = items_clone.choose(&mut rng)
                            .cloned() // Clone the chosen item
                            .unwrap_or_else(|| items_clone[0].clone()); // Fallback clone

                        // Use chosen if it's different from the one captured at effect start
                        if chosen != current_item_value {
                            break chosen;
                        }
                        // If only one item, break immediately (won't change but avoids infinite loop)
                        if items_clone.len() <= 1 { break chosen; }
                    };

                    // log::info!("Changing item..."); // Optional logging
                    current_item_setter.set(next_item);
                    // Note: current_item_value used in loop comparison doesn't update here.
                });
                // Store handle to keep interval running
                interval_handle_state.set(Some(interval));
            } else {
                 log::warn!("Not starting interval: only one or zero items provided.");
            }

            // Cleanup function
            move || {
                log::info!("Cleaning up random item interval.");
                // Interval is cancelled automatically when handle is dropped by state
            }
        });
    } // End effect block

    current_item // Return the state handle for the current item
}

