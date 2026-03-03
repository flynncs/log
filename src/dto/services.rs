use serde::Serialize;

#[derive(Serialize)]
pub struct ServicesResponse {
    pub services: Vec<String>,
}
