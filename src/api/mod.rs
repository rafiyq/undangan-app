pub mod d1;

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    use leptos::server_fn::axum::register_explicit;

    register_explicit::<d1::GetGuest>();
}