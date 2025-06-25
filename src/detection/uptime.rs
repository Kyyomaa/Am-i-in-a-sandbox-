// src/detection/uptime.rs
use windows::Win32::System::SystemInformation::GetTickCount64;
use crate::detection::shared::Scorable;

pub struct UptimeChecker;

impl Scorable<u64> for UptimeChecker {
    fn calculate_score(&self) -> f64 {
        let uptime = unsafe { GetTickCount64() };
        if uptime <= 1800000 
        { 0.35 } 
        else 
        { 0.0 }
    }

    fn create_comment(&self) -> String {
        if self.calculate_score() == 0.35 {
            "Tick count too low. Suspicious".into()
        } else {
            "Tick count OK".into()
        }
    }

    fn build_struct(&self) -> crate::detection::shared::CheckResult<u64> {
        let result = unsafe { GetTickCount64() };
        crate::detection::shared::CheckResult::new(
            result,
            self.calculate_score(),
            self.create_comment()
        )
    }


}

    pub fn main() {
        let checker = UptimeChecker;
        let result = checker.build_struct();
        
        println!("Uptime Results:");
        println!("- Value: {} ms", result.result);
        println!("- Score: {}", result.score);
        println!("- Comment: {}", result.comment);
    }