use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Guest {
	pub username: String,
	pub fullname: String,
    pub title: String,
	pub address: String,
    pub session: String
}

#[cfg_attr(feature = "ssr", worker::send)]
#[server(GetGuest)]
pub async fn get_guest(username: String) -> Result<Option<Guest>, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    use std::sync::Arc;
    use worker::*;
    
    let Extension(env): Extension<Arc<Env>> = extract().await?;
    let d1 = env.d1("DB")?;
    let statement = d1.prepare("SELECT * FROM Guests WHERE username = ?1");
    let query = statement.bind(&[username.into()])?;

    let result = query.first::<Guest>(None).await.unwrap();
    Ok(result)
}

#[cfg_attr(feature = "ssr", worker::send)]
#[server(GetBucket)]
pub async fn get_bucket(filename: String) -> Result<worker::Response, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    use std::sync::Arc;
    use worker::*;
    
    let Extension(env): Extension<Arc<Env>> = extract().await?;
    let bucket = env.bucket("bucket")?;

    let object = bucket.get(filename).execute().await?;
    let body = object.body().unwrap().bytes().await?;
    let http_header = object.http_metadata().content_type.unwrap_or("application/octet-stream".to_string());
    let mut headers = Headers::new();
    headers.set("Content-Type", &http_header)?;
    Ok(Response::from_bytes(body)?.with_headers(headers))
}

#[cfg_attr(feature = "ssr", worker::send)]
#[server(GetKV)]
pub async fn get_kv(key: String) -> Result<worker::Response, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    use std::sync::Arc;
    use worker::*;
    
    let Extension(env): Extension<Arc<Env>> = extract().await?;
    let kv = env.kv("static")?;
    let value = kv.get(key).bytes().await?;

    let mime = asset_str.rsplit_once('.').map_or_else(
        || "text/plain",
        |(_, ext)| match ext {
            "css" => "text/css",
            "html" => "text/html",
            "js" => "application/javascript",
            "json" => "application/json",
            "ico" => "image/x-icon",
            "xml" => "application/xml",
            _ => "text/plain",
        },
    );

    let mut headers = Headers::new();
    headers.set("Content-Type", mime)?;
    Ok(Response::from_bytes(file_content)?.with_headers(headers))
}

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    use leptos::server_fn::axum::register_explicit;

    register_explicit::<GetGuest>();
    register_explicit::<GetBucket>();
    register_explicit::<GetKV>();
}