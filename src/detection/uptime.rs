// src/detection/uptime.rs
use windows::Win32::System::SystemInformation::GetTickCount64;
use crate::detection::shared::Scorable;
use colored::Colorize;

pub struct UptimeChecker;

impl Scorable<u64> for UptimeChecker {
    fn calculate_score(&self) -> f64 {
        let uptime = unsafe { GetTickCount64() };
        if uptime <= 1800000 
        { 0.7 } 
        else 
        { 0.0}
    }
    fn weight(&self) -> f64 {
        0.7 
    }
    
    fn create_comment(&self) -> String {
        if self.calculate_score() != 0.00 {
            "Tick count too low. Suspicious".red().to_string()
        } else {
            "Tick count OK".to_string()
        }
    }
    fn weighted_score(&self) -> f64 {
        self.calculate_score() * self.weight()
    }

    fn build_struct(&self) -> crate::detection::shared::CheckResult<u64> {
        let result = unsafe { GetTickCount64() };
        crate::detection::shared::CheckResult::new(
            result,
            self.weight(),
            self.calculate_score(),
            self.weighted_score(), 
            self.create_comment(),
        )
    }


}

    /* pub fn main() {
        let checker = UptimeChecker;
        let result = checker.build_struct();
        
        println!("Uptime Results:");
        println!("- Value: {} ms", result.result);
        println!("- Score: {}", result.score);
        println!("- Weighted Score: {}", result.weighted_score);
        println!("- Comment: {}", result.comment);
    } 
    */