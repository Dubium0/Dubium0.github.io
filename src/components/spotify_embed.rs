// src/components/spotify_embed.rs
use yew::prelude::*;

#[function_component(SpotifyEmbed)]
pub fn spotify_embed() -> Html {
    /*
    <iframe style="border-radius:12px" src="https://open.spotify.com/embed/playlist/77zgVtI8g94QY39QzfyEZK?utm_source=generator&theme=0" width="100%" height="152" frameBorder="0" allowfullscreen="" allow="autoplay; clipboard-write; encrypted-media; fullscreen; picture-in-picture" loading="lazy"></iframe>
     */
    // Replace with your actual Spotify Playlist URI (found via Share -> Embed Playlist -> Copy Spotify URI)
    // Example URI: spotify:playlist:37i9dQZF1DXcBWIGoYBM5M
    let playlist_uri = "spotify:playlist:77zgVtI8g94QY39QzfyEZK"; // <-- REPLACE THIS
    // Construct the embed URL
    let embed_src = format!("https://open.spotify.com/embed/playlist/{}?utm_source=generator", playlist_uri.split(':').last().unwrap_or(""));

    // Spotify Icon SVG
    let spotify_logo_path = "images/spotify_logo.png"; 

    html! {
        // New outer wrapper to handle hover state for both trigger and panel
        <div class="spotify-widget-wrapper">
            // Trigger remains visible
            <div class="spotify-trigger" aria-label="Show Spotify Playlist">
                <img src={spotify_logo_path} alt="Spotify Logo" class="spotify-trigger-icon" />
            </div>
            // Panel itself, initially hidden
            <div class="spotify-slide-out">
                <div class="spotify-player-container">
                    <iframe
                        style="border-radius:8px;" // Adjust radius if needed
                        src={embed_src}
                        width="100%"
                        height="152" // Compact player height
                        frameBorder="0"
                        allow="autoplay; clipboard-write; encrypted-media; fullscreen; picture-in-picture"
                        loading="lazy">
                    </iframe>
                </div>
            </div>
        </div>
    }
}
