mod templates;
mod utils;

use askama::Template;
use templates::{Countdown, Guest, IndexTemplate, Remaining};
use utils::get_content_type;
use worker::{event, Context, Env, Headers, Request, Response, Result, Router};

const DT_UNDANGAN: &str = "2024-10-27T08:00:00+07:00";
const _DATE_TIME: &str = "27-Okt-2024 08:00:00 +0700";

#[event(fetch, respond_with_errors)]
async fn fetch(
    req: Request,
    env: Env,
    _ctx: Context,
) -> Result<Response> {
    console_error_panic_hook::set_once();

    Router::new()
        .get_async("/", |_, _| async move { index(None) })
        .get_async("/countdown", |_, _| async move { countdown() })
        .get_async("/tamu/:username", |_, ctx| async move {
            let username = ctx.param("username").unwrap();
            let d1 = ctx.env.d1("DB")?;
            let statement = d1.prepare("SELECT * FROM Guests WHERE username = ?1");
            let query = statement.bind(&[username.into()])?;

            match query.first::<Guest>(None).await? {
                Some(guest) => index(Some(guest)),
                None => Response::error("Not found", 404),
            }
        })
        .get_async("/assets/:filename", |_, ctx| async move {
            let filename = ctx.param("filename").unwrap();
            let assets = ctx.kv("assets")?;

            return match assets.get(filename).bytes().await? {
                Some(file_content) => {
                    let mut headers = Headers::new();
                    headers.set("Content-Type", get_content_type(filename))?;
                    Ok(Response::from_bytes(file_content)?.with_headers(headers))
                },
                None => Response::error("Item Not Found", 404)
            };
        })
        .get_async("/bucket/:filename", |_, ctx| async move {
            let filename = ctx.param("filename").unwrap();
            let bucket = ctx.bucket("bucket")?;

            return match bucket.get(filename).execute().await? {
                Some(object) => {
                    let body = object.body().unwrap().bytes().await?;
                    let http_header = object.http_metadata().content_type.unwrap_or("application/octet-stream".to_string());
                    let mut headers = Headers::new();
                    headers.set("Content-Type", &http_header)?;
                    Ok(Response::from_bytes(body)?.with_headers(headers))
                },
                None => Response::error("Item Not Found", 404)
            };
        })
        .run(req, env).await
}

fn index(guest: Option<Guest>) -> Result<Response> {
    let index = IndexTemplate { 
        guest,
        remaining: Remaining::from_rfc3339(DT_UNDANGAN)
    };
    Response::from_html(index.render().unwrap())
}

fn countdown() -> Result<Response> {
    let countdown = Countdown {
        remaining: Remaining::from_rfc3339(DT_UNDANGAN)
    };
    Response::from_html(countdown.render().unwrap())
}