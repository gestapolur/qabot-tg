use serde_json;
use reqwest;

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Json(serde_json::Error);
        Http(reqwest::Error);
    }
}
