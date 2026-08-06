#![allow(unused_imports)]
use std::cell::RefCell;
use std::ffi::{CStr, CString, c_char};
use std::os::raw::{c_int, c_void, c_long};
use log::info;
use crate::opts;
use crate::url::Url;

pub static mut OG_SETOPT: Option<extern "C" fn(*mut c_void, c_int, *mut c_void) -> c_int> = None;
pub static mut EOS_OG_SETOPT: Option<extern "C" fn(*mut c_void, c_int, *mut c_void) -> c_int> = None;
thread_local! {
    static REDIRECT_BUF: RefCell<Option<CString>> = RefCell::new(None);
}

pub extern "C" fn fn_setopt_hook(handle: *mut c_void, option: c_int, arg: *mut c_void) -> c_int {
    unsafe {
        return internal_setopt(handle, option, arg, OG_SETOPT.unwrap());
    }
}

pub extern "C" fn eos_setopt_hook(handle: *mut c_void, option: c_int, arg: *mut c_void) -> c_int {
    unsafe {
        return internal_setopt(handle, option, arg, EOS_OG_SETOPT.unwrap());
    }
}

pub extern "C" fn internal_setopt(handle: *mut c_void, option: c_int, arg: *mut c_void, og_setopt: extern "C" fn(*mut c_void, c_int, *mut c_void) -> c_int) -> c_int {
    unsafe {
        if option == 10002 && !arg.is_null() {
            let url_cstr = CStr::from_ptr(arg as *const c_char);
            if let Ok(url_str) = url_cstr.to_str() {
                let url = Url::parse_url(url_str);
                // info!("Original Host: {}\nOriginal Path and Query: {}", url.host, url.path_and_query);

                if Url::should_redirect(&url.host) {
                    let redirected = Url::create_url(opts::BACKEND_URL.as_str(), &url.path_and_query);
                    // info!("Redirected URL from {} to {}", url_str, redirected);

                    return REDIRECT_BUF.with(|buf| {
                        let mut buf = buf.borrow_mut();
                        let cstr = CString::new(redirected).unwrap();
                        let ptr = cstr.as_ptr();
                        *buf = Some(cstr);

                        og_setopt(handle, option, ptr as *mut c_void)
                    });
                }
            }
        } else if option == 64 && opts::BYPASS_SSL {
            return og_setopt(handle, option, (0 as c_long) as usize as *mut c_void);
        }

        og_setopt(handle, option, arg)
    }
}
