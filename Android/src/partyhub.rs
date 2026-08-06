#![cfg(feature = "partyhub")]
use jni::objects::JClass;
use jni::sys::jstring;
use jni::JNIEnv;
use crate::url::Url;
use crate::opts;

// jni rust functions are annoying istg the compiler wont stfu about my shitty code
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_razer_moonwave_Partyhub_getLoginUrl(env: JNIEnv, _class: JClass) -> jstring {
    env.new_string(opts::LOGIN_URL.as_str())
        .expect("Failed to create Login url wow")
        .into_raw()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_razer_moonwave_Partyhub_getRegisterUrl(env: JNIEnv, _class: JClass) -> jstring {
    env.new_string(opts::REGISTER_URL.as_str())
        .expect("Failed to create register url wow")
        .into_raw()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_razer_moonwave_Partyhub_getGraphqlUrl(env: JNIEnv, _class: JClass) -> jstring {
    let graphql_url = Url::create_url(opts::BACKEND_URL.as_str(), "/partyhub/graphql");
    env.new_string(graphql_url)
        .expect("Failed to create graphql url wow")
        .into_raw()
}