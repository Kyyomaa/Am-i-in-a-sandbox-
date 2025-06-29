use sysinfo::Disks;
use crate::detection::shared::Scorable;
use crate::detection::shared::CheckResult;
use colored::Colorize;

pub fn main() {
}





/////////////
pub struct Gb;

impl Scorable<u64> for Gb {

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
        let disks = Disks::new_with_refreshed_list();
        let result: u64 = disks.list().iter()
        .map(|disk| disk.total_space())
        .sum();
        let score = if result <= 85899345920 { 0.7 } else { 0.0 };
        let weighted = score * self.weight();
        let comment = if score > 0.0 {
            "Less than 80 gb (potato computer?). Suspicious".red().to_string()
        } else {
            "Disk space OK".green().to_string()
        };
        CheckResult::new(result, self.weight(), score, weighted, comment)
    }
}