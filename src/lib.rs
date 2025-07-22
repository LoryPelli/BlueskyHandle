use worker::{Context, Env, Request, Response, Result, event};

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

macro_rules! bsky {
    () => {
        concat!("https://bsky.app/profile/", did!())
    };
}

trait Redirect: Sized {
    fn redirect_str(s: &str) -> Result<Self>;
}

impl Redirect for Response {
    fn redirect_str(s: &str) -> Result<Self> {
        Self::redirect(s.parse()?)
    }
}

#[event(fetch)]
async fn fetch(req: Request, _: Env, _: Context) -> Result<Response> {
    match req.path().as_str() {
        prefix!("atproto-did") => Response::ok(did!()),
        prefix!("discord") => Response::ok(dh!()),
        _ => Response::redirect_str(bsky!()),
    }
}
