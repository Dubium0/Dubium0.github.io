use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="portfolio-container">
            <header>
                <h1>{ "My Awesome Portfolio" }</h1>
                <nav>
                    <ul>
                        <li><a href="#about">{ "About" }</a></li>
                        <li><a href="#projects">{ "Projects" }</a></li>
                        <li><a href="#contact">{ "Contact" }</a></li>
                    </ul>
                </nav>
            </header>

            <main>
                <section id="about">
                    <h2>{ "About Me" }</h2>
                    <p>{ "Welcome! I'm a developer passionate about Rust and WebAssembly." }</p>
                    // Add more about yourself here
                </section>

                <section id="projects">
                    <h2>{ "Projects" }</h2>
                    // Add project components or details here later
                    <p>{ "Project details coming soon!" }</p>
                </section>

                <section id="contact">
                    <h2>{ "Contact" }</h2>
                    <p>{ "You can reach me at [your email]" }</p>
                    // Add a contact form or links here
                </section>
            </main>

            <footer>
                <p>{ "© 2025 Your Name" }</p>
            </footer>
        </div>
    }
}