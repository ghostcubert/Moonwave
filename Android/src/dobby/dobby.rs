// yes i ported dobby to rust how ud
use core::ffi::{c_char, c_int, c_void};

pub type AddrT = usize;
pub type DobbyDummyFuncT = *mut c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub union FPReg {
    pub q: i128,
    pub d: [f64; 2],
    pub f: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DobbyRegisterContext {
    pub dummy_0: u64,
    pub sp: u64,
    pub dummy_1: u64,
    pub general: DobbyGeneralRegs,
    pub fp: u64,
    pub lr: u64,
    pub floating: DobbyFPRegs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union DobbyGeneralRegs {
    pub x: [u64; 29],
    pub regs: DobbyGeneralRegsNamed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DobbyGeneralRegsNamed {
    pub x0: u64, pub x1: u64, pub x2: u64, pub x3: u64,
    pub x4: u64, pub x5: u64, pub x6: u64, pub x7: u64,
    pub x8: u64, pub x9: u64, pub x10: u64, pub x11: u64,
    pub x12: u64, pub x13: u64, pub x14: u64, pub x15: u64,
    pub x16: u64, pub x17: u64, pub x18: u64, pub x19: u64,
    pub x20: u64, pub x21: u64, pub x22: u64, pub x23: u64,
    pub x24: u64, pub x25: u64, pub x26: u64, pub x27: u64,
    pub x28: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union DobbyFPRegs {
    pub q: [FPReg; 32],
}

pub type DobbyInstrumentCallback = extern "C" fn(address: *mut c_void, ctx: *mut DobbyRegisterContext);
unsafe extern "C" {
    pub fn DobbyHook(
        address: *mut c_void,
        replace_func: DobbyDummyFuncT,
        origin_func: *mut DobbyDummyFuncT,
    ) -> c_int;

    // i ported ts for fun i only need DobbyHook lol
    pub fn DobbyCodePatch(
        address: *mut c_void,
        buffer: *const u8,
        buffer_size: u32,
    ) -> c_int;

    pub fn DobbyInstrument(
        address: *mut c_void,
        pre_handler: DobbyInstrumentCallback,
    ) -> c_int;

    pub fn DobbyDestroy(address: *mut c_void) -> c_int;

    pub fn DobbyGetVersion() -> *const c_char;

    pub fn DobbySymbolResolver(
        image_name: *const c_char,
        symbol_name: *const c_char,
    ) -> *mut c_void;

    pub fn DobbyImportTableReplace(
        image_name: *mut c_char,
        symbol_name: *mut c_char,
        fake_func: DobbyDummyFuncT,
        orig_func: *mut DobbyDummyFuncT,
    ) -> c_int;

    pub fn dobby_enable_near_branch_trampoline();
    pub fn dobby_disable_near_branch_trampoline();
}
