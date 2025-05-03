use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BisectionRequest {
    pub polynomial: String,
    pub a: f64,
    pub b: f64,
}

#[derive(Serialize)]
pub struct BisectionResult {
    pub root: f64,
    pub iterations: usize,
    pub error: f64,
}