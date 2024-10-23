use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, MetaTags, Stylesheet};
use leptos_router::{
    components::{FlatRoutes, Route, Router}, ParamSegment, StaticSegment
};

use crate::components::{HomePage, TamuPage, UndanganPage};

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
        <Meta name="description" content="Undangan Pernikahan Dian dan Rafiyq"/>
        <Router>
            <FlatRoutes fallback=|| "Not found.">
                <Route path=(StaticSegment("tamu"), ParamSegment("username")) view=TamuPage/>
                <Route path=StaticSegment("undangan") view=UndanganPage/>
                <Route path=StaticSegment("") view=HomePage/>
            </FlatRoutes>
        </Router>
    }
}