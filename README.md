# Moonwave - Android

**Moonwave** is a **Fortnite** mobile redirect made to redirect **Epic Games** requests to a set **URL** by hooking **curl_easy_setopt** and the first ever mobile redirect made in **Rust** (let alone a Rust redirect but whatever).

## Requirements

- **Rust**: to use this.
- **NDK**: to build.
- **libc++_shared.so** (for some builds): for this to work.

## How to build

To build the redirect just run ```cargo build --release --target aarch64-linux-android``` in a **Terminal** and you should find the lib under ``target/aarch64-linux-android/release/libmoonwave.so``

## How to inject into the APK

Put **libmoonwave.so** and **libc++_shared.so** inside ``libs/arm64-v8a/`` in the apk and go to ``smali/com/epicgames/ue4 or unreal/GameActivity.smali`` and do this

```diff
.method public onCreate(Landroid/os/Bundle;)V
    .locals 11
    # if it doesnt exist on your apk you need this if it does don't do this and set v0 for moonwave
+   const-string v0, "c++_shared"
+   invoke-static {v0}, Ljava/lang/System;->loadLibrary(Ljava/lang/String;)V

+   const-string v1, "moonwave"
+   invoke-static {v1}, Ljava/lang/System;->loadLibrary(Ljava/lang/String;)V
```

Optional (disables minimal system requirements)

```diff
.method private processSystemInfo(Ljava/lang/String;Ljava/lang/String;)Z
    .locals 29
    
+   const/4 v0, 0x1
+   return v0

    :try_start_0 
```

## Partyhub smali part

**You must compile with** ```--features partyhub``` **so it can be included in the native lib!!!!!**

```smali
.class public Lcom/razer/moonwave/Partyhub;
.super Ljava/lang/Object;

.method public constructor <init>()V
    .locals 0
    invoke-direct {p0}, Ljava/lang/Object;-><init>()V
    return-void
.end method

.method public static native getLoginUrl()Ljava/lang/String;
.end method

.method public static native getRegisterUrl()Ljava/lang/String;
.end method

.method public static native getGraphqlUrl()Ljava/lang/String;
.end method
```
