use serde::{Serialize, Deserialize };

// To recieve model lists from Ollama
#[derive(Serialize, Deserialize, Debug)]
pub struct Details {
    format: String,
    family: String,
    families: Vec<String>,
    parameter_size: String,
    quantization_level: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub name: String,
    pub model: String,
    modified_at: String,
    size: usize,
    digest: String,
    details: Details,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Models {
    pub models: Vec<Model>,
}
