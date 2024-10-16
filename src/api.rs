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
pub async fn get_bucket(filename: String) -> Result<Vec<u8>, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    use std::sync::Arc;
    use worker::*;
    
    let Extension(env): Extension<Arc<Env>> = extract().await?;
    let bucket = env.bucket("bucket")?;
    let object = bucket.get(filename).execute().await?;
    let body = object.unwrap().body().unwrap().bytes().await?; 

    Ok(body)
}

#[cfg_attr(feature = "ssr", worker::send)]
#[server(GetKV)]
pub async fn get_kv(key: String) -> Result<Option<Vec<u8>>, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    use std::sync::Arc;
    use worker::*;
    
    let Extension(env): Extension<Arc<Env>> = extract().await?;
    let kv = env.kv("static")?;
    let value = kv.get(&key).bytes().await?;

    Ok(value)
}

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    use leptos::server_fn::axum::register_explicit;

    register_explicit::<GetBucket>();
    register_explicit::<GetGuest>();
    register_explicit::<GetKV>();
}