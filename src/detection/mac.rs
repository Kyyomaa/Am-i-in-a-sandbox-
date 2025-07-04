use mac_address::get_mac_address;
use crate::detection::shared::Scorable;
use colored::Colorize;


fn get_mac_info() -> String {
    match get_mac_address() {
        Ok(Some(mac)) => mac.to_string(),
        Ok(None) => "NO_MAC_FOUND".to_string(),
        Err(e) => format!("ERROR: {}", e), // Handle the error case
    }
}


fn is_virtual_mac(mac: &str) -> bool {
    let vm_ouis = [
        "00:05:69", // VMware 
        "00:0c:29", // VMware
        "00:1c:14", // VMware 
        "00:50:56", // VMware
        "08:00:27", // VirtualBox
        "00:15:5d", // Hyper-V
        "00:1c:42", // Parallels
        "52:54:00", // QEMU/KVM
        "00:16:3e", // Xen/Citrix
        "54:52:00", // Red Hat Virt
        "02:00:00", // Amazon EC2 
        "02:42:ac", // Docker
    ];

    vm_ouis.iter().any(|oui| mac.to_lowercase().starts_with(oui))
}


fn mac_check() -> Result<bool, Box<dyn std::error::Error>> {
    let result = match get_mac_address()? {
        Some(mac) => {
            let mac_str: String = mac.to_string();
            Ok(is_virtual_mac(&mac_str))
        }
        None => Ok(false), // Or true? But usually false.
    };

    result
}
pub struct MacChecker;
impl Scorable<String> for MacChecker{ 

fn calculate_score(&self) -> f64 {
    let result = mac_check();

    match result {
        Ok(true) => 0.9,
        Ok(false) => 0.0,
        Err(_) => 0.0, 
    }
}
    fn weight(&self) -> f64 {
        0.9
    }
    
    fn create_comment(&self) -> String {
        if self.calculate_score() != 0.00 {
            "Found Known Virtual mac address OUI. Suspicious".red().to_string()
        } else {
            "Mac addr OK".green().to_string()
        }
    }
    fn weighted_score(&self) -> f64 {
        self.calculate_score() * self.weight()
    }

    fn build_struct(&self) -> crate::detection::shared::CheckResult<String> {
        let result: String = get_mac_info();
        crate::detection::shared::CheckResult::new(
            result,
            self.weight(),
            self.calculate_score(),
            self.weighted_score(), 
            self.create_comment(),
        )
    }

}

