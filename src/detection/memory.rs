use sysinfo::{
    Components, Disks, Networks, System,
};




pub fn main(){
println!("=> disks:");
let disks = Disks::new_with_refreshed_list();
for disk in &disks {
    println!("{disk:?}");
}
    }