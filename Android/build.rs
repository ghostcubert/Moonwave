// js tells the compiler where dobby is so we can hook
fn main() {
    println!("cargo:rustc-link-search=native=src/dobby/libs/arm64-v8a");
    println!("cargo:rustc-link-lib=static=dobby");
}
