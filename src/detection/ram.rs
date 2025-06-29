use sysinfo::{
    System,
};
use crate::detection::shared::Scorable;
use crate::detection::shared::CheckResult;
use colored::Colorize;

fn get_ram_mem() -> u64 {
        let mut sys = System::new_all();
        sys.refresh_all();
        sys.total_memory()
    }

pub struct Mem;

impl Scorable<u64> for Mem {

    fn calculate_score(&self) -> f64 {
        0.0
    }

    fn weight(&self) -> f64  {
        0.5
    }
    
    fn create_comment(&self) -> String  {
        ".".to_string()
    }
    fn weighted_score(&self) -> f64 {
        0.0
    }

// implemented directly in the struct bc more efficient, otherwise i would have to call 
// refresh_all 3 times
    fn build_struct(&self) -> crate::detection::shared::CheckResult<u64> {
        let result = get_ram_mem();  // cache the value here
        let score = if result <= 4294967296 { 0.7 } else { 0.0 };
        let weighted = score * self.weight();
        let comment = if score > 0.0 {
            "Ram too low (potato computer?). Suspicious".red().to_string()
        } else {
            "RAM mem OK".green().to_string()
        };
        CheckResult::new(result, self.weight(), score, weighted, comment)
    }
}

/*
pub fn main () {
        let checker = Mem;
        let result = checker.build_struct();
        println!("Uptime Results:");
        println!("- Value: {} memory", result.result);
        println!("- Score: {}", result.score);
        println!("- Weighted Score: {}", result.weighted_score);
        println!("- Comment: {}", result.comment);
    }

*/