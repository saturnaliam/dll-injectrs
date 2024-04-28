use windows::Win32::{Foundation::HANDLE, System::Threading::*, System::Memory::*, System::Diagnostics::Debug::*};
use std::{env, ffi::{c_void, CString}};

fn main() {
    let args: Vec<String> = env::args().collect();
    let pid: u32 = args[1].parse().expect("error getting pid!");
    let dll_path: &str = &args[2];

    println!("{}", dll_path);
    unsafe {
        let process: HANDLE = OpenProcess(PROCESS_ALL_ACCESS, true, pid).expect("error while getting the process handle!");
        let base_address: *mut c_void = VirtualAllocEx(process, None, dll_path.len() + 1, MEM_COMMIT, PAGE_READWRITE);

        let dll_path_c: *const c_void = match CString::new(dll_path) {
            Ok(c) => c.as_ptr() as *const c_void,
            Err(_) => panic!("error converting dll path to c string!"),
        };

        match WriteProcessMemory(process, base_address, dll_path_c, dll_path.len() + 1, None) {
            Ok(_) => println!("wrote memory successfully!"),
            Err(_) => panic!("error while writing process memory!"),
        }
    }
}
