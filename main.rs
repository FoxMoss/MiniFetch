mod sysinfo;
use sysinfo::getSysInfo;

fn main()
{
    println!("MiniFetch - System info simply.");
    println!("Hostname:\t {}", getSysInfo()[1]);
    println!("Linux:\t\t {}", getSysInfo()[2]);
}
