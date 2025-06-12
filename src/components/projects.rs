/*
 Project {
            title: "Ocean Simulation".to_string(),
            description: "Developed for CS426 Spring 2024, this project achieves realistic real-time ocean surface simulation in Unity through procedural mesh generation, shader-based wave functions, and dynamic tessellation for visual appeal and performance.".to_string(),
            technologies: vec!["Unity".to_string(), "C#".to_string(), "HLSL".to_string()],
            live_url: None, //Some("https://example.com/live-demo".to_string()), // Replace with actual URL or None
            repo_url: Some("https://github.com/Dubium0/OceanSimulation".to_string()), // Replace with actual URL or None
            // image_url: Some("project1-thumb.png".to_string()), // Example image path
        },

*/


// src/components/projects.rs
use yew::prelude::*;
// Import the ProjectCard component from the sibling module
use super::project_card::ProjectCard;

// --- Enum for Project Category ---
#[derive(Clone, PartialEq)]
pub enum ProjectCategory {
    Games,
    Other,
    // Add other categories if needed, e.g., Contribution
}

// --- Project Data Structure ---
#[derive(Clone, PartialEq)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub technologies: Vec<String>,
    pub live_url: Option<String>,
    pub repo_url: Option<String>,
    pub image_url: Option<String>,
    pub category: ProjectCategory, // <-- Added category field
}

// --- Projects Section Component ---
#[function_component(Projects)]
pub fn projects() -> Html {
    // --- Define your project data here ---
    let all_projects: Vec<Project> = vec![
        
          Project {
            title: "Portfolio Website".to_string(),
            description: "This project is actually this website, written in Rust and I used yew framework.".to_string(),
            technologies: vec!["Rust".to_string(), "Scss".to_string(), "Github Pages".to_string(),"Trunk".to_string()],
            live_url: None,
            repo_url: Some("https://github.com/Dubium0/Dubium0.github.io".to_string()),
            image_url: Some("images/website.gif".to_string()), // Example: Can be jpg, png, gif etc.
            category : ProjectCategory::Other,
        },
       
        Project {
            title: "Ocean Simulation".to_string(),
            description: "Developed for CS426 Spring 2024, this project achieves real-time ocean surface simulation in Unity through procedural mesh generation, gertsner waves, and dynamic tessellation for visual appeal and performance.".to_string(),
            technologies: vec!["Unity".to_string(), "C#".to_string(), "HLSL".to_string()],
            live_url: None,//Some("https://example.com/live-demo".to_string()),
            repo_url: Some("https://github.com/Dubium0/OceanSimulation".to_string()),
            image_url: Some("images/OceanSimulation.gif".to_string()), // Example: Place preview in assets folder
            category : ProjectCategory::Other,
        },
        Project {
            title: "Game Library API".to_string(),
            description: "Aim of this project creating a REST api for game library applications. Also I made a frontend with flutter to showcase the api s functionality.".to_string(),
            technologies: vec!["C#".to_string(), "ASP.NET".to_string(),"Flutter".to_string(),"VS code".to_string(),"REST architecture".to_string(),"PostgreSQL".to_string(),],
            live_url: None,
            repo_url: Some("https://github.com/Dubium0/GameLib".to_string()),
            image_url: Some("images/gamelib.gif".to_string()), // Example: Can be jpg, png, gif etc.
            category : ProjectCategory::Other,
        },
        Project {
            title: "OpenGL Template".to_string(),
            description: "Get started with OpenGL rendering instantly. This project's purpose is to cut down on lengthy setup, so you can see results on screen right away.".to_string(),
            technologies: vec!["C++".to_string(), "OpenGL".to_string(), "GLSL".to_string(),"CMake".to_string(),"Visual Studio".to_string(),"GLFW".to_string(),],
            live_url: None,
            repo_url: Some("https://github.com/Dubium0/OpenGL_Application".to_string()),
            image_url: Some("images/openglTemplate.gif".to_string()), // Example: Can be jpg, png, gif etc.
            category : ProjectCategory::Other,
        },
        Project {
            title: "Jade Game Engine".to_string(),
            description: "This is my attempt to create a game engine. It is still in-progress. My main focus will be developing a game engine which can draw many objects efficiently. Currently working on native ECS support.".to_string(),
            technologies: vec!["C++".to_string(), "Vulkan".to_string(), "GLSL".to_string(), "Premake5".to_string(), "SDL2".to_string(), "ImGui".to_string()],
            live_url: None, //
            repo_url: Some("https://github.com/Dubium0/Jade".to_string()),
            image_url: Some("images/jadeEngine.gif".to_string()), // Example: Can be jpg, png, gif etc.
            category : ProjectCategory::Other,
        },
     
         Project {
            title: "Marching Cubes".to_string(),
            description: "This project is about rendering a procedurally generated cave through a 3D noise with marching cube algorithm.".to_string(),
            technologies: vec!["C++".to_string(), "OpenGL".to_string(), "GLSL".to_string(),"CMake".to_string(),"Visual Studio".to_string(),"GLFW".to_string(),],
            live_url: None,
            repo_url: Some("https://github.com/Dubium0/MarchingCubes".to_string()),
            image_url: Some("images/marchingCubes.gif".to_string()), // Example: Can be jpg, png, gif etc.
            category : ProjectCategory::Other,
        },
        Project {
            title: "Football Sim".to_string(),
            description: "Football Sim is my senior project for CS degree. Aim of this project is to make a football game that supports; Football Player vs AI, Local PVP and Online Multiplayer PVP. We achieved multiplayer with implementing a P2P connection between players by using Unity Netcode and SteamWorks. Football Player AI managed by a behavior tree.".to_string(),
            technologies: vec!["Unity".to_string(),"C#".to_string(),"Netcode".to_string(),"Steamworks".to_string(),"Behavior Tree".to_string()],
            live_url: None, //
            repo_url: Some("https://github.com/Dubium0/FutbolSim".to_string()),
            image_url: Some("images/footballSim.gif".to_string()), // Example: Can be jpg, png, gif etc.
            category : ProjectCategory::Games,
            
        },
        Project {
            title: "The Jester".to_string(),
            description: "The Jester was our submission for GGJ-2024. The theme of the game jam was 'Make me laugh'. My friend and I made a game about being a jester to a king. Through cards, we try to match the king's mood. If we achieve this, we survive; if not, the king will kill us XD.".to_string(),
            technologies: vec!["Unity".to_string(),"C#".to_string()],
            live_url: None, //
            repo_url: Some("https://github.com/Dubium0/Jester".to_string()),
            image_url: Some("images/jester.gif".to_string()), // Example: Can be jpg, png, gif etc.
            category : ProjectCategory::Games,
        },
        Project {
            title: "QuestLog".to_string(),
            description: "Questlog is an online game journal, where you can track the games you play or review. Main architecture is MVVM, DifUtils and Databinding can be found in the project. Developed for Kotlin certified course.".to_string(),
            technologies: vec!["Kotlin".to_string(),"Android Studio".to_string()],
            live_url: None, //
            repo_url: Some("https://github.com/Dubium0/QuestLog".to_string()),
            image_url: Some("images/questlog.gif".to_string()), // Example: Can be jpg, png, gif etc.
            category : ProjectCategory::Other,
        },
        Project {
            title: "Behavior Tree Implementation".to_string(),
            description: "Main purpose of this project is demonstrating common software engineering patterns for CS434 (Software Engineering Design Course). I made my demonstration with C# implementation of a Behavior Tree (BT), a popular AI technique used in game development and robotics for autonomous agents.".to_string(),
            technologies: vec!["C#".to_string(), "Unity".to_string()],
            live_url: Some("https://yunyun0909.itch.io/behavior-tree-demo".to_string()), //
            repo_url: Some("https://github.com/Dubium0/BT-Implementation".to_string()),
            image_url: Some("images/behaviorTreeExample.png".to_string()), // Example: Can be jpg, png, gif etc.
            category : ProjectCategory::Other,
        },
        
        Project {
            title: "Nuts & Bolts Puzzle Game".to_string(),
            description: "The goal of the game is to release all the nuts from the bolts. Sounds simple, right? 
            :D I worked on this project during my internship at Any Games and gained valuable insights. 
            I learned a lot, particularly about modular design practices for games. ( Can't share the code because of ownership)".to_string(),
            technologies: vec!["C#".to_string(), "Unity".to_string()],
            live_url: None, //
            repo_url: None,
            image_url: Some("images/nutandbolts.gif".to_string()), // Example: Can be jpg, png, gif etc.
            category : ProjectCategory::Games,
        },
        
        Project {
            title: "Multiplayer Plate up clone".to_string(),
            description: "In this project I developed online futures. I used photon PUN 2. 
            I learned RPC ‘s to sync behaviors  and the other main ideas of online gameplay. Also, common software patterns utilized such as observer and mediator.
            ( Can't share the code because of ownership)".to_string(),
            technologies: vec!["C#".to_string(), "Unity".to_string(),"Photon".to_string(),"Networking".to_string()],
            live_url: None, //
            repo_url: None,
            image_url: Some("images/plateup.gif".to_string()), // Example: Can be jpg, png, gif etc.
            category : ProjectCategory::Games,
        },
        Project {
            title: "Battleship".to_string(),
            description: "This is a desktop version of the classic board game called BattleShip. Main purpose of this project is utilizing software engineering patterns. Project was implemented as a group project for the CS 320 ( Software Engineering )".to_string(),
            technologies: vec!["Java".to_string()],
            live_url: None, //
            repo_url: Some("https://github.com/Dubium0/BattleShip".to_string()),
            image_url: Some("images/battleship.gif".to_string()), // Example: Can be jpg, png, gif etc.
            category : ProjectCategory::Games,
        },
        
        
        // Add more projects here, including their image_url
        ];
        
        // Filter projects into separate lists
        let game_projects: Vec<Project> = all_projects
            .iter()
            .filter(|p| p.category == ProjectCategory::Games)
            .cloned()
            .collect();
        
        let other_projects: Vec<Project> = all_projects
            .iter()
        .filter(|p| p.category == ProjectCategory::Other)
        .cloned()
        .collect();

    // Helper function to render a list of projects
    let render_project_list = |projects_to_render: Vec<Project>| -> Html {
        if projects_to_render.is_empty() {
            html! {} // Don't render the list div if empty
        } else {
            html! {
                <div class="project-list">
                    {
                        projects_to_render.iter().map(|project_item| {
                            html! {
                                <ProjectCard key={project_item.title.clone()} project={project_item.clone()} />
                            }
                        }).collect::<Html>()
                    }
                </div>
            }
        }
    };

    html! {
        <section id="projects" class="content-section alt-bg projects-section-categorized"> // Added new class
            // Main section header remains constrained
            <h2>{ "Projects" }</h2>
            <p>{ "Here's a selection of work I'm proud of. Hover over cards for details!" }</p>

            // --- Personal Projects Subsection ---
            if !game_projects.is_empty() {
                <div class="project-category-group"> // Wrapper for category
                    <h3 class="project-category-title">{ "Game Projects" }</h3>
                    { render_project_list(game_projects) }
                </div>
            }

            // --- Group Projects Subsection ---
             if !other_projects.is_empty() {
                <div class="project-category-group"> // Wrapper for category
                    <h3 class="project-category-title">{ "Other Projects" }</h3>
                    { render_project_list(other_projects) }
                </div>
            }

            // Add more subsections here if you have other categories
        </section>
    }
}
