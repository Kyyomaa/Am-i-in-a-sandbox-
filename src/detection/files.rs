use crate::detection::shared::{Scorable, CheckResult};
use colored::Colorize;
use std::path::Path;

    fn file_detection() -> Vec<String> {
        let paths = [
            //vmware
        r"C:\Windows\System32\drivers\vmhgfs.sys",
        r"C:\Windows\System32\drivers\vmmouse.sys",
        r"C:\Windows\System32\drivers\vm3dgl.dll",
        r"C:\Windows\System32\drivers\vmtray.dll",
        r"C:\Windows\System32\drivers\vmx_svga.sys",
        r"C:\Windows\System32\drivers\vmrawdsk.sys",
        r"C:\Windows\System32\drivers\vmusbmouse.sys",
        r"C:\Windows\System32\drivers\vmusb.sys",
        r"C:\Windows\System32\drivers\vmci.sys",
        r"C:\Windows\System32\drivers\vmnet.sys",
        r"C:\Windows\System32\drivers\vmnetadapter.sys",
        r"C:\Windows\System32\vmGuestLib.dll",
        r"C:\Windows\System32\vmhgfs.dll",
        r"C:\Windows\System32\vmtools.dll",
        r"C:\Program Files\VMware\VMware Tools\vmtoolsd.exe",
        //vbox
        r"C:\Windows\System32\VBoxControl.exe",
        r"C:\Windows\System32\VBoxService.exe",
        r"C:\Windows\System32\VBoxTray.exe",
        r"C:\Windows\System32\drivers\VBoxGuest.sys",
        r"C:\Windows\System32\drivers\VBoxMouse.sys",
        r"C:\Windows\System32\drivers\VBoxSF.sys",
        r"C:\Windows\System32\drivers\VBoxVideo.sys",
        r"C:\Program Files\Oracle\VirtualBox Guest Additions\VBoxControl.exe",
        //sandboxie
        r"C:\Program Files\Sandboxie\SbieCtrl.exe",
        r"C:\Windows\System32\drivers\SbieDrv.sys",
        //CUCKOO
        r"C:\Windows\System32\cuckoomon.dll",
        r"C:\cuckoo\agent.py",
        r"C:\cuckoo\",
        //joe snbx
        r"C:\Windows\System32\drivers\jsmonitor.sys",
        r"C:\JoeSandbox\agent.exe",
        r"C:\JoeSandbox\",
        //tools
        r"C:\Program Files\Autoruns",
        r"C:\Program Files (x86)\Fiddler",
        r"C:\Program Files (x86)\Fiddler2\Settings",
        r"C:\Program Files (x86)\OllyDbg",
        r"C:\Program Files\OllyDbg",
        r"C:\Program Files\x64dbg",
        r"C:\Program Files\x32dbg",
        r"C:\Program Files\IDA Pro",
        r"C:\Program Files\IDA",
        r"C:\Program Files (x86)\IDA Pro",
        r"C:\Program Files\Debugging Tools for Windows",
        r"C:\Program Files (x86)\Windows Kits\10\Debuggers",
        r"C:\Program Files (x86)\Immunity Inc\Immunity Debugger",
        r"C:\ghidra",
        r"C:\Program Files\ghidra",
        r"C:\Users\<User>\.ghidra",
        r"C:\Program Files\Procmon",
        r"C:\Program Files\Sysinternals",
        r"C:\Program Files\Process Explorer",
        r"C:\Program Files\Wireshark",
        r"C:\Program Files (x86)\Wireshark",
        r"C:\Users\<User>\AppData\Roaming\Wireshark",
        r"C:\Program Files\TcpView",
        r"C:\Program Files\RegShot",
        r"C:\Program Files\Process Hacker",
        r"C:\Program Files (x86)\Process Hacker",
        r"C:\Program Files\Malwarebytes",
        r"C:\ProgramData\Malwarebytes",
        r"C:\Program Files\Sandboxie",
        r"C:\Program Files\Sandboxie-Plus",
        r"C:\Sandbox",
        r"C:\Windows\System32\drivers\mbam.sys",
        r"C:\Windows\System32\drivers\SbieDrv.sys",
        r"C:\Windows\Prefetch\autoruns.exe-*.pf",
        r"C:\Windows\Prefetch\fiddler.exe-*.pf",
        r"C:\Windows\Prefetch\ollydbg.exe-*.pf",
        r"C:\Windows\Prefetch\x32dbg.exe-*.pf",
        r"C:\Windows\Prefetch\x64dbg.exe-*.pf",
        r"C:\Windows\Prefetch\idaq.exe-*.pf",
        r"C:\Windows\Prefetch\windbg.exe-*.pf",
        r"C:\Windows\Prefetch\immunitydebugger.exe-*.pf",
        r"C:\Windows\Prefetch\ghidra.exe-*.pf",
        r"C:\Windows\Prefetch\procmon.exe-*.pf",
        r"C:\Windows\Prefetch\procexp.exe-*.pf",
        r"C:\Windows\Prefetch\wireshark.exe-*.pf",
        r"C:\Windows\Prefetch\tcpview.exe-*.pf",
        r"C:\Windows\Prefetch\regshot.exe-*.pf",
        r"C:\Windows\Prefetch\processhacker.exe-*.pf",
        r"C:\Windows\Prefetch\mcshield.exe-*.pf",
        r"C:\Windows\Prefetch\sandboxie.exe-*.pf",
        r"C:\Users\<User>\AppData\Local\Temp\Fiddler",
        r"C:\Users\<User>\AppData\Local\Temp\Wireshark",
        r"C:\Users\<User>\AppData\Local\Immunity Debugger",
        r"C:\Users\<User>\AppData\Roaming\Microsoft\Windbg",
        r"C:\Users\<User>\Documents\OllyDbg",
        r"C:\Users\<User>\Documents\x64dbg",
        r"C:\Users\<User>\Documents\x32dbg",
        r"C:\Users\<User>\Documents\IDA Pro",
        r"C:\Users\<User>\Documents\RegShot",
        r"C:\ProgramData\Sandboxie",
        r"C:\ProgramData\Malwarebytes\MBAMService",
        r"C:\ProgramData\Microsoft\Windows\WER\ReportArchive"

    ];

            paths
            .iter()
            .filter(|p| Path::new(p).exists())
            .map(|p| p.to_string())
            .collect()

    }
   // let found: Vec<String> = file_detection();
    
    /*for file in found {
        println!("Detected artifact: {}", file);
    }
    */

pub struct FilesChecker;

impl Scorable<String> for FilesChecker {  
    fn calculate_score(&self) -> f64 {
    let found: Vec<String> = file_detection();
    let len = found.len();
    if len >= 1 && len <= 2 {
        0.2
    } else if len >= 4 {
        0.8
    } else {
        0.0
    }
}
    fn weight(&self) -> f64 {
        0.8
    }
    
    fn create_comment(&self) -> String {
        let found: Vec<String> = file_detection();
        if !found.is_empty() {
            let mut output = "Found suspicious files:".red().to_string();
            for name in found {
                output.push_str(&format!("\n {}", name));
            }
            output
        } else {
            "No suspicious Files found".green().to_string()
        }
    }
    
    fn build_struct(&self) -> CheckResult<String> { 
        let found = file_detection();
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

