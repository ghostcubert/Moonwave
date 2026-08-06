use std::sync::LazyLock;
use std::os::raw::c_void;
use std::ffi::{CString, CStr};
use std::collections::HashMap;
use std::sync::Mutex;
use objc::runtime::{Class, Object, Sel, Method, Imp};
use objc::{msg_send, sel, sel_impl};
use block::Block;
use crate::opts::{LOGIN_URL, REGISTER_URL};

// these are stack overflow fixes idfk how but it worked im dead tired
static OG_IMPS: LazyLock<Mutex<HashMap<usize, Imp>>> = LazyLock::new(|| { Mutex::new(HashMap::new()) });
static mut OG_SET_NAV: Option<extern "C" fn(*mut Object, Sel, *mut Object)> = None;

// some stupid fix i need to not piss off apple
#[unsafe(link_section = "__TEXT,__text")]
unsafe extern "C" fn decide_policy_hook(this: *mut Object, _cmd: Sel, webview: *mut Object, navigation_action: *mut Object, decision_handler: *mut std::ffi::c_void) {
    let delegate_cls: *const Class = msg_send![this, class];
    let delegate_cls_key: usize = delegate_cls as usize;
    let og_imp = {
        let map = OG_IMPS.lock().unwrap();
        map.get(&delegate_cls_key).copied()
    };

    let call_og = || {
        if let Some(imp) = og_imp {
            let f: extern "C" fn(*mut Object, Sel, *mut Object, *mut Object, *mut c_void) =
                unsafe { std::mem::transmute(imp) };
            f(this, _cmd, webview, navigation_action, decision_handler);
        }
    };

    let target_frame: *mut Object = unsafe { msg_send![navigation_action, targetFrame] };
    let is_main_frame: bool = if target_frame.is_null() {
        true
    } else {
        unsafe { msg_send![target_frame, isMainFrame] }
    };

    if !is_main_frame {
        if !decision_handler.is_null() {
            let block: &Block<(i64,), ()> = unsafe { &*(decision_handler as *mut Block<(i64,), ()>) };
            unsafe { block.call((1,)) };
        }
        return;
    }

    if navigation_action.is_null() {
        call_og();
        return;
    }

    let request: *mut Object = msg_send![navigation_action, request];
    if request.is_null() {
        call_og();
        return;
    }

    let url_obj: *mut Object = msg_send![request, URL];
    if url_obj.is_null() {
        call_og();
        return;
    }

    // this is necessairy unless u wanna break scheme handling
    let scheme: *mut Object = msg_send![url_obj, scheme];
    if !scheme.is_null() {
        let scheme_cstr: *const i8 = msg_send![scheme, UTF8String];
        if !scheme_cstr.is_null() {
            let scheme_str: std::borrow::Cow<'_, str> = unsafe { CStr::from_ptr(scheme_cstr).to_string_lossy() };
            let host: *mut Object = msg_send![url_obj, host];
            let host_str: Option<String> = if !host.is_null() {
                let c: *const i8 = msg_send![host, UTF8String];
                if !c.is_null() {
                    Some(unsafe { CStr::from_ptr(c).to_string_lossy().to_string() })
                } else {
                    None
                }
            } else {
                None
            };

            // handling login (i cba this works fine too)
            if scheme_str == "com.epicgames.fortnite" && host_str.as_deref() == Some("authorize") {
                call_og();
                return;
            }

            // other stuff (i was implementing a custom way but failed miserably)
            if scheme_str != "http" && scheme_str != "https" {
                let block: &Block<(i64,), ()> = unsafe { &*(decision_handler as *mut Block<(i64,), ()>) };
                unsafe { block.call((0,)) };

                let ui_app: *mut Object = msg_send![Class::get("UIApplication").unwrap(), sharedApplication];
                let can: bool = msg_send![ui_app, canOpenURL: url_obj];
                if can {
                    let _: () = msg_send![ui_app, openURL: url_obj options: std::ptr::null::<Object>() completionHandler: std::ptr::null::<Object>()];
                }
                return;
            }
        }
    }

    let abs: *mut Object = msg_send![url_obj, absoluteString];
    if abs.is_null() {
        call_og();
        return;
    }

    let cstr: *const i8 = msg_send![abs, UTF8String];
    if cstr.is_null() {
        call_og();
        return;
    }

    let url_str: &str = match unsafe { CStr::from_ptr(cstr).to_str() } {
        Ok(s) => s,
        Err(_) => {
            call_og();
            return;
        }
    };

    // proper url selection
    let redirect_to: Option<&str> = if url_str.contains("/id/login") {
        Some(LOGIN_URL)
    } else if url_str.contains("/id/register") {
        Some(REGISTER_URL)
    } else {
        None
    };

    // fun stuff
    if let Some(target) = redirect_to {
        if !decision_handler.is_null() {
            let block: &Block<(i64,), ()> = unsafe { &*(decision_handler as *mut Block<(i64,), ()>) };
            unsafe { block.call((1,)) };
        }

        let target: String = target.to_owned();
        let webview: *mut Object = webview;
        unsafe {
            let ns_str_cls: &Class = Class::get("NSString").unwrap();
            let ns_url_cls: &Class = Class::get("NSURL").unwrap();
            let req_cls: &Class = Class::get("NSURLRequest").unwrap();

            let cstr: CString = CString::new(target).unwrap();
            let ns_str: *mut Object = msg_send![ns_str_cls, stringWithUTF8String: cstr.as_ptr()];
            let url: *mut Object = msg_send![ns_url_cls, URLWithString: ns_str];
            let req: *mut Object = msg_send![req_cls, requestWithURL: url];

            let _: () = msg_send![webview, loadRequest: req];
        }

        return;
    }

    // this is needed dont touch
    if !decision_handler.is_null() {
        let block: &Block<(i64,), ()> = unsafe { &*(decision_handler as *mut Block<(i64,), ()>) };
        unsafe { block.call((1,)) };
    }
}

// some weird function i need to hook cuz yeah apple is a faggot
unsafe extern "C" fn set_navigation_delegate_hook(this: *mut Object, _cmd: Sel, delegate: *mut Object) {
    if !delegate.is_null() {
        let delegate_cls: *const Class = msg_send![delegate, class];
        let delegate_cls_key = delegate_cls as usize;
        let mut map = OG_IMPS.lock().unwrap();
        if !map.contains_key(&delegate_cls_key) {
            let sel: Sel = sel!(webView:decidePolicyForNavigationAction:decisionHandler:);
            let method: *mut Method = unsafe { objc::runtime::class_getInstanceMethod(delegate_cls, sel) } as *mut Method;
            if !method.is_null() {
                let og_imp: Imp = unsafe { objc::runtime::method_getImplementation(method) };
                map.insert(delegate_cls_key, og_imp);

                let new_imp: Imp = unsafe { std::mem::transmute(
                    decide_policy_hook as unsafe extern "C" fn(*mut Object, Sel, *mut Object, *mut Object, *mut c_void)
                ) };
                unsafe { objc::runtime::method_setImplementation(method, new_imp) };
            }
        }
    }

    if let Some(og) = unsafe { OG_SET_NAV } {
        og(this, _cmd, delegate);
    }
}

// proper hooks frfr
pub unsafe fn init_moonwave_webview_delegate() {
    let cls: &Class = Class::get("WKWebView").unwrap();
    let sel: Sel = sel!(setNavigationDelegate:);
    let method: *mut Method = unsafe { objc::runtime::class_getInstanceMethod(cls, sel) } as *mut Method;

    unsafe { OG_SET_NAV = Some(std::mem::transmute(objc::runtime::method_getImplementation(method))) };

    let new_imp: Imp = unsafe { std::mem::transmute(
        set_navigation_delegate_hook as unsafe extern "C" fn(*mut Object, Sel, *mut Object)
    ) };
    unsafe { objc::runtime::method_setImplementation(method, new_imp) };
}
