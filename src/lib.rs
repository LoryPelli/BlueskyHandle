use worker::{Context, Env, Request, Response, Result, Url, event};

const DID: &str = "did:plc:jai46evw5qma2hfcrq7mxyjc";

#[event(fetch)]
async fn fetch(req: Request, _: Env, _: Context) -> Result<Response> {
    let is_well_known = req.url()?.path() == "/.well-known/atproto-did";
    if is_well_known {
        Response::ok(DID)
    } else {
        let url = format!("https://bsky.app/profile/{DID}");
        Response::redirect(Url::parse(&url)?)
    }
}
