use crate::detection::{self, processes};
use crate::detection::shared::Scorable;
use colored::Colorize;

//Probabilistic Detection 
pub fn score() {
    // Initialize all checkers
    let uptime_checker: detection::uptime::UptimeChecker = detection::uptime::UptimeChecker;
    let proc_checker: detection::processor::ProcChecker = detection::processor::ProcChecker;
    let ram_checker: detection::ram::Mem = detection::ram::Mem;
    let gb_checker: detection::memory::Gb= detection::memory::Gb;
    let mac_checker: detection::mac::MacChecker=  detection::mac::MacChecker;
    let processes_checker: detection::processes::ProcessChecker = detection::processes::ProcessChecker;
    let files_checker:  detection::files::FilesChecker = detection::files::FilesChecker;
   
    let mut check_results = Vec::new();
    let mut combined_probability = 1.0; // Start with 100% chance of being genuine

    let uptime_result = uptime_checker.build_struct();
    combined_probability *= 1.0 - uptime_result.weighted_score;
    check_results.push((
        "Uptime Check",
        uptime_result.result.to_string(),
        uptime_result.weighted_score,
        uptime_result.comment
    ));

    let proc_result = proc_checker.build_struct();
    combined_probability *= 1.0 - proc_result.weighted_score;
    check_results.push((
        "Processor Check",
        proc_result.result.to_string(),
        proc_result.weighted_score,
        proc_result.comment
    ));

    let ram_result = ram_checker.build_struct();
    combined_probability *= 1.0 - ram_result.weighted_score;
    check_results.push((
        "RAM Check",
        ram_result.result.to_string(),
        ram_result.weighted_score,
        ram_result.comment
    ));

    let gb_result = gb_checker.build_struct();
    combined_probability *= 1.0 - gb_result.weighted_score;
    check_results.push((
        "Disk Check",
        gb_result.result.to_string(),
        gb_result.weighted_score,
        gb_result.comment
    ));

    let mac_result = mac_checker.build_struct();
    combined_probability *= 1.0 - mac_result.weighted_score;
    check_results.push((
        "Mac address Check",
        mac_result.result.to_string(),
        mac_result.weighted_score,
        mac_result.comment
    ));

    let processes_result = processes_checker.build_struct();
    combined_probability *= 1.0 - processes_result.weighted_score;
    check_results.push((
        "Suspicious running processes Check",
        processes_result.result.to_string(),
        processes_result.weighted_score,
        processes_result.comment
    ));

    let files_result = files_checker.build_struct();
    combined_probability *= 1.0 - files_result.weighted_score;
    check_results.push((
        "Suspicious Files Check",
        files_result.result.to_string(),
        files_result.weighted_score,
        files_result.comment
    ));



    // ill Add more checks ...

    
    let sandbox_probability: f64 = 1.0 - combined_probability;

    // Print results
    println!("=== Sandbox Detection Results ===");
    for (name, value, score, comment) in check_results {
        println!("\n{}:", name);
        println!("- Value: {}", value);
        println!("- Weighted Score: {:.2}", score);
        println!("- Comment: {}", comment);
    }

    println!("\n=== Final Probability ===");
    println!("Sandbox Probability: {:.2}", sandbox_probability);

    // verdict
    if sandbox_probability > 0.5 {
        println!("{}", "VERDICT: Suspicious - possible sandbox".red());
    } else if sandbox_probability > 0.3 {
        println!("{}", "VERDICT: Suspicious - needs further analysis".yellow());
    } else {
        println!("{}", "VERDICT: Likely genuine system".green());
    }
}