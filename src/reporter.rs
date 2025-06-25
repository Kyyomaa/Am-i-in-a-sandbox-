pub struct ScoreResult {
    pub score: f64,
    pub reason: String,
}

pub trait Scorer {
    fn score(&self, data: &str) -> ScoreResult;
}
