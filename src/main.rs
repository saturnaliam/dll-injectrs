use windows::Win32::{Foundation::HANDLE, System::Threading::*};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let pid: u32 = args[1].parse().expect(":(");

    unsafe {
        let process: HANDLE = OpenProcess(PROCESS_ALL_ACCESS, true, pid).expect("wawa");
        println!("{:?}", process);
    }
}
