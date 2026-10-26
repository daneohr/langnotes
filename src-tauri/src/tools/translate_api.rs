use reqwest::{ post, HeaderMap };

pub async fn make_request(lang_code: String, content: String) {
    let mut params = HashMap::new();
    params.insert("q", content);
    params.insert("source", "en");
    params.insert("target", lang_code);
    params.insert("format", "text");
    params.insert("alternatives", 3);
    params.insert("api_key", "");

    let body = post("https://libretranslate.com/translate")
        .json(&params)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;

    print!(body);
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
