mod templates;
mod utils;

use askama::Template;
use templates::{Countdown, Guest, IndexTemplate, Remaining};
use utils::get_content_type;
use worker::{console_log, event, Context, Env, Headers, Request, Response, Result, Router};

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
        .get_async("/", |_, _| async move {
            let remaining = utils::make_duration(DT_UNDANGAN);
            let index = IndexTemplate { 
                guest: None,
                remaining: Remaining::new(remaining)
            };
            let html = index.render().unwrap();
            Response::from_html(html)
        })
        .get_async("/update", |_, _| async move {
            let remaining = utils::make_duration(DT_UNDANGAN);
            let countdown = Countdown { 
                remaining: Remaining::new(remaining) 
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
                    Some(file_content) => {
                        let mut headers = Headers::new();
                        headers.set("Content-Type", get_content_type(filename))?;
                        Ok(Response::from_bytes(file_content)?.with_headers(headers))
                    },
                    None => Response::error("Not found", 404)
                    
                };
            }
            Response::error("Bad Request", 400)
        })
        .get_async("/tamu/:username", |_, ctx| async move {
            let username = ctx.param("username").unwrap();
            let d1 = ctx.env.d1("DB")?;
			let statement = d1.prepare("SELECT * FROM Guests WHERE username = ?1");
			let query = statement.bind(&[username.into()])?;
			let result = query.first::<Guest>(None).await?;
			match result {
				Some(guest) => {
                    let remaining = utils::make_duration(DT_UNDANGAN);
                    let index = IndexTemplate { 
                        guest: Some(guest),
                        remaining: Remaining::new(remaining)
                    };
                    let html = index.render().unwrap();
                    Response::from_html(html)
                },
				None => Response::error("Not found", 404),
			}
        })
        .run(req, env).await
}