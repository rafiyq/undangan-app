use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{FlatRoutes, Route, Router}, ParamSegment, StaticSegment
};
use crate::components::{countdown::Countdown, occasion::Occasion, greeting::Greeting};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="id">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

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
        <Meta name="description" content="Undangan Dian dan Rafiyq"/>
        <Router>
            <FlatRoutes fallback=|| "Not found.">
                <Route path=(StaticSegment("tamu"), ParamSegment("username")) view=HomePage/>
                <Route path=StaticSegment("undangan") view=UndanganPage/>
                <Route path=StaticSegment("") view=HomePage/>
            </FlatRoutes>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Title text="Beranda - Dian Rafiyq"/>
        <main class="flex min-h-dvh flex-col justify-center sm:py-12 bg-colour1 text-colour3 font-['Inter']">
            <article class="px-6 pt-10 pb-8 flex flex-col gap-6 justify-center sm:mx-auto sm:aspect-[3/4] sm:rounded-lg sm:px-10 sm:shadow-nm-flat">
                <Occasion />
                <Countdown />
                <Greeting />
                <a class="btn-primary" href="/undangan">Buka Undangan</a>
            </article>
        </main>
    }
}

#[component]
fn UndanganPage() -> impl IntoView {
    view! {
        <Title text="Undangan - Dian Rafiyq"/>
        <main class="flex flex-col justify-center bg-gray-500">
            <video autoplay loop controls controlslist="play timeline volume" class="h-dvh aspect-[9/16] mx-auto shadow-nm-flat"> 
                <source src="https://r2.dianrafiyq.site/Dian%26Rafiyq%20Live_20241015_183829_0001.mp4" type="video/mp4"/>
            </video>
        </main>
    }
}