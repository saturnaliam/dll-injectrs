use windows::{core::PCSTR, Win32::{Foundation::{HANDLE, HMODULE}, System::{Diagnostics::Debug::*, LibraryLoader::*, Memory::*, Threading::*}}};
use std::{env, ffi::{c_void, CString}};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("usage: dll-inject <pid> <dll path>");
    }

    let pid: u32 = args[1].parse().expect("error getting pid!");
    let dll_path: &str = &args[2];

    unsafe {
        let process: HANDLE = OpenProcess(PROCESS_ALL_ACCESS, true, pid).expect("error while getting the process handle!");
        let base_address: *mut c_void = VirtualAllocEx(process, None, dll_path.len() + 1, MEM_COMMIT, PAGE_READWRITE);

        let dll_path_c: *const c_void = match CString::new(dll_path) {
            Ok(c) => c.as_ptr() as *const c_void,
            Err(_) => panic!("error converting dll path to c string!"),
        };

        match WriteProcessMemory(process, base_address, dll_path_c, dll_path.len() + 1, None) {
            Ok(_) => (),
            Err(_) => panic!("error while writing process memory!"),
        }

        let kernel32: PCSTR = PCSTR::from_raw("kernel32.dll".as_ptr());
        let kernel32_base: HMODULE = match GetModuleHandleA(kernel32) {
            Ok(c) => c,
            Err(_) => panic!("error while getting kernel32.dll"),
        };
        let thread: HANDLE = match CreateRemoteThread(process, None, 0, GetProcAddress(kernel32_base, PCSTR::from_raw("LoadLibraryA".as_ptr())), base_address, 0, None) {
            Ok(c) => c,
            Err(_) => panic!("error while creatig remote thread!"),
        };
   }
}
