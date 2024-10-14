#[cfg(feature = "ssr")]
use worker::*;

pub mod app;
pub mod components;
pub mod api;

#[cfg(feature = "ssr")]
async fn router(env: Env) -> axum::Router {
    use axum::{routing::post, Extension, Router};
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, handle_server_fns, LeptosRoutes};
    use std::sync::Arc;
    use crate::app::{App, shell};
    use crate::api::register_server_functions;

    // Match what's in Cargo.toml
    // Doesn't seem to be able to do this automatically
    let leptos_options = LeptosOptions {
        output_name: "undangan_app".into(),
        site_root: "target/site".into(),
        site_pkg_dir: "pkg".into(),
        env: Env::DEV,
        site_addr: "127.0.0.1:8787".parse().unwrap(),
        reload_port: 3001,
        reload_external_port: None,
        reload_ws_protocol: ReloadWSProtocol::WS,
        not_found_path: "/404".into(),
        hash_file: "hash.txt".into(),
        hash_files: false,
    };

    let routes = generate_route_list(App);
    register_server_functions();

    Router::new()
        .route("/api/*fn_name", post(handle_server_fns))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .with_state(leptos_options)
        .layer(Extension(Arc::new(env)))
}

#[cfg(feature = "ssr")]
#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    use tower_service::Service;

    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    Ok(router(env).await.call(req).await?)
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::App;

    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}