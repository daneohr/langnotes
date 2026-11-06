use tauri_plugin_http::reqwest;
use serde::{ Serialize, Deserialize };
use super::data_structs::Models;

// TODO: Add data structures in separate rust file

#[tauri::command]
pub async fn ollama_list() -> Result<Vec<String>, String>{

    let response = reqwest::get("http://100.74.159.59:11434/api/tags")
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    match response.status() {
        // Request is good, return list of model names
        reqwest::StatusCode::OK => {
            let response_json: Models = response.json().await.unwrap();
            let names: Vec<String> = response_json.models.
                into_iter()
                .map(|model| model.name)
                .collect();
            Ok(names)
        },
        // not found: no model (check if endpoint is correct)
       reqwest::StatusCode::NOT_FOUND => {
           Err("Error: Not found".into())
       },

       reqwest::StatusCode::REQUEST_TIMEOUT => {
           Err("Error: Request Timeout".into())
       },

       _ => {
           Err("Error: Not found".into())
       }
   }

}

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

//    #[test]
//    fn req() {
//        let result = tauri::async_runtime::block_on( async {
//            make_request("de".into(), "Hey, whats up".into()).await
//        });
//    }

    #[test]
    fn list() {
        let result = tauri::async_runtime::block_on( async {
            let list = ollama_list().await;
            match list {
                Ok(res) => {
                    println!("GOOD: {:?}", res);
                }
                Err(e) => {
                    println!("BAD: {}", e);
                }
            }
        });
    }
}

    // :lang fi
