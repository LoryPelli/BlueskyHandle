macro_rules! prefix {
    ($suffix:expr) => {
        concat!(".well-known/", $suffix)
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

macro_rules! icon {
    () => {
        "https://web-cdn.bsky.app/static/favicon-32x32.png"
    };
}

macro_rules! gh {
    () => {
        "https://github.com/lorypelli/blueskyhandle"
    };
}

macro_rules! bsky {
    () => {
        concat!("https://bsky.app/profile/", did!())
    };
}
