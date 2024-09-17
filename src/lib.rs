mod templates;
mod utils;

use askama::Template;
use templates::{Countdown, Guest, IndexTemplate, Remaining};
use worker::{console_log, event, query, Context, Env, Request, Response, Result, Router};

const DT_UNDANGAN: &str = "2024-10-27T08:00:00+07:00";

#[event(fetch, respond_with_errors)]
async fn fetch(
    req: Request,
    env: Env,
    _ctx: Context,
) -> Result<Response> {
    console_error_panic_hook::set_once();

    Router::new()
        .get_async("/", |_, _| async move {
            let dt_remaining = utils::make_duration(DT_UNDANGAN);
            let day_time = Remaining::from_timedelta(dt_remaining);
            let index = IndexTemplate {
                countdown_ongoing: !day_time.is_timeout(),
                remaining: day_time,
            };
            let html = index.render().unwrap();
            Response::from_html(html)
        })
        .get_async("/update", |_, _| async move {
            let dt_remaining = utils::make_duration(DT_UNDANGAN);
            let countdown = Countdown { 
                remaining: Remaining::from_timedelta(dt_remaining) 
            };
            let html = countdown.render().unwrap();
            Response::from_html(html)
        })
        .get_async("assets/:filename", |_req, ctx| async move {
            console_log!("hello");
            if let Some(filename) = ctx.param("filename") {
                let assets = ctx.kv("assets")?;
                console_log!("filename: {}", filename);
                return match assets.get(filename).bytes().await? {
                    Some(file_content) => Response::from_bytes(file_content),
                    None => Response::error("Not found", 404)
                    
                };
            }
            Response::error("Bad Request", 400)
        })
        .get_async("/tamu/:username", |_, ctx| async move {
            if let Some(username) = ctx.param("username") {
                let d1 = ctx.env.d1("guest-list")?;
			    let query = query!(
                    &d1,
                    "SELECT * FROM Guests WHERE UserName = ?1",
                    &username,
                )?;
			    let result = query.first::<Guest>(None).await?;
			    let _ = match result {
				    Some(guest) => Response::from_json(&guest),
				    None => Response::error("Not found", 404),
			    };
            }
            Response::error("Bad Request", 400)
        })
        .run(req, env).await
}