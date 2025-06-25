// src/detection/shared.rs

// Make the trait generic by adding <T>
pub trait Scorable<T> {
    fn calculate_score(&self) -> f64;
    fn weight(&self) -> f64;  // Add this for weighted scoring
    fn create_comment(&self) -> String;
    fn build_struct(&self) -> CheckResult<T>;
    fn weighted_score(&self) -> f64 {
        self.calculate_score() * self.weight()
    }

}

#[derive(Debug)]
pub struct CheckResult<T> {
    pub result: T,
    pub weight: f64,
    pub score: f64,
    pub weighted_score: f64,
    pub comment: String
}

impl<T> CheckResult<T> {
    pub fn new(result: T, weight: f64, score: f64, weighted_score: f64, comment: String) -> Self {
        Self { result, weight, score, weighted_score, comment }
    }
}