use worker::{Context, Env, Request, Response, Result, Url, event};

macro_rules! prefix {
    ($suffix:expr) => {
        concat!("/.well-known/", $suffix)
    };
}

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

macro_rules! bsky_url {
    () => {
        concat!("https://bsky.app/profile/", did!())
    };
}

#[event(fetch)]
async fn fetch(req: Request, _: Env, _: Context) -> Result<Response> {
    match req.path().as_str() {
        prefix!("atproto-did") => Response::ok(did!()),
        prefix!("discord") => Response::ok(dh!()),
        _ => Response::redirect(to_url(bsky_url!())),
    }
}

fn to_url(s: &str) -> Url {
    Url::parse(s).unwrap()
}
