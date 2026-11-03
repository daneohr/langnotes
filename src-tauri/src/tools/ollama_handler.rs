use tauri_plugin_http::reqwest;
use serde::{ Serialize, Deserialize };


#[derive(Debug, Serialize)]
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

    println!("{:?}", params);

    let body = client.post("https://libretranslate.com/translate")
        .json(&params)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    match body {
        Ok(response) => {
            println!("{}", response.status());
            Ok(format!("{}", response.status()))
        }
        Err(e) => {
            println!("{}", e);
            Err(format!("ERROR: {}", e))
        }
    }

    //Ok(String::from("Yoo"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn req() {
        let result = tauri::async_runtime::block_on( async {
            println!("TEST TEST TEST");
            make_request("de".into(), "Hey, whats up".into()).await
        });
    }
}

    // :lang fi
