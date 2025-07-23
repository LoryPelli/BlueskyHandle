use worker::{Context, Env, Request, Response, Result, event};

mod macros;

#[event(fetch)]
async fn fetch(req: Request, _: Env, _: Context) -> Result<Response> {
    match req.path().as_str() {
        prefix!("atproto-did") => Response::ok(did!()),
        prefix!("discord") => Response::ok(dh!()),
        _ => Response::redirect_str(bsky!()),
    }
}

trait Redirect: Sized {
    fn redirect_str(s: &str) -> Result<Self>;
}

impl Redirect for Response {
    fn redirect_str(s: &str) -> Result<Self> {
        Self::redirect(s.parse()?)
    }
}
