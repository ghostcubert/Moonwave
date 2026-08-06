#![allow(unused)]
use obfstr::obfstr;
use std::sync::LazyLock;

pub static BACKEND_URL: LazyLock<String> = LazyLock::new(|| {
    obfstr!("http://192.168.1.69:8080").to_string()
});

pub const USE_CURL_SYMBOL: bool = true; // disable this if 18.40+
pub const USE_EOS: bool = false; // enable this if 22+
pub const BYPASS_SSL: bool = false; // idk why u will use this its js for debugging with fiddler

// Partyhub shit
pub static LOGIN_URL: LazyLock<String> = LazyLock::new(|| {
    obfstr!("http://192.168.1.69:5000/login").to_string()
});

pub static REGISTER_URL: LazyLock<String> = LazyLock::new(|| {
    obfstr!("http://192.168.1.69:5000/register").to_string()
});