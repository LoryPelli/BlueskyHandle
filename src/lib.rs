use worker::{Context, Env, Request, Response, Result, Url, event};

const DID: &str = "did:plc:jai46evw5qma2hfcrq7mxyjc";

const DH: &str = "dh=18180114862af9d7aacc108e3c62aa172ae5e904";

#[event(fetch)]
async fn fetch(req: Request, _: Env, _: Context) -> Result<Response> {
    let url = req.url()?;
    let path = url.path();
    let well_known_uri = path.strip_prefix("/.well-known/");
    match well_known_uri {
        Some("atproto-did") => Response::ok(DID),
        Some("discord") => Response::ok(DH),
        Some(_) | None => redirect(),
    }
}

fn redirect() -> Result<Response> {
    let url = format!("https://bsky.app/profile/{DID}");
    Response::redirect(Url::parse(&url)?)
}
