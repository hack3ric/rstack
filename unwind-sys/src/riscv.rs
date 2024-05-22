use libc::{c_char, c_int, c_void, size_t, ucontext_t};

use crate::*;

pub const UNW_TDEP_CURSOR_LEN: c_int = 4096;

#[cfg(target_arch = "riscv32")]
pub type unw_word_t = u32;
#[cfg(target_arch = "riscv32")]
pub type unw_sword_t = i32;

#[cfg(target_arch = "riscv64")]
pub type unw_word_t = u64;
#[cfg(target_arch = "riscv64")]
pub type unw_sword_t = i64;

/* integer registers */
pub const UNW_RISCV_X0: c_int = 0;
pub const UNW_RISCV_X1: c_int = 1;
pub const UNW_RISCV_X2: c_int = 2;
pub const UNW_RISCV_X3: c_int = 3;
pub const UNW_RISCV_X4: c_int = 4;
pub const UNW_RISCV_X5: c_int = 5;
pub const UNW_RISCV_X6: c_int = 6;
pub const UNW_RISCV_X7: c_int = 7;
pub const UNW_RISCV_X8: c_int = 8;
pub const UNW_RISCV_X9: c_int = 9;
pub const UNW_RISCV_X10: c_int = 10;
pub const UNW_RISCV_X11: c_int = 11;
pub const UNW_RISCV_X12: c_int = 12;
pub const UNW_RISCV_X13: c_int = 13;
pub const UNW_RISCV_X14: c_int = 14;
pub const UNW_RISCV_X15: c_int = 15;
pub const UNW_RISCV_X16: c_int = 16;
pub const UNW_RISCV_X17: c_int = 17;
pub const UNW_RISCV_X18: c_int = 18;
pub const UNW_RISCV_X19: c_int = 19;
pub const UNW_RISCV_X20: c_int = 20;
pub const UNW_RISCV_X21: c_int = 21;
pub const UNW_RISCV_X22: c_int = 22;
pub const UNW_RISCV_X23: c_int = 23;
pub const UNW_RISCV_X24: c_int = 24;
pub const UNW_RISCV_X25: c_int = 25;
pub const UNW_RISCV_X26: c_int = 26;
pub const UNW_RISCV_X27: c_int = 27;
pub const UNW_RISCV_X28: c_int = 28;
pub const UNW_RISCV_X29: c_int = 29;
pub const UNW_RISCV_X30: c_int = 30;
pub const UNW_RISCV_X31: c_int = 31;

/* floating point registers */
pub const UNW_RISCV_F0: c_int = 32;
pub const UNW_RISCV_F1: c_int = 33;
pub const UNW_RISCV_F2: c_int = 34;
pub const UNW_RISCV_F3: c_int = 35;
pub const UNW_RISCV_F4: c_int = 36;
pub const UNW_RISCV_F5: c_int = 37;
pub const UNW_RISCV_F6: c_int = 38;
pub const UNW_RISCV_F7: c_int = 39;
pub const UNW_RISCV_F8: c_int = 40;
pub const UNW_RISCV_F9: c_int = 41;
pub const UNW_RISCV_F10: c_int = 42;
pub const UNW_RISCV_F11: c_int = 43;
pub const UNW_RISCV_F12: c_int = 44;
pub const UNW_RISCV_F13: c_int = 45;
pub const UNW_RISCV_F14: c_int = 46;
pub const UNW_RISCV_F15: c_int = 47;
pub const UNW_RISCV_F16: c_int = 48;
pub const UNW_RISCV_F17: c_int = 49;
pub const UNW_RISCV_F18: c_int = 50;
pub const UNW_RISCV_F19: c_int = 51;
pub const UNW_RISCV_F20: c_int = 52;
pub const UNW_RISCV_F21: c_int = 53;
pub const UNW_RISCV_F22: c_int = 54;
pub const UNW_RISCV_F23: c_int = 55;
pub const UNW_RISCV_F24: c_int = 56;
pub const UNW_RISCV_F25: c_int = 57;
pub const UNW_RISCV_F26: c_int = 58;
pub const UNW_RISCV_F27: c_int = 59;
pub const UNW_RISCV_F28: c_int = 60;
pub const UNW_RISCV_F29: c_int = 61;
pub const UNW_RISCV_F30: c_int = 62;
pub const UNW_RISCV_F31: c_int = 63;

pub const UNW_RISCV_PC: c_int = 64;

pub const UNW_TDEP_LAST_REG: c_int = UNW_RISCV_PC;

/* The CFA is the value of SP in previous frame */
pub const UNW_RISCV_CFA: c_int = UNW_RISCV_X2;

pub const UNW_TDEP_IP: c_int = UNW_RISCV_PC;
pub const UNW_TDEP_SP: c_int = UNW_RISCV_X2;
pub const UNW_TDEP_EH: c_int = UNW_RISCV_X10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unw_tdep_save_loc_t {
    #[cfg(not(pre12))]
    pub unused: c_char,
}

pub type unw_tdep_context_t = ucontext_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unw_tdep_proc_info_t {
    #[cfg(not(pre12))]
    pub unused: c_char,
}

#[macro_export]
macro_rules! unw_tdep_getcontext {
    ($uc:expr) => {
        $crate::unw_tdep_getcontext($uc)
    };
}

extern "C" {
    #[link_name = "_Uriscv_getcontext"]
    pub fn unw_tdep_getcontext(ctx: *mut unw_tdep_context_t) -> c_int;

    #[link_name = "_Uriscv_init_local"]
    pub fn unw_init_local(cur: *mut unw_cursor_t, ctx: *mut unw_context_t) -> c_int;

    #[link_name = "_Uriscv_init_remote"]
    pub fn unw_init_remote(cur: *mut unw_cursor_t, spc: unw_addr_space_t, p: *mut c_void) -> c_int;

    #[link_name = "_Uriscv_step"]
    pub fn unw_step(cur: *mut unw_cursor_t) -> c_int;

    #[link_name = "_Uriscv_get_reg"]
    pub fn unw_get_reg(cur: *mut unw_cursor_t, reg: unw_regnum_t, valp: *mut unw_word_t) -> c_int;

    #[link_name = "_Uriscv_set_reg"]
    pub fn unw_set_reg(cur: *mut unw_cursor_t, reg: unw_regnum_t, val: unw_word_t) -> c_int;

    #[link_name = "_Uriscv_resume"]
    pub fn unw_resume(cur: *mut unw_cursor_t) -> c_int;

    #[link_name = "_Uriscv_create_addr_space"]
    pub fn unw_create_addr_space(
        accessors: *mut unw_accessors_t,
        byteorder: c_int,
    ) -> unw_addr_space_t;

    #[link_name = "_Uriscv_destroy_addr_space"]
    pub fn unw_destroy_addr_space(spc: unw_addr_space_t);

    #[link_name = "_Uriscv_get_accessors"]
    pub fn unw_get_accessors(spc: unw_addr_space_t) -> *mut unw_accessors_t;

    #[link_name = "_Uriscv_flush_cache"]
    pub fn unw_flush_cache(spc: unw_addr_space_t, lo: unw_word_t, hi: unw_word_t);

    #[link_name = "_Uriscv_set_caching_policy"]
    pub fn unw_set_caching_policy(spc: unw_addr_space_t, policy: unw_caching_policy_t) -> c_int;

    #[link_name = "_Uriscv_regname"]
    pub fn unw_regname(reg: unw_regnum_t) -> *const c_char;

    #[link_name = "_Uriscv_get_proc_info"]
    pub fn unw_get_proc_info(cur: *mut unw_cursor_t, info: *mut unw_proc_info_t) -> c_int;

    #[link_name = "_Uriscv_get_save_loc"]
    pub fn unw_get_save_loc(cur: *mut unw_cursor_t, a: c_int, p: *mut unw_save_loc_t) -> c_int;

    #[link_name = "_Uriscv_is_fpreg"]
    pub fn unw_tdep_is_fpreg(reg: unw_regnum_t) -> c_int;

    #[link_name = "_Uriscv_is_signal_frame"]
    pub fn unw_is_signal_frame(cur: *mut unw_cursor_t) -> c_int;

    #[link_name = "_Uriscv_get_proc_name"]
    pub fn unw_get_proc_name(
        cur: *mut unw_cursor_t,
        buf: *mut c_char,
        len: size_t,
        offp: *mut unw_word_t,
    ) -> c_int;

    #[link_name = "_Uriscv_strerror"]
    pub fn unw_strerror(err_code: c_int) -> *const c_char;

    #[link_name = "_Uriscv_local_addr_space"]
    pub static unw_local_addr_space: unw_addr_space_t;
}
