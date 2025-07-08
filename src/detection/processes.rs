use sysinfo::System;
use std::ffi::OsStr;
use crate::detection::shared::{Scorable, CheckResult};
use colored::Colorize;

pub fn main() {
    let checker = ProcessChecker;
    let result = checker.build_struct();
    println!("Detection results: {:?}", result);
}

pub struct ProcessChecker;

impl Scorable<String> for ProcessChecker {  
    fn calculate_score(&self) -> f64 {
        let found = proc_detection();
    let suspicious = [
        OsStr::new("vmtoolsd.exe"),
        OsStr::new("vmwaretray.exe"),
        OsStr::new("vboxservice.exe"),
        OsStr::new("vboxtray.exe"),
    ];
    if found.iter().any(|p| suspicious.contains(&OsStr::new(p))) {
        return 0.9;
    }
    let len = found.len();
    if len >= 1 && len <= 2 {
        0.2
    } else if len >= 5 {
        0.8
    } else {
        0.0
    }
}
    
    fn weight(&self) -> f64 {
        0.8
    }
    
    fn create_comment(&self) -> String {
        let found = proc_detection();
        if !found.is_empty() {
            let mut output = "Found suspicious processes:".red().to_string();
            for name in found {
                output.push_str(&format!("\n {}", name));
            }
            output
        } else {
            "No suspicious processes found".green().to_string()
        }
    }
    
    fn build_struct(&self) -> CheckResult<String> { 
        let found = proc_detection();
        let result_string = if !found.is_empty() {
            found.join(", ") 
        } else {
            "None".to_string()
        };
        
        CheckResult::new(
            result_string,  
            self.weight(),
            self.calculate_score(),
            self.weighted_score(),
            self.create_comment()
        )
    }
}

fn proc_detection() -> Vec<String> {
    let processes = [

        OsStr::new("vmtoolsd.exe"), 
        OsStr::new("vmwaretray.exe"), 
        OsStr::new("vboxservice.exe"), 
        OsStr::new("vboxtray.exe"),
        OsStr::new("qemu-ga.exe"),
        OsStr::new("prl_cc.exe"),
        OsStr::new("xenservice.exe"),
        OsStr::new("ollydbg.exe"), 
        OsStr::new("x32dbg.exe"),
        OsStr::new("x64dbg.exe"), 
        OsStr::new("idaq.exe"),
        OsStr::new("windbg.exe"),
        OsStr::new("immunitydebugger.exe"),
        OsStr::new("ghidra.exe"),
        OsStr::new("procmon.exe"),
        OsStr::new("procexp.exe"),
        OsStr::new("wireshark.exe"),
        OsStr::new("fiddler.exe"),
        OsStr::new("tcpview.exe"),
        OsStr::new("regshot.exe"),
        OsStr::new("processhacker.exe"),
        OsStr::new("mcshield.exe"), 
        OsStr::new("cuckoo.exe"),
        OsStr::new("joebox.exe"),
        OsStr::new("anubis.exe"),
        OsStr::new("fakenet.exe"),
        OsStr::new("wireshark.exe"), 
        OsStr::new("sandboxie.exe")
    ];
    
    let s = System::new_all();
    let mut found = Vec::new();
    
    for name in processes.iter() {
        if s.processes_by_name(name).next().is_some() {
            found.push(name.to_string_lossy().into_owned());
        }
    }
    found
}