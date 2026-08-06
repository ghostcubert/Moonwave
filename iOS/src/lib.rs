#![allow(unexpected_cfgs)]
use ctor::ctor;
mod protocol;
mod opts;
mod wkwebview;
mod cfnetwork;

// CLEAN CODE WOW
#[ctor]
fn main() {
    unsafe {
        protocol::init_moonwave_url_protocol();
    }

    if opts::USE_PARTYHUB {
        unsafe {
            wkwebview::init_moonwave_webview_delegate();
            cfnetwork::init_moonwave_cfnetwork_hook();
        }
    }
}
