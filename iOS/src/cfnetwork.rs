use objc::runtime::{Class, Object, Sel, Imp, Method};
use objc::{msg_send, sel, sel_impl};
use std::os::raw::c_void;
use std::ffi::CStr;
use crate::opts::BACKEND_URL;

static mut OG_DATA_TASK: Option<Imp> = None;

// most proper graphql redirect frfr
unsafe extern "C" fn data_task_hook(this: *mut Object, _cmd: Sel, request: *mut Object, completion_handler: *mut c_void) -> *mut Object {
    let mut modified_request: *mut Object = request;
    if !request.is_null() {
        let request_url: *mut Object = msg_send![request, URL];
        if !request_url.is_null() {
            let abs_nsstr: *mut Object = msg_send![request_url, absoluteString];
            if !abs_nsstr.is_null() {
                let cstr: *const i8 = msg_send![abs_nsstr, UTF8String];
                if !cstr.is_null() {
                    if let Ok(url_str) = unsafe { CStr::from_ptr(cstr).to_str() } {
                        if url_str.contains("graphql.epicgames.com") {
                            let nsstr_cls: &Class = Class::get("NSString").unwrap();
                            let nsurl_cls: &Class = Class::get("NSURL").unwrap();
                            let req_cls: &Class = Class::get("NSMutableURLRequest").unwrap();

                            // new request stuff
                            let path: *mut Object = msg_send![request_url, path];
                            let query: *mut Object = msg_send![request_url, query];
                            let mut new_url: String = BACKEND_URL.to_string();
                            if !path.is_null() {
                                let path_cstr: *const i8 = msg_send![path, UTF8String];
                                if let Ok(path_str) = unsafe { CStr::from_ptr(path_cstr).to_str() } {
                                    new_url.push_str(path_str);
                                }
                            }
                            if !query.is_null() {
                                let query_cstr: *const i8 = msg_send![query, UTF8String];
                                if let Ok(q) = unsafe { CStr::from_ptr(query_cstr).to_str() } {
                                    new_url.push('?');
                                    new_url.push_str(q);
                                }
                            }

                            // Make new request
                            let ns_new_str: *mut Object = msg_send![nsstr_cls, stringWithUTF8String: new_url.as_ptr() as *const i8];
                            let new_url_obj: *mut Object = msg_send![nsurl_cls, URLWithString: ns_new_str];
                            let new_request: *mut Object = msg_send![req_cls, requestWithURL: new_url_obj];

                            // Skid body
                            let body: *mut Object = msg_send![request, HTTPBody];
                            if !body.is_null() {
                                let _: () = msg_send![new_request, setHTTPBody: body];
                            }

                            // Skid headers
                            let headers: *mut Object = msg_send![request, allHTTPHeaderFields];
                            if !headers.is_null() {
                                let _: () = msg_send![new_request, setAllHTTPHeaderFields: headers];
                            }

                            // Skid HTTP method
                            let method: *mut Object = msg_send![request, HTTPMethod];
                            if !method.is_null() {
                                let _: () = msg_send![new_request, setHTTPMethod: method];
                            }

                            modified_request = new_request;
                        }
                    }
                }
            }
        }
    }

    if let Some(imp) = unsafe { OG_DATA_TASK } {
        let og: extern "C" fn(*mut Object, Sel, *mut Object, *mut c_void) -> *mut Object = unsafe { std::mem::transmute(imp) };
        return og(this, _cmd, modified_request, completion_handler);
    }

    // making the compiler shut up
    std::ptr::null_mut()
}

// proper hooking frfr
pub unsafe fn init_moonwave_cfnetwork_hook() {
    let cls: &Class = Class::get("NSURLSession").unwrap();
    let sel = sel!(dataTaskWithRequest:completionHandler:);
    let method: *mut Method = unsafe { objc::runtime::class_getInstanceMethod(cls, sel) } as *mut Method;

    if !method.is_null() {
        unsafe { OG_DATA_TASK = Some(objc::runtime::method_getImplementation(method)) };
        let new_imp: Imp = unsafe { std::mem::transmute(data_task_hook as unsafe extern "C" fn(*mut Object, Sel, *mut Object, *mut c_void) -> *mut Object) };
        unsafe { objc::runtime::method_setImplementation(method, new_imp) };
    }
}
