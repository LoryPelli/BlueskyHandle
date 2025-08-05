use worker::{Context, Env, Request, Response, Result, event};

#[macro_use]
mod macros;

#[event(fetch)]
async fn fetch(req: Request, _: Env, _: Context) -> Result<Response> {
    match &req.path()[1..] {
        "favicon.ico" => Response::redirect_str(icon!()),
        prefix!("atproto-did") => Response::ok(did!()),
        prefix!("discord") => Response::ok(dh!()),
        "_gh" => Response::redirect_str(gh!()),
        _ => Response::redirect_str(bsky!()),
    }
}

trait Redirect: Sized {
    fn redirect_str(s: &str) -> Result<Self>;
}

impl Redirect for Response {
    #[inline]
    fn redirect_str(s: &str) -> Result<Self> {
        Self::redirect(s.parse()?)
    }
}
