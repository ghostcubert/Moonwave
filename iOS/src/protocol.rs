use objc::runtime::{Class, Object, Sel};
use objc::{msg_send, sel, sel_impl};
use crate::opts::{BACKEND_URL};
use std::ffi::CString;
use std::ptr;

// ts tuff icl frfr peak
pub unsafe fn init_moonwave_url_protocol() {
    let ns_url_protocol: &Class = Class::get("NSURLProtocol").unwrap();
    let subclass_name = CString::new("MoonwaveProtocol").unwrap();

    // create class
    let subclass: *mut Class = match Class::get("MoonwaveProtocol") {
        Some(c) => c as *const Class as *mut Class,
        None => {
            unsafe { objc::runtime::objc_allocateClassPair(
                ns_url_protocol as *const _,
                subclass_name.as_ptr(),
                0,
            ) }
        }
    };

    if subclass.is_null() {
        return;
    }

    // ud hooks the old one sucked
    unsafe {
        let meta = objc::runtime::object_getClass(subclass as *mut _) as *mut Class;
    
        // +canInitWithRequest:
        objc::runtime::class_addMethod(
            meta,
            sel!(canInitWithRequest:),
            std::mem::transmute(
                can_init_with_request
                    as unsafe extern "C" fn(*mut Class, Sel, *mut Object) -> bool
            ),
            b"B#:@\0".as_ptr() as *const i8,
        );
    
        // +canInitWithTask:
        objc::runtime::class_addMethod(
            meta,
            sel!(canInitWithTask:),
            std::mem::transmute(
                can_init_with_task
                    as unsafe extern "C" fn(*mut Class, Sel, *mut Object) -> bool
            ),
            b"B#:@\0".as_ptr() as *const i8,
        );
    
        // +canonicalRequestForRequest:
        objc::runtime::class_addMethod(
            meta,
            sel!(canonicalRequestForRequest:),
            std::mem::transmute(
                canonical_request_for_request
                    as unsafe extern "C" fn(*mut Class, Sel, *mut Object) -> *mut Object
            ),
            b"@#:@\0".as_ptr() as *const i8,
        );
    
        // -startLoading
        objc::runtime::class_addMethod(
            subclass,
            sel!(startLoading),
            std::mem::transmute(
                start_loading as unsafe extern "C" fn(*mut Object, Sel)
            ),
            b"v@:\0".as_ptr() as *const i8,
        );
    
        // -stopLoading
        objc::runtime::class_addMethod(
            subclass,
            sel!(stopLoading),
            std::mem::transmute(
                stop_loading as unsafe extern "C" fn(*mut Object, Sel)
            ),
            b"v@:\0".as_ptr() as *const i8,
        );
    }    

    // registering yay
    unsafe { objc::runtime::objc_registerClassPair(subclass) };
    let _: () = msg_send![ns_url_protocol, registerClass: subclass];
}

// proper hooks
unsafe extern "C" fn can_init_with_request(_cls: *mut Class, _cmd: Sel, request: *mut Object) -> bool {
    if request.is_null() { return false; }

    let url: *mut Object = msg_send![request, URL];
    if url.is_null() { return false; }

    let abs_url: *mut Object = msg_send![url, absoluteString];
    if abs_url.is_null() { return false; }

    let cstr: *const i8 = msg_send![abs_url, UTF8String];
    if cstr.is_null() { return false; }

    let url_str = unsafe { std::ffi::CStr::from_ptr(cstr).to_string_lossy() };
    const EPIC_DOMAINS: [&str; 6] = [
        "game-social.epicgames.com",
        "ol.epicgames.com",
        "ol.epicgames.net",
        "on.epicgames.com",
        "ak.epicgames.com",
        "epicgames.dev"
    ];
    
    for &domain in EPIC_DOMAINS.iter() {
        if url_str.contains(domain) {
            return true;
        }
    }

    false
}

unsafe extern "C" fn can_init_with_task(cls: *mut Class, _cmd: Sel, task: *mut Object) -> bool {
    if task.is_null() { return false; }
    let req: *mut Object = msg_send![task, currentRequest];
    if req.is_null() {
        let req: *mut Object = msg_send![task, originalRequest];
        if req.is_null() { return false; }
        return unsafe { can_init_with_request(cls, sel!(canInitWithRequest:), req) };
    }
    unsafe { can_init_with_request(cls, sel!(canInitWithRequest:), req) }
}

unsafe extern "C" fn canonical_request_for_request(_cls: *mut Class, _cmd: Sel, request: *mut Object) -> *mut Object {
    request
}

pub unsafe extern "C" fn start_loading(this: *mut Object, _cmd: Sel) {
    let og_req: *mut Object = msg_send![this, request];
    if og_req.is_null() { return; }

    let request: *mut Object = msg_send![og_req, mutableCopy];
    if request.is_null() { return; }

    let url: *mut Object = msg_send![request, URL];
    if url.is_null() { return; }

    let backend_cstr = CString::new(BACKEND_URL).unwrap();
    let ns_backend: *mut Object = msg_send![Class::get("NSString").unwrap(), stringWithUTF8String: backend_cstr.as_ptr()];

    let components: *mut Object = msg_send![Class::get("NSURLComponents").unwrap(), componentsWithString: ns_backend];
    let path: *mut Object = msg_send![url, path];
    let query: *mut Object = msg_send![url, query];
    let _: () = msg_send![components, setPath: path];
    let _: () = msg_send![components, setQuery: query];

    let new_url: *mut Object = msg_send![components, URL];
    let _: () = msg_send![request, setURL: new_url];

    let client: *mut Object = msg_send![this, client];
    if !client.is_null() {
        let _: () = msg_send![client,
            URLProtocol:this
            wasRedirectedToRequest:request
            redirectResponse: ptr::null_mut::<Object>()
        ];
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn stop_loading(_this: *mut Object, _cmd: Sel) {
    /*let task: *mut Object = msg_send![this, task];
    if !task.is_null() {
        let _: () = msg_send![task, cancel];
        let _: () = msg_send![this, setTask: ptr::null_mut::<Object>()];
    }*/
}
