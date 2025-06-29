use windows::Win32::System::SystemInformation::{GetSystemInfo, SYSTEM_INFO};
use std::mem;
use crate::detection::shared::Scorable;
use colored::Colorize;


fn sys_info() -> u32{
    // Proper way to call GetSystemInfo
    let sys_info = unsafe {
        let mut info: SYSTEM_INFO = mem::zeroed();
        GetSystemInfo(&mut info);
        info
    };
    return sys_info.dwNumberOfProcessors;
}

pub struct ProcChecker;
impl Scorable<u32> for ProcChecker {
    fn calculate_score(&self) -> f64 {
        let procnum = sys_info();
        if procnum <= 4
        { 0.8 } 
        else 
        { 0.0 }
    }
    fn weight(&self) -> f64 {
        0.3  // 30% weight for processor check
    }
    
    fn create_comment(&self) -> String {
        if self.calculate_score() != 0.0 {
            "Proc number too low (potato computer). Suspicious".red().to_string()
        } else {
            "Proc num OK".green().to_string()
        }
    }
    fn weighted_score(&self) -> f64 {
        self.calculate_score() * self.weight()
    }

    fn build_struct(&self) -> crate::detection::shared::CheckResult<u32> {
        let result: u32 = sys_info();
        crate::detection::shared::CheckResult::new(
            result,
            self.weight(),
            self.calculate_score(),
            self.weighted_score(), 
            self.create_comment()
        )
    }

}

/*
pub fn main () {
        let checker = ProcChecker;
        let result = checker.build_struct();
        println!("Uptime Results:");
        println!("- Value: {} processors", result.result);
        println!("- Score: {}", result.score);
        println!("- Weighted Score: {}", result.weighted_score);
        println!("- Comment: {}", result.comment);
    }

*/