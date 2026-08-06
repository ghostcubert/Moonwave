# Moonwave - iOS

**Moonwave - iOS** is the **iOS Version** of the **Android** counterpart, this branch focuses on creating a **NSURLProtocol** and injecting it to **Fortnite** to redirect it's traffic and swizzling **WKWebview & CFNetwork** internal functions to redirect Partyhub (can be toggled optionally).

## Requirememnts

- **Rust**: to build this.
- **MacOS with XCode**: to have proper tools to build this.
- **A way to inject dylib into the IPA**: i use sideloadly

## How to build

After you setup your **MacOS device** and **Rust** to build libraries run ``cargo build --release --target aarch64-apple-ios`` and you'll find it in ``target/aarch64-apple-ios/release/libmoonwave.dylib``, now you can inject it to an IPA.
