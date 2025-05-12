// src/components/soundcloud_embed.rs
use yew::prelude::*;

/*
<iframe width="100%" 
height="300" 
scrolling="no" 
frameborder="no" 
allow="autoplay" 
src="https://w.soundcloud.com/player/?url=https%3A//api.soundcloud.com/playlists/2019175608&color=%23ff5500&auto_play=true&hide_related=false&show_comments=true&show_user=true&show_reposts=false&show_teaser=true&visual=true">
</iframe><div style="font-size: 10px; 
color: #cccccc;
line-break: anywhere;
word-break: normal;
overflow: hidden;
white-space: nowrap;
text-overflow: ellipsis; 
font-family: Interstate,Lucida Grande,Lucida Sans Unicode,Lucida Sans,Garuda,Verdana,Tahoma,sans-serif;
font-weight: 100;">
<a href="https://soundcloud.com/yunus-emre-aslan-372635110" 
title="Yunus Emre Aslan" 
target="_blank" 
style="color: #cccccc; 
text-decoration: none;">Yunus Emre Aslan</a> ·
 <a href="https://soundcloud.com/yunus-emre-aslan-372635110/sets/falling-into-clouds" 
 title="Falling into clouds" target="_blank" style="color: #cccccc; text-decoration: none;">Falling into clouds</a></div>



*/
#[function_component(SoundCloudEmbed)]
pub fn soundcloud_embed() -> Html {
 
    
   
    let embed_src = "https://w.soundcloud.com/player/?url=https%3A//api.soundcloud.com/playlists/2019175608&color=%23ff5500&auto_play=true&hide_related=false&show_comments=false&show_user=true&show_reposts=false&show_teaser=true&visual=true";

    // Path to your SoundCloud logo PNG in the assets folder
    // Make sure you have a soundcloud_logo.png in your assets folder
    let soundcloud_logo_path = "images/soundcloud.svg"; // <-- REPLACE if your logo has a different name/path

    html! {
        // Outer wrapper for hover state
        <div class="soundcloud-widget-wrapper"> // Renamed class
            // Trigger remains visible
            <div class="soundcloud-trigger" aria-label="Show SoundCloud Player"> // Renamed class
                <img src={soundcloud_logo_path} alt="SoundCloud Logo" class="soundcloud-trigger-icon" /> // Renamed class
            </div>
            // Panel itself, initially hidden
            <div class="soundcloud-slide-out"> // Renamed class
                <div class="soundcloud-player-container"> // Renamed class
                    <iframe
                        width="100%"
                        height="300" 
                        scrolling="no"
                        frameborder="no"
                        allow="autoplay" // SoundCloud allows autoplay
                        src={embed_src}>
                    </iframe>
                </div>
            </div>
        </div>
    }
}
