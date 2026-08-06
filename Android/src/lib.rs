#[path = "dobby/dobby.rs"]
pub mod dobby;
mod hooking;
mod opts;
mod curl_hook;
mod url;

#[cfg(feature = "partyhub")]
pub mod partyhub;

use log::info;
use ctor::ctor;

#[ctor]
fn main() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_min_level(log::Level::Info)
            .with_tag("moonwave"),
    );

    info!("Moonwave initialized!");
    std::thread::spawn(|| unsafe {
        hooking::init_ue_hook();
    });

    if opts::USE_EOS {
        std::thread::spawn(|| unsafe {
            hooking::init_eos_hook();
        });
    }
}
