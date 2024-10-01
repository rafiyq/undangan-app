use leptos::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::*;
use crate::components::countdown::Countdown;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/undangan_app.css"/>
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/undangan" view=UndanganPage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let rfc3339 = "2024-10-27T08:00:00+07:00";

    view! {
        <Title text="Welcome"/>
        <Countdown rfc3339 />
        <h1>"Hello world!"</h1>
    }
}

#[component]
fn UndanganPage() -> impl IntoView {
    view! {
        <h1>"Undangan Page"</h1>
    }
}