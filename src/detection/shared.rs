// src/detection/shared.rs

// Make the trait generic by adding <T>
pub trait Scorable<T> {
    fn calculate_score(&self) -> f64;
    fn create_comment(&self) -> String;
    fn build_struct(&self) -> CheckResult<T>;
}

#[derive(Debug)]
pub struct CheckResult<T> {
    pub result: T,
    pub score: f64,
    pub comment: String
}

impl<T> CheckResult<T> {
    pub fn new(result: T, score: f64, comment: String) -> Self {
        Self { result, score, comment }
    }
}