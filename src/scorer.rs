use crate::detection;
use crate::detection::shared::Scorable;
use colored::Colorize;

pub fn score() {
    // Create all your checkers
    let uptime_checker: detection::uptime::UptimeChecker = detection::uptime::UptimeChecker;
    let proc_checker: detection::processor::ProcChecker = detection::processor::ProcChecker;
    let ram_checker:detection::ram::Mem = detection::ram::Mem;
    // Add more checkers here...

    // Store all check results as strings
    let mut check_results = Vec::new();
    let mut total_weighted_score = 0.0;

    let uptime_result = uptime_checker.build_struct();
    total_weighted_score += uptime_result.weighted_score;
    check_results.push((
        "Uptime Check",
        uptime_result.result.to_string(), 
        uptime_result.weighted_score,
        uptime_result.comment
    ));

    let proc_result = proc_checker.build_struct();
    total_weighted_score += proc_result.weighted_score;
    check_results.push((
        "Processor Check",
        proc_result.result.to_string(), 
        proc_result.weighted_score,
        proc_result.comment
    ));

    let ram_result:detection::shared::CheckResult<u64> = ram_checker.build_struct();
        total_weighted_score += ram_result.weighted_score;
         check_results.push((
        "Ram Check",
        ram_result.result.to_string(), 
        ram_result.weighted_score,
        ram_result.comment
    ));

    // Add more checks here following the same pattern...

    println!("=== Sandbox Detection Results ===");
    for (name, value, score, comment) in check_results {
        println!("\n{}:", name);
        println!("- Value: {}", value); 
        println!("- Weighted Score: {:.2}", score);
        println!("- Comment: {}", comment);
    }

    println!("\n=== Final Score ===");
    println!("Total Weighted Score: {:.2}", total_weighted_score);


if total_weighted_score > 0.5 {
    println!("{}", "VERDICT: Suspicious - possible sandbox".red());
} else {
    println!("{}", "VERDICT: Likely genuine system".green());
}
}