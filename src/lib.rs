mod templates;

use axum::{routing::get, Router};
use chrono::{DateTime, Utc};
use tower_service::Service;
use worker::{event, Context, Env, HttpRequest, Result};

const DT_UNDANGAN: &str = "2024-10-27T08:00:00+07:00";

fn router() -> Router {
    Router::new()
        .route("/", get(index))
        // .route_layer("/update", get(update))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    Ok(router().call(req).await?)
}

async fn index() -> templates::Index {
    let dt_remaining = DateTime::parse_from_rfc3339(DT_UNDANGAN).unwrap()
        .with_timezone(&Utc)
        .signed_duration_since(Utc::now());

    let day_time = templates::DayAndTime::from_timedelta(dt_remaining);

    templates::Index {
        title: "Beranda".to_string(), 
        guestname: "Dian".to_string(),
        countdown_ongoing: !day_time.is_timeout(),
        countdown_remaining: day_time,
    }
}

fn update() -> templates::Countdown {
    let dt_remaining = DateTime::parse_from_rfc3339(DT_UNDANGAN).unwrap()
        .with_timezone(&Utc)
        .signed_duration_since(Utc::now());
    
    templates::Countdown { 
        countdown_remaining: templates::DayAndTime::from_timedelta(dt_remaining) 
    }
}