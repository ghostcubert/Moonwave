use core::ffi::{c_char, c_int, c_void};
use std::ffi::CString;
use crate::opts;
use crate::dobby::{DobbyHook, DobbySymbolResolver};
use log::{error, info};
use obfstr::obfstr;
use crate::curl_hook::{eos_setopt_hook, fn_setopt_hook};
use std::fs::File;
use std::io::{BufRead, BufReader};

unsafe extern "C" {
    fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> c_int;
}

pub unsafe fn init_ue_hook() {
    info!("Initializing Unreal hook...");

    let curl_easy_setopt: *mut c_void;
    if opts::USE_CURL_SYMBOL {
        curl_easy_setopt = unsafe { DobbySymbolResolver(CString::new(obfstr!("libUE4.so")).unwrap().as_ptr(), CString::new(obfstr!("curl_easy_setopt")).unwrap().as_ptr()) };
        if curl_easy_setopt.is_null() {
            error!("Failed to find curl_easy_setopt symbol");
            return;
        }

        info!("Found curl_easy_setopt at {:p}", curl_easy_setopt);
    } else {
        let lib_name = CString::new(obfstr!("libUnreal.so")).unwrap(); // change this to libUnreal.so if ur doing s19+ else change to libUE4.so
        let handle = unsafe { dlopen(lib_name.as_ptr(), 2) };
        if handle.is_null() {
            error!("Failed to load Unreal lib");
            return;
        }

        let base = get_module_base(lib_name.to_str().unwrap());
        if base == 0 { // IMPOSSIBLE!!!!!!!!!!!
            error!("Failed to find libUnreal.so base");
            unsafe { dlclose(handle) };
            return;
        }

        curl_easy_setopt = (base + 0x12DB8640) as *mut c_void; // set this address based on ur version
        info!("Found curl_easy_setopt at {:p}", curl_easy_setopt);
        unsafe { dlclose(handle) };
    }

    let result = unsafe { DobbyHook(
        curl_easy_setopt,
        fn_setopt_hook as *mut c_void,
        &raw mut crate::curl_hook::OG_SETOPT as *mut _ as *mut *mut c_void,
    ) };

    if result == 0 {
        info!("Successfully hooked curl_easy_setopt");
    } else {
        error!("Failed to hook curl_easy_setopt");
    }
}

pub unsafe fn init_eos_hook() {
    info!("Initializing EOS hook...");

    let lib_name = CString::new(obfstr!("libEOSSDK.so")).unwrap();
    let handle = unsafe { dlopen(lib_name.as_ptr(), 2) };
    if handle.is_null() {
        error!("Failed to load EOS lib");
        return;
    }

    let base = get_module_base(lib_name.to_str().unwrap());
    if base == 0 { // AGAIN IMPOSSIBLE!!!!!!!!!!!
        error!("Failed to find libEOSSDK.so base");
        unsafe { dlclose(handle) };
        return;
    }

    let curl_easy_setopt = (base + 0x1988998) as *mut c_void; // set this address based on ur version
    info!("Found curl_easy_setopt at {:p}", curl_easy_setopt);

    let result = unsafe { DobbyHook(
        curl_easy_setopt,
        eos_setopt_hook as *mut c_void,
        &raw mut crate::curl_hook::EOS_OG_SETOPT as *mut _ as *mut *mut c_void,
    ) };

    if result == 0 {
        info!("Successfully hooked eos's curl_easy_setopt");
    } else {
        error!("Failed to hook eos's curl_easy_setopt");
    }

    unsafe { dlclose(handle) };
}

// ud modulebase finder
pub fn get_module_base(lib_name: &str) -> usize {
    let file = match File::open("/proc/self/maps") {
        Ok(f) => f,
        Err(_) => return 0,
    };

    let reader = BufReader::new(file);
    for line in reader.lines().flatten() {
        if line.contains(lib_name) {
            if let Some(range) = line.split_whitespace().next() {
                if let Some(start) = range.split('-').next() {
                    if let Ok(base) = usize::from_str_radix(start, 16) {
                        return base;
                    }
                }
            }
        }
    }

    0
}