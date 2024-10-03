use leptos::*;
use leptos_meta::{provide_meta_context, Link, Stylesheet, Title};
use leptos_router::*;
use crate::components::{countdown::Countdown, occasion::Occasion};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous"/>
        <Stylesheet id="google-font" href="https://fonts.googleapis.com/css2?family=Dancing+Script:wght@400..700&family=Gowun+Batang:wght@400;700&family=Merriweather:wght@700;900&family=Inter:ital,opsz,wght@0,14..32,100..900;1,14..32,100..900&display=swap"/>
        <Stylesheet id="trajan-pro" href="https://fonts.cdnfonts.com/css/trajan-pro"/>
        <Stylesheet id="leptos" href="/pkg/undangan_app.css"/>
        <Link rel="preload" type_="image/x-icon" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=HomePage/>
                <Route path="/undangan" view=UndanganPage/>
            </Routes>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let rfc3339 = "2024-10-27T08:00:00+07:00";

    view! {
        <Title text="Beranda - Dian Rafiyq"/>
        <main id="index">
            <Occasion />
            <Countdown rfc3339 />
            // <Greeting />
            <button class="btn-primary" href="/undangan">Buka Undangan</button>
        </main>
    }
}

#[component]
fn UndanganPage() -> impl IntoView {
    view! {
        <h1>"Undangan Page"</h1>
    }
}