use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="container" style="font-family: 'Helvetica Neue', Arial, sans-serif; background-color: #fefefe; color: #444; padding: 30px; line-height: 1.6;">

            <header class="header" style="background-color: #e0f7fa; color: #00796b; padding: 20px; text-align: center;">
                <h1 style="margin: 0; font-size: 2.5rem;">{"Rasul Ramixanov"}</h1>
                <p style="margin: 5px 0 0; font-size: 1.2rem;">{"Full-Stack Developer | Proficient in Flutter | Exploring Rust & Swift | Open Source Contributor"}</p>
            </header>
            <nav class="nav" style="margin-top: 20px; text-align: center;">
                <a href="#about" style="color: #00796b; text-decoration: none; padding: 10px 15px;">{"About"}</a>
                <a href="#portfolio" style="color: #00796b; text-decoration: none; padding: 10px 15px;">{"Portfolio"}</a>
                <a href="#contact" style="color: #00796b; text-decoration: none; padding: 10px 15px;">{"Contact"}</a>
            </nav>
            <main class="main-content" style="margin-top: 40px;">
                <section id="about" class="about" style="padding: 20px;">
                    <h2 style="color: #00796b;">{"About Me"}</h2>
                    <p>{"I am a dedicated software developer with over 4 years of experience in crafting cross-platform mobile applications using Flutter..."}</p>
                </section>

                <section id="portfolio" class="portfolio" style="margin-top: 40px;">
                    <h2 style="color: #00796b;">{"Projects"}</h2>
                    <div style="display: flex; flex-wrap: wrap; gap: 20px; justify-content: center; margin-top: 20px;">


                    </div>
                </section>

                <section id="contact" class="contact" style="margin-top: 40px; padding: 20px;">
                    <h2 style="color: #00796b;">{"Contact"}</h2>
                    <p>{"Feel free to reach out to me through the following platforms:"}</p>
                    <div style="display: flex; justify-content: center; gap: 15px; margin-top: 15px;">

                    </div>
                </section>
            </main>
            <footer class="footer" style="background-color: #e0f7fa; color: #00796b; padding: 10px; text-align: center; margin-top: 40px;">
                <p>{"© 2024 Rasul Ramixanov. All rights reserved."}</p>
            </footer>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
