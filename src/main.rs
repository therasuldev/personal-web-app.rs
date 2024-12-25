use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="container">
            <nav class="navbar">
                <h2>{"My Yew App"}</h2>
                <div class="nav-links">
                    <a href="#">{"Home"}</a>
                    <a href="#">{"About"}</a>
                    <a href="#">{"Contact"}</a>
                </div>
            </nav>
            <main class="content">
                <h1>{"Welcome to Yew"}</h1>
                <p>{"Build awesome web apps with Rust"}</p>
                <button class="button">{"Get Started"}</button>
            </main>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}