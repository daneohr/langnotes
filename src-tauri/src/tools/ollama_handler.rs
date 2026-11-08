use tauri_plugin_http::reqwest;
use serde::{ Serialize, Deserialize };
use super::data_structs::{ Models, GeneratePost, GenerateResponse };

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

pub async fn make_request(lang_code: String, content: String) -> Result<String, String> {

    let client = reqwest::Client::new();

    let params = GeneratePost {
        model: "translategemma:4b".into(),
        prompt: "How do you say say hello in Mandarin Chinese? Provide 3 different alternatives.".into(),
        stream: false
    };

    let response = client.post("http://100.74.159.59:11434/api/generate")
        .json(&params)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    if response.status().is_success() {
        let response_json: GenerateResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        Ok(response_json.response)
    } else {
        Err("An unknown error occured".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn req() {
        let result = tauri::async_runtime::block_on( async {
            let response = make_request("de".into(), "Hey, whats up".into()).await.unwrap();
        });
    }

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
