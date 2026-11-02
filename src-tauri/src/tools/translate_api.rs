// use reqwest::{ post, HeaderMap };
use tauri_plugin_http::reqwest;
use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize)]
struct Headers {
    q: String,
    source: String,
    target: String,
    format: String,
    alternatives: u8,
    api_key: String
}

pub async fn make_request(lang_code: String, content: String) -> Result<String, String> {

    let client = reqwest::Client::new();

    let params = Headers {
        q: content,
        source: "en".into(),
        target: lang_code,
        format: "text".into(),
        alternatives: 3,
        api_key: "".into()
    };

    let body = client.post("https://libretranslate.com/translate")
        .json(&params)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send()
        .await?;

    print!("{}", body);

    Ok(format!("{}", body))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn req() {
        make_request("de".into(), "Hey, what's up".into());
    }
}

    // :lang fi
