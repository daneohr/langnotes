use serde::{Serialize, Deserialize };

// To recieve model lists from Ollama
#[derive(Serialize, Deserialize)]
struct Details {
    format: String,
    family: String,
    families: Vec<String>,
}


#[derive(Serialize, Deserialize)]
struct Model {
    name: String,
    model: String,
    size: usize,
    digest: String,
    details: Vec<Details>,
    parameter_size: String,
    quantization_level: String,
}

#[derive(Serialize, Deserialize)]
pub struct Models {
    models: Vec<Model>,
}
