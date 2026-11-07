use serde::{Serialize, Deserialize };

// To recieve model lists from Ollama
#[derive(Deserialize, Debug)]
pub struct Details {
    format: String,
    family: String,
    families: Vec<String>,
    parameter_size: String,
    quantization_level: String,
}

#[derive(Deserialize, Debug)]
pub struct Model {
    pub name: String,
    pub model: String,
    modified_at: String,
    size: u64,
    digest: String,
    details: Details,
}

#[derive(Deserialize, Debug)]
pub struct Models {
    pub models: Vec<Model>,
}


// -------------------------------------
// /api/generate
// Generate response based on prompt
// -------------------------------------

// POST REQ:

#[derive(Serialize, Debug)]
pub struct GeneratePost {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
}
// OUTPUT:

#[derive(Deserialize, Debug)]
pub struct GenerateResponse {
    model: String,
    created_at: String,
    pub response: String,
    done: bool,
    done_reason: String,
    total_duration: u64,
    load_duration: u64,
    prompt_eval_count: u16,
    prompt_eval_duration: u64,
    eval_count: u16,
    eval_duration: u64,
}
