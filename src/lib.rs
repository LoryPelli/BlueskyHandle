use worker::{Context, Env, Request, Response, Result, Url, event};

macro_rules! did {
    () => {
        "did:plc:jai46evw5qma2hfcrq7mxyjc"
    };
}

macro_rules! dh {
    () => {
        "dh=18180114862af9d7aacc108e3c62aa172ae5e904"
    };
}

#[event(fetch)]
async fn fetch(req: Request, _: Env, _: Context) -> Result<Response> {
    let url = req.url()?;
    let well_known_uri = url.path().strip_prefix("/.well-known/");
    match well_known_uri {
        Some("atproto-did") => Response::ok(did!()),
        Some("discord") => Response::ok(dh!()),
        Some(_) | None => redirect(),
    }
}

fn redirect() -> Result<Response> {
    let url = bsky_url();
    Response::redirect(Url::parse(url)?)
}

const fn bsky_url<'a>() -> &'a str {
    concat!("https://bsky.app/profile/", did!())
}
