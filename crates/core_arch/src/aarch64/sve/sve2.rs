// This code is automatically generated. DO NOT MODIFY.
//
// Instead, modify `crates/stdarch-gen2/sve2.spec.yml` and run the following command to re-generate this file:
//
// ```
// cargo run --bin=stdarch-gen2 -- crates/stdarch-gen2/spec
// ```
#![allow(improper_ctypes)]

#[cfg(test)]
use stdarch_test::assert_instr;

use super::*;

#[doc = "Absolute difference and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaba[_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saba))]
pub fn svaba_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.saba.nxv16i8")]
        fn _svaba_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t;
    }
    unsafe { _svaba_s8(op1, op2, op3) }
}
#[doc = "Absolute difference and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaba[_n_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saba))]
pub fn svaba_n_s8(op1: svint8_t, op2: svint8_t, op3: i8) -> svint8_t {
    svaba_s8(op1, op2, svdup_n_s8(op3))
}
#[doc = "Absolute difference and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaba[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saba))]
pub fn svaba_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.saba.nxv8i16")]
        fn _svaba_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t;
    }
    unsafe { _svaba_s16(op1, op2, op3) }
}
#[doc = "Absolute difference and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaba[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saba))]
pub fn svaba_n_s16(op1: svint16_t, op2: svint16_t, op3: i16) -> svint16_t {
    svaba_s16(op1, op2, svdup_n_s16(op3))
}
#[doc = "Absolute difference and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaba[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saba))]
pub fn svaba_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.saba.nxv4i32")]
        fn _svaba_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _svaba_s32(op1, op2, op3) }
}
#[doc = "Absolute difference and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaba[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saba))]
pub fn svaba_n_s32(op1: svint32_t, op2: svint32_t, op3: i32) -> svint32_t {
    svaba_s32(op1, op2, svdup_n_s32(op3))
}
#[doc = "Absolute difference and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaba[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saba))]
pub fn svaba_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.saba.nxv2i64")]
        fn _svaba_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _svaba_s64(op1, op2, op3) }
}
#[doc = "Absolute difference and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaba[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saba))]
pub fn svaba_n_s64(op1: svint64_t, op2: svint64_t, op3: i64) -> svint64_t {
    svaba_s64(op1, op2, svdup_n_s64(op3))
}
#[doc = "Absolute difference and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaba[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaba))]
pub fn svaba_u8(op1: svuint8_t, op2: svuint8_t, op3: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uaba.nxv16i8")]
        fn _svaba_u8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t;
    }
    unsafe { _svaba_u8(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Absolute difference and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaba[_n_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaba))]
pub fn svaba_n_u8(op1: svuint8_t, op2: svuint8_t, op3: u8) -> svuint8_t {
    svaba_u8(op1, op2, svdup_n_u8(op3))
}
#[doc = "Absolute difference and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaba[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaba))]
pub fn svaba_u16(op1: svuint16_t, op2: svuint16_t, op3: svuint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uaba.nxv8i16")]
        fn _svaba_u16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t;
    }
    unsafe { _svaba_u16(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Absolute difference and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaba[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaba))]
pub fn svaba_n_u16(op1: svuint16_t, op2: svuint16_t, op3: u16) -> svuint16_t {
    svaba_u16(op1, op2, svdup_n_u16(op3))
}
#[doc = "Absolute difference and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaba[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaba))]
pub fn svaba_u32(op1: svuint32_t, op2: svuint32_t, op3: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uaba.nxv4i32")]
        fn _svaba_u32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _svaba_u32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Absolute difference and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaba[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaba))]
pub fn svaba_n_u32(op1: svuint32_t, op2: svuint32_t, op3: u32) -> svuint32_t {
    svaba_u32(op1, op2, svdup_n_u32(op3))
}
#[doc = "Absolute difference and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaba[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaba))]
pub fn svaba_u64(op1: svuint64_t, op2: svuint64_t, op3: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uaba.nxv2i64")]
        fn _svaba_u64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _svaba_u64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Absolute difference and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaba[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaba))]
pub fn svaba_n_u64(op1: svuint64_t, op2: svuint64_t, op3: u64) -> svuint64_t {
    svaba_u64(op1, op2, svdup_n_u64(op3))
}
#[doc = "Absolute difference long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabalb[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sabalb))]
pub fn svabalb_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sabalb.nxv8i16")]
        fn _svabalb_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svabalb_s16(op1, op2, op3) }
}
#[doc = "Absolute difference long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabalb[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sabalb))]
pub fn svabalb_n_s16(op1: svint16_t, op2: svint8_t, op3: i8) -> svint16_t {
    svabalb_s16(op1, op2, svdup_n_s8(op3))
}
#[doc = "Absolute difference long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabalb[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sabalb))]
pub fn svabalb_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sabalb.nxv4i32")]
        fn _svabalb_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svabalb_s32(op1, op2, op3) }
}
#[doc = "Absolute difference long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabalb[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sabalb))]
pub fn svabalb_n_s32(op1: svint32_t, op2: svint16_t, op3: i16) -> svint32_t {
    svabalb_s32(op1, op2, svdup_n_s16(op3))
}
#[doc = "Absolute difference long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabalb[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sabalb))]
pub fn svabalb_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sabalb.nxv2i64")]
        fn _svabalb_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svabalb_s64(op1, op2, op3) }
}
#[doc = "Absolute difference long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabalb[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sabalb))]
pub fn svabalb_n_s64(op1: svint64_t, op2: svint32_t, op3: i32) -> svint64_t {
    svabalb_s64(op1, op2, svdup_n_s32(op3))
}
#[doc = "Absolute difference long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabalb[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uabalb))]
pub fn svabalb_u16(op1: svuint16_t, op2: svuint8_t, op3: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uabalb.nxv8i16")]
        fn _svabalb_u16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svabalb_u16(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Absolute difference long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabalb[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uabalb))]
pub fn svabalb_n_u16(op1: svuint16_t, op2: svuint8_t, op3: u8) -> svuint16_t {
    svabalb_u16(op1, op2, svdup_n_u8(op3))
}
#[doc = "Absolute difference long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabalb[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uabalb))]
pub fn svabalb_u32(op1: svuint32_t, op2: svuint16_t, op3: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uabalb.nxv4i32")]
        fn _svabalb_u32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svabalb_u32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Absolute difference long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabalb[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uabalb))]
pub fn svabalb_n_u32(op1: svuint32_t, op2: svuint16_t, op3: u16) -> svuint32_t {
    svabalb_u32(op1, op2, svdup_n_u16(op3))
}
#[doc = "Absolute difference long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabalb[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uabalb))]
pub fn svabalb_u64(op1: svuint64_t, op2: svuint32_t, op3: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uabalb.nxv2i64")]
        fn _svabalb_u64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svabalb_u64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Absolute difference long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabalb[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uabalb))]
pub fn svabalb_n_u64(op1: svuint64_t, op2: svuint32_t, op3: u32) -> svuint64_t {
    svabalb_u64(op1, op2, svdup_n_u32(op3))
}
#[doc = "Absolute difference long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabalt[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sabalt))]
pub fn svabalt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sabalt.nxv8i16")]
        fn _svabalt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svabalt_s16(op1, op2, op3) }
}
#[doc = "Absolute difference long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabalt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sabalt))]
pub fn svabalt_n_s16(op1: svint16_t, op2: svint8_t, op3: i8) -> svint16_t {
    svabalt_s16(op1, op2, svdup_n_s8(op3))
}
#[doc = "Absolute difference long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabalt[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sabalt))]
pub fn svabalt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sabalt.nxv4i32")]
        fn _svabalt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svabalt_s32(op1, op2, op3) }
}
#[doc = "Absolute difference long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabalt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sabalt))]
pub fn svabalt_n_s32(op1: svint32_t, op2: svint16_t, op3: i16) -> svint32_t {
    svabalt_s32(op1, op2, svdup_n_s16(op3))
}
#[doc = "Absolute difference long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabalt[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sabalt))]
pub fn svabalt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sabalt.nxv2i64")]
        fn _svabalt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svabalt_s64(op1, op2, op3) }
}
#[doc = "Absolute difference long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabalt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sabalt))]
pub fn svabalt_n_s64(op1: svint64_t, op2: svint32_t, op3: i32) -> svint64_t {
    svabalt_s64(op1, op2, svdup_n_s32(op3))
}
#[doc = "Absolute difference long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabalt[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uabalt))]
pub fn svabalt_u16(op1: svuint16_t, op2: svuint8_t, op3: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uabalt.nxv8i16")]
        fn _svabalt_u16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svabalt_u16(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Absolute difference long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabalt[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uabalt))]
pub fn svabalt_n_u16(op1: svuint16_t, op2: svuint8_t, op3: u8) -> svuint16_t {
    svabalt_u16(op1, op2, svdup_n_u8(op3))
}
#[doc = "Absolute difference long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabalt[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uabalt))]
pub fn svabalt_u32(op1: svuint32_t, op2: svuint16_t, op3: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uabalt.nxv4i32")]
        fn _svabalt_u32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svabalt_u32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Absolute difference long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabalt[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uabalt))]
pub fn svabalt_n_u32(op1: svuint32_t, op2: svuint16_t, op3: u16) -> svuint32_t {
    svabalt_u32(op1, op2, svdup_n_u16(op3))
}
#[doc = "Absolute difference long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabalt[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uabalt))]
pub fn svabalt_u64(op1: svuint64_t, op2: svuint32_t, op3: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uabalt.nxv2i64")]
        fn _svabalt_u64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svabalt_u64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Absolute difference long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabalt[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uabalt))]
pub fn svabalt_n_u64(op1: svuint64_t, op2: svuint32_t, op3: u32) -> svuint64_t {
    svabalt_u64(op1, op2, svdup_n_u32(op3))
}
#[doc = "Absolute difference long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabdlb[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sabdlb))]
pub fn svabdlb_s16(op1: svint8_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sabdlb.nxv8i16")]
        fn _svabdlb_s16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svabdlb_s16(op1, op2) }
}
#[doc = "Absolute difference long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabdlb[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sabdlb))]
pub fn svabdlb_n_s16(op1: svint8_t, op2: i8) -> svint16_t {
    svabdlb_s16(op1, svdup_n_s8(op2))
}
#[doc = "Absolute difference long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabdlb[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sabdlb))]
pub fn svabdlb_s32(op1: svint16_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sabdlb.nxv4i32")]
        fn _svabdlb_s32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svabdlb_s32(op1, op2) }
}
#[doc = "Absolute difference long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabdlb[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sabdlb))]
pub fn svabdlb_n_s32(op1: svint16_t, op2: i16) -> svint32_t {
    svabdlb_s32(op1, svdup_n_s16(op2))
}
#[doc = "Absolute difference long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabdlb[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sabdlb))]
pub fn svabdlb_s64(op1: svint32_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sabdlb.nxv2i64")]
        fn _svabdlb_s64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svabdlb_s64(op1, op2) }
}
#[doc = "Absolute difference long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabdlb[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sabdlb))]
pub fn svabdlb_n_s64(op1: svint32_t, op2: i32) -> svint64_t {
    svabdlb_s64(op1, svdup_n_s32(op2))
}
#[doc = "Absolute difference long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabdlb[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uabdlb))]
pub fn svabdlb_u16(op1: svuint8_t, op2: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uabdlb.nxv8i16")]
        fn _svabdlb_u16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svabdlb_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Absolute difference long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabdlb[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uabdlb))]
pub fn svabdlb_n_u16(op1: svuint8_t, op2: u8) -> svuint16_t {
    svabdlb_u16(op1, svdup_n_u8(op2))
}
#[doc = "Absolute difference long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabdlb[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uabdlb))]
pub fn svabdlb_u32(op1: svuint16_t, op2: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uabdlb.nxv4i32")]
        fn _svabdlb_u32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svabdlb_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Absolute difference long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabdlb[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uabdlb))]
pub fn svabdlb_n_u32(op1: svuint16_t, op2: u16) -> svuint32_t {
    svabdlb_u32(op1, svdup_n_u16(op2))
}
#[doc = "Absolute difference long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabdlb[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uabdlb))]
pub fn svabdlb_u64(op1: svuint32_t, op2: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uabdlb.nxv2i64")]
        fn _svabdlb_u64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svabdlb_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Absolute difference long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabdlb[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uabdlb))]
pub fn svabdlb_n_u64(op1: svuint32_t, op2: u32) -> svuint64_t {
    svabdlb_u64(op1, svdup_n_u32(op2))
}
#[doc = "Absolute difference long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabdlt[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sabdlt))]
pub fn svabdlt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sabdlt.nxv8i16")]
        fn _svabdlt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svabdlt_s16(op1, op2) }
}
#[doc = "Absolute difference long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabdlt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sabdlt))]
pub fn svabdlt_n_s16(op1: svint8_t, op2: i8) -> svint16_t {
    svabdlt_s16(op1, svdup_n_s8(op2))
}
#[doc = "Absolute difference long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabdlt[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sabdlt))]
pub fn svabdlt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sabdlt.nxv4i32")]
        fn _svabdlt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svabdlt_s32(op1, op2) }
}
#[doc = "Absolute difference long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabdlt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sabdlt))]
pub fn svabdlt_n_s32(op1: svint16_t, op2: i16) -> svint32_t {
    svabdlt_s32(op1, svdup_n_s16(op2))
}
#[doc = "Absolute difference long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabdlt[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sabdlt))]
pub fn svabdlt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sabdlt.nxv2i64")]
        fn _svabdlt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svabdlt_s64(op1, op2) }
}
#[doc = "Absolute difference long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabdlt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sabdlt))]
pub fn svabdlt_n_s64(op1: svint32_t, op2: i32) -> svint64_t {
    svabdlt_s64(op1, svdup_n_s32(op2))
}
#[doc = "Absolute difference long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabdlt[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uabdlt))]
pub fn svabdlt_u16(op1: svuint8_t, op2: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uabdlt.nxv8i16")]
        fn _svabdlt_u16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svabdlt_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Absolute difference long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabdlt[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uabdlt))]
pub fn svabdlt_n_u16(op1: svuint8_t, op2: u8) -> svuint16_t {
    svabdlt_u16(op1, svdup_n_u8(op2))
}
#[doc = "Absolute difference long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabdlt[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uabdlt))]
pub fn svabdlt_u32(op1: svuint16_t, op2: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uabdlt.nxv4i32")]
        fn _svabdlt_u32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svabdlt_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Absolute difference long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabdlt[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uabdlt))]
pub fn svabdlt_n_u32(op1: svuint16_t, op2: u16) -> svuint32_t {
    svabdlt_u32(op1, svdup_n_u16(op2))
}
#[doc = "Absolute difference long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabdlt[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uabdlt))]
pub fn svabdlt_u64(op1: svuint32_t, op2: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uabdlt.nxv2i64")]
        fn _svabdlt_u64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svabdlt_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Absolute difference long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svabdlt[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uabdlt))]
pub fn svabdlt_n_u64(op1: svuint32_t, op2: u32) -> svuint64_t {
    svabdlt_u64(op1, svdup_n_u32(op2))
}
#[doc = "Add and accumulate long pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svadalp[_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sadalp))]
pub fn svadalp_s16_m(pg: svbool_t, op1: svint16_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sadalp.nxv8i16")]
        fn _svadalp_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svadalp_s16_m(pg.into(), op1, op2) }
}
#[doc = "Add and accumulate long pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svadalp[_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sadalp))]
pub fn svadalp_s16_x(pg: svbool_t, op1: svint16_t, op2: svint8_t) -> svint16_t {
    svadalp_s16_m(pg, op1, op2)
}
#[doc = "Add and accumulate long pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svadalp[_s16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sadalp))]
pub fn svadalp_s16_z(pg: svbool_t, op1: svint16_t, op2: svint8_t) -> svint16_t {
    svadalp_s16_m(pg, svsel_s16(pg, op1, svdup_n_s16(0)), op2)
}
#[doc = "Add and accumulate long pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svadalp[_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sadalp))]
pub fn svadalp_s32_m(pg: svbool_t, op1: svint32_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sadalp.nxv4i32")]
        fn _svadalp_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svadalp_s32_m(pg.into(), op1, op2) }
}
#[doc = "Add and accumulate long pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svadalp[_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sadalp))]
pub fn svadalp_s32_x(pg: svbool_t, op1: svint32_t, op2: svint16_t) -> svint32_t {
    svadalp_s32_m(pg, op1, op2)
}
#[doc = "Add and accumulate long pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svadalp[_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sadalp))]
pub fn svadalp_s32_z(pg: svbool_t, op1: svint32_t, op2: svint16_t) -> svint32_t {
    svadalp_s32_m(pg, svsel_s32(pg, op1, svdup_n_s32(0)), op2)
}
#[doc = "Add and accumulate long pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svadalp[_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sadalp))]
pub fn svadalp_s64_m(pg: svbool_t, op1: svint64_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sadalp.nxv2i64")]
        fn _svadalp_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svadalp_s64_m(pg.into(), op1, op2) }
}
#[doc = "Add and accumulate long pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svadalp[_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sadalp))]
pub fn svadalp_s64_x(pg: svbool_t, op1: svint64_t, op2: svint32_t) -> svint64_t {
    svadalp_s64_m(pg, op1, op2)
}
#[doc = "Add and accumulate long pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svadalp[_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sadalp))]
pub fn svadalp_s64_z(pg: svbool_t, op1: svint64_t, op2: svint32_t) -> svint64_t {
    svadalp_s64_m(pg, svsel_s64(pg, op1, svdup_n_s64(0)), op2)
}
#[doc = "Add and accumulate long pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svadalp[_u16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uadalp))]
pub fn svadalp_u16_m(pg: svbool_t, op1: svuint16_t, op2: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uadalp.nxv8i16")]
        fn _svadalp_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svadalp_u16_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Add and accumulate long pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svadalp[_u16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uadalp))]
pub fn svadalp_u16_x(pg: svbool_t, op1: svuint16_t, op2: svuint8_t) -> svuint16_t {
    svadalp_u16_m(pg, op1, op2)
}
#[doc = "Add and accumulate long pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svadalp[_u16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uadalp))]
pub fn svadalp_u16_z(pg: svbool_t, op1: svuint16_t, op2: svuint8_t) -> svuint16_t {
    svadalp_u16_m(pg, svsel_u16(pg, op1, svdup_n_u16(0)), op2)
}
#[doc = "Add and accumulate long pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svadalp[_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uadalp))]
pub fn svadalp_u32_m(pg: svbool_t, op1: svuint32_t, op2: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uadalp.nxv4i32")]
        fn _svadalp_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svadalp_u32_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Add and accumulate long pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svadalp[_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uadalp))]
pub fn svadalp_u32_x(pg: svbool_t, op1: svuint32_t, op2: svuint16_t) -> svuint32_t {
    svadalp_u32_m(pg, op1, op2)
}
#[doc = "Add and accumulate long pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svadalp[_u32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uadalp))]
pub fn svadalp_u32_z(pg: svbool_t, op1: svuint32_t, op2: svuint16_t) -> svuint32_t {
    svadalp_u32_m(pg, svsel_u32(pg, op1, svdup_n_u32(0)), op2)
}
#[doc = "Add and accumulate long pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svadalp[_u64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uadalp))]
pub fn svadalp_u64_m(pg: svbool_t, op1: svuint64_t, op2: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uadalp.nxv2i64")]
        fn _svadalp_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svadalp_u64_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Add and accumulate long pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svadalp[_u64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uadalp))]
pub fn svadalp_u64_x(pg: svbool_t, op1: svuint64_t, op2: svuint32_t) -> svuint64_t {
    svadalp_u64_m(pg, op1, op2)
}
#[doc = "Add and accumulate long pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svadalp[_u64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uadalp))]
pub fn svadalp_u64_z(pg: svbool_t, op1: svuint64_t, op2: svuint32_t) -> svuint64_t {
    svadalp_u64_m(pg, svsel_u64(pg, op1, svdup_n_u64(0)), op2)
}
#[doc = "Add with carry long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svadclb[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(adclb))]
pub fn svadclb_u32(op1: svuint32_t, op2: svuint32_t, op3: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.adclb.nxv4i32")]
        fn _svadclb_u32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _svadclb_u32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Add with carry long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svadclb[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(adclb))]
pub fn svadclb_n_u32(op1: svuint32_t, op2: svuint32_t, op3: u32) -> svuint32_t {
    svadclb_u32(op1, op2, svdup_n_u32(op3))
}
#[doc = "Add with carry long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svadclb[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(adclb))]
pub fn svadclb_u64(op1: svuint64_t, op2: svuint64_t, op3: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.adclb.nxv2i64")]
        fn _svadclb_u64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _svadclb_u64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Add with carry long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svadclb[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(adclb))]
pub fn svadclb_n_u64(op1: svuint64_t, op2: svuint64_t, op3: u64) -> svuint64_t {
    svadclb_u64(op1, op2, svdup_n_u64(op3))
}
#[doc = "Add with carry long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svadclt[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(adclt))]
pub fn svadclt_u32(op1: svuint32_t, op2: svuint32_t, op3: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.adclt.nxv4i32")]
        fn _svadclt_u32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _svadclt_u32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Add with carry long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svadclt[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(adclt))]
pub fn svadclt_n_u32(op1: svuint32_t, op2: svuint32_t, op3: u32) -> svuint32_t {
    svadclt_u32(op1, op2, svdup_n_u32(op3))
}
#[doc = "Add with carry long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svadclt[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(adclt))]
pub fn svadclt_u64(op1: svuint64_t, op2: svuint64_t, op3: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.adclt.nxv2i64")]
        fn _svadclt_u64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _svadclt_u64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Add with carry long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svadclt[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(adclt))]
pub fn svadclt_n_u64(op1: svuint64_t, op2: svuint64_t, op3: u64) -> svuint64_t {
    svadclt_u64(op1, op2, svdup_n_u64(op3))
}
#[doc = "Add narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddhnb[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addhnb))]
pub fn svaddhnb_s16(op1: svint16_t, op2: svint16_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.addhnb.nxv8i16")]
        fn _svaddhnb_s16(op1: svint16_t, op2: svint16_t) -> svint8_t;
    }
    unsafe { _svaddhnb_s16(op1, op2) }
}
#[doc = "Add narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddhnb[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addhnb))]
pub fn svaddhnb_n_s16(op1: svint16_t, op2: i16) -> svint8_t {
    svaddhnb_s16(op1, svdup_n_s16(op2))
}
#[doc = "Add narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddhnb[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addhnb))]
pub fn svaddhnb_s32(op1: svint32_t, op2: svint32_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.addhnb.nxv4i32")]
        fn _svaddhnb_s32(op1: svint32_t, op2: svint32_t) -> svint16_t;
    }
    unsafe { _svaddhnb_s32(op1, op2) }
}
#[doc = "Add narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddhnb[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addhnb))]
pub fn svaddhnb_n_s32(op1: svint32_t, op2: i32) -> svint16_t {
    svaddhnb_s32(op1, svdup_n_s32(op2))
}
#[doc = "Add narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddhnb[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addhnb))]
pub fn svaddhnb_s64(op1: svint64_t, op2: svint64_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.addhnb.nxv2i64")]
        fn _svaddhnb_s64(op1: svint64_t, op2: svint64_t) -> svint32_t;
    }
    unsafe { _svaddhnb_s64(op1, op2) }
}
#[doc = "Add narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddhnb[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addhnb))]
pub fn svaddhnb_n_s64(op1: svint64_t, op2: i64) -> svint32_t {
    svaddhnb_s64(op1, svdup_n_s64(op2))
}
#[doc = "Add narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddhnb[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addhnb))]
pub fn svaddhnb_u16(op1: svuint16_t, op2: svuint16_t) -> svuint8_t {
    unsafe { svaddhnb_s16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Add narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddhnb[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addhnb))]
pub fn svaddhnb_n_u16(op1: svuint16_t, op2: u16) -> svuint8_t {
    svaddhnb_u16(op1, svdup_n_u16(op2))
}
#[doc = "Add narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddhnb[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addhnb))]
pub fn svaddhnb_u32(op1: svuint32_t, op2: svuint32_t) -> svuint16_t {
    unsafe { svaddhnb_s32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Add narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddhnb[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addhnb))]
pub fn svaddhnb_n_u32(op1: svuint32_t, op2: u32) -> svuint16_t {
    svaddhnb_u32(op1, svdup_n_u32(op2))
}
#[doc = "Add narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddhnb[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addhnb))]
pub fn svaddhnb_u64(op1: svuint64_t, op2: svuint64_t) -> svuint32_t {
    unsafe { svaddhnb_s64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Add narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddhnb[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addhnb))]
pub fn svaddhnb_n_u64(op1: svuint64_t, op2: u64) -> svuint32_t {
    svaddhnb_u64(op1, svdup_n_u64(op2))
}
#[doc = "Add narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddhnt[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addhnt))]
pub fn svaddhnt_s16(even: svint8_t, op1: svint16_t, op2: svint16_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.addhnt.nxv8i16")]
        fn _svaddhnt_s16(even: svint8_t, op1: svint16_t, op2: svint16_t) -> svint8_t;
    }
    unsafe { _svaddhnt_s16(even, op1, op2) }
}
#[doc = "Add narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddhnt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addhnt))]
pub fn svaddhnt_n_s16(even: svint8_t, op1: svint16_t, op2: i16) -> svint8_t {
    svaddhnt_s16(even, op1, svdup_n_s16(op2))
}
#[doc = "Add narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddhnt[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addhnt))]
pub fn svaddhnt_s32(even: svint16_t, op1: svint32_t, op2: svint32_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.addhnt.nxv4i32")]
        fn _svaddhnt_s32(even: svint16_t, op1: svint32_t, op2: svint32_t) -> svint16_t;
    }
    unsafe { _svaddhnt_s32(even, op1, op2) }
}
#[doc = "Add narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddhnt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addhnt))]
pub fn svaddhnt_n_s32(even: svint16_t, op1: svint32_t, op2: i32) -> svint16_t {
    svaddhnt_s32(even, op1, svdup_n_s32(op2))
}
#[doc = "Add narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddhnt[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addhnt))]
pub fn svaddhnt_s64(even: svint32_t, op1: svint64_t, op2: svint64_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.addhnt.nxv2i64")]
        fn _svaddhnt_s64(even: svint32_t, op1: svint64_t, op2: svint64_t) -> svint32_t;
    }
    unsafe { _svaddhnt_s64(even, op1, op2) }
}
#[doc = "Add narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddhnt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addhnt))]
pub fn svaddhnt_n_s64(even: svint32_t, op1: svint64_t, op2: i64) -> svint32_t {
    svaddhnt_s64(even, op1, svdup_n_s64(op2))
}
#[doc = "Add narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddhnt[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addhnt))]
pub fn svaddhnt_u16(even: svuint8_t, op1: svuint16_t, op2: svuint16_t) -> svuint8_t {
    unsafe { svaddhnt_s16(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Add narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddhnt[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addhnt))]
pub fn svaddhnt_n_u16(even: svuint8_t, op1: svuint16_t, op2: u16) -> svuint8_t {
    svaddhnt_u16(even, op1, svdup_n_u16(op2))
}
#[doc = "Add narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddhnt[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addhnt))]
pub fn svaddhnt_u32(even: svuint16_t, op1: svuint32_t, op2: svuint32_t) -> svuint16_t {
    unsafe { svaddhnt_s32(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Add narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddhnt[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addhnt))]
pub fn svaddhnt_n_u32(even: svuint16_t, op1: svuint32_t, op2: u32) -> svuint16_t {
    svaddhnt_u32(even, op1, svdup_n_u32(op2))
}
#[doc = "Add narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddhnt[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addhnt))]
pub fn svaddhnt_u64(even: svuint32_t, op1: svuint64_t, op2: svuint64_t) -> svuint32_t {
    unsafe { svaddhnt_s64(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Add narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddhnt[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addhnt))]
pub fn svaddhnt_n_u64(even: svuint32_t, op1: svuint64_t, op2: u64) -> svuint32_t {
    svaddhnt_u64(even, op1, svdup_n_u64(op2))
}
#[doc = "Add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlb[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddlb))]
pub fn svaddlb_s16(op1: svint8_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.saddlb.nxv8i16")]
        fn _svaddlb_s16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svaddlb_s16(op1, op2) }
}
#[doc = "Add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlb[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddlb))]
pub fn svaddlb_n_s16(op1: svint8_t, op2: i8) -> svint16_t {
    svaddlb_s16(op1, svdup_n_s8(op2))
}
#[doc = "Add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlb[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddlb))]
pub fn svaddlb_s32(op1: svint16_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.saddlb.nxv4i32")]
        fn _svaddlb_s32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svaddlb_s32(op1, op2) }
}
#[doc = "Add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlb[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddlb))]
pub fn svaddlb_n_s32(op1: svint16_t, op2: i16) -> svint32_t {
    svaddlb_s32(op1, svdup_n_s16(op2))
}
#[doc = "Add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlb[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddlb))]
pub fn svaddlb_s64(op1: svint32_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.saddlb.nxv2i64")]
        fn _svaddlb_s64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svaddlb_s64(op1, op2) }
}
#[doc = "Add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlb[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddlb))]
pub fn svaddlb_n_s64(op1: svint32_t, op2: i32) -> svint64_t {
    svaddlb_s64(op1, svdup_n_s32(op2))
}
#[doc = "Add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlb[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaddlb))]
pub fn svaddlb_u16(op1: svuint8_t, op2: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uaddlb.nxv8i16")]
        fn _svaddlb_u16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svaddlb_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlb[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaddlb))]
pub fn svaddlb_n_u16(op1: svuint8_t, op2: u8) -> svuint16_t {
    svaddlb_u16(op1, svdup_n_u8(op2))
}
#[doc = "Add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlb[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaddlb))]
pub fn svaddlb_u32(op1: svuint16_t, op2: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uaddlb.nxv4i32")]
        fn _svaddlb_u32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svaddlb_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlb[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaddlb))]
pub fn svaddlb_n_u32(op1: svuint16_t, op2: u16) -> svuint32_t {
    svaddlb_u32(op1, svdup_n_u16(op2))
}
#[doc = "Add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlb[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaddlb))]
pub fn svaddlb_u64(op1: svuint32_t, op2: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uaddlb.nxv2i64")]
        fn _svaddlb_u64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svaddlb_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlb[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaddlb))]
pub fn svaddlb_n_u64(op1: svuint32_t, op2: u32) -> svuint64_t {
    svaddlb_u64(op1, svdup_n_u32(op2))
}
#[doc = "Add long (bottom + top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlbt[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddlbt))]
pub fn svaddlbt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.saddlbt.nxv8i16"
        )]
        fn _svaddlbt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svaddlbt_s16(op1, op2) }
}
#[doc = "Add long (bottom + top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlbt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddlbt))]
pub fn svaddlbt_n_s16(op1: svint8_t, op2: i8) -> svint16_t {
    svaddlbt_s16(op1, svdup_n_s8(op2))
}
#[doc = "Add long (bottom + top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlbt[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddlbt))]
pub fn svaddlbt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.saddlbt.nxv4i32"
        )]
        fn _svaddlbt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svaddlbt_s32(op1, op2) }
}
#[doc = "Add long (bottom + top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlbt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddlbt))]
pub fn svaddlbt_n_s32(op1: svint16_t, op2: i16) -> svint32_t {
    svaddlbt_s32(op1, svdup_n_s16(op2))
}
#[doc = "Add long (bottom + top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlbt[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddlbt))]
pub fn svaddlbt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.saddlbt.nxv2i64"
        )]
        fn _svaddlbt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svaddlbt_s64(op1, op2) }
}
#[doc = "Add long (bottom + top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlbt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddlbt))]
pub fn svaddlbt_n_s64(op1: svint32_t, op2: i32) -> svint64_t {
    svaddlbt_s64(op1, svdup_n_s32(op2))
}
#[doc = "Add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlt[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddlt))]
pub fn svaddlt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.saddlt.nxv8i16")]
        fn _svaddlt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svaddlt_s16(op1, op2) }
}
#[doc = "Add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddlt))]
pub fn svaddlt_n_s16(op1: svint8_t, op2: i8) -> svint16_t {
    svaddlt_s16(op1, svdup_n_s8(op2))
}
#[doc = "Add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlt[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddlt))]
pub fn svaddlt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.saddlt.nxv4i32")]
        fn _svaddlt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svaddlt_s32(op1, op2) }
}
#[doc = "Add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddlt))]
pub fn svaddlt_n_s32(op1: svint16_t, op2: i16) -> svint32_t {
    svaddlt_s32(op1, svdup_n_s16(op2))
}
#[doc = "Add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlt[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddlt))]
pub fn svaddlt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.saddlt.nxv2i64")]
        fn _svaddlt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svaddlt_s64(op1, op2) }
}
#[doc = "Add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddlt))]
pub fn svaddlt_n_s64(op1: svint32_t, op2: i32) -> svint64_t {
    svaddlt_s64(op1, svdup_n_s32(op2))
}
#[doc = "Add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlt[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaddlt))]
pub fn svaddlt_u16(op1: svuint8_t, op2: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uaddlt.nxv8i16")]
        fn _svaddlt_u16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svaddlt_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlt[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaddlt))]
pub fn svaddlt_n_u16(op1: svuint8_t, op2: u8) -> svuint16_t {
    svaddlt_u16(op1, svdup_n_u8(op2))
}
#[doc = "Add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlt[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaddlt))]
pub fn svaddlt_u32(op1: svuint16_t, op2: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uaddlt.nxv4i32")]
        fn _svaddlt_u32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svaddlt_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlt[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaddlt))]
pub fn svaddlt_n_u32(op1: svuint16_t, op2: u16) -> svuint32_t {
    svaddlt_u32(op1, svdup_n_u16(op2))
}
#[doc = "Add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlt[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaddlt))]
pub fn svaddlt_u64(op1: svuint32_t, op2: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uaddlt.nxv2i64")]
        fn _svaddlt_u64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svaddlt_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddlt[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaddlt))]
pub fn svaddlt_n_u64(op1: svuint32_t, op2: u32) -> svuint64_t {
    svaddlt_u64(op1, svdup_n_u32(op2))
}
#[doc = "Add pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddp[_f32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(faddp))]
pub fn svaddp_f32_m(pg: svbool_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.faddp.nxv4f32")]
        fn _svaddp_f32_m(pg: svbool4_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t;
    }
    unsafe { _svaddp_f32_m(pg.into(), op1, op2) }
}
#[doc = "Add pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddp[_f32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(faddp))]
pub fn svaddp_f32_x(pg: svbool_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t {
    svaddp_f32_m(pg, op1, op2)
}
#[doc = "Add pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddp[_f64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(faddp))]
pub fn svaddp_f64_m(pg: svbool_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.faddp.nxv2f64")]
        fn _svaddp_f64_m(pg: svbool2_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t;
    }
    unsafe { _svaddp_f64_m(pg.into(), op1, op2) }
}
#[doc = "Add pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddp[_f64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(faddp))]
pub fn svaddp_f64_x(pg: svbool_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t {
    svaddp_f64_m(pg, op1, op2)
}
#[doc = "Add pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddp[_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addp))]
pub fn svaddp_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.addp.nxv16i8")]
        fn _svaddp_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svaddp_s8_m(pg, op1, op2) }
}
#[doc = "Add pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddp[_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addp))]
pub fn svaddp_s8_x(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svaddp_s8_m(pg, op1, op2)
}
#[doc = "Add pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddp[_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addp))]
pub fn svaddp_s16_m(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.addp.nxv8i16")]
        fn _svaddp_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svaddp_s16_m(pg.into(), op1, op2) }
}
#[doc = "Add pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddp[_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addp))]
pub fn svaddp_s16_x(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svaddp_s16_m(pg, op1, op2)
}
#[doc = "Add pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddp[_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addp))]
pub fn svaddp_s32_m(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.addp.nxv4i32")]
        fn _svaddp_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svaddp_s32_m(pg.into(), op1, op2) }
}
#[doc = "Add pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddp[_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addp))]
pub fn svaddp_s32_x(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svaddp_s32_m(pg, op1, op2)
}
#[doc = "Add pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddp[_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addp))]
pub fn svaddp_s64_m(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.addp.nxv2i64")]
        fn _svaddp_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svaddp_s64_m(pg.into(), op1, op2) }
}
#[doc = "Add pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddp[_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addp))]
pub fn svaddp_s64_x(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svaddp_s64_m(pg, op1, op2)
}
#[doc = "Add pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddp[_u8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addp))]
pub fn svaddp_u8_m(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    unsafe { svaddp_s8_m(pg, op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Add pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddp[_u8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addp))]
pub fn svaddp_u8_x(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svaddp_u8_m(pg, op1, op2)
}
#[doc = "Add pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddp[_u16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addp))]
pub fn svaddp_u16_m(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    unsafe { svaddp_s16_m(pg, op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Add pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddp[_u16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addp))]
pub fn svaddp_u16_x(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svaddp_u16_m(pg, op1, op2)
}
#[doc = "Add pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddp[_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addp))]
pub fn svaddp_u32_m(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    unsafe { svaddp_s32_m(pg, op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Add pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddp[_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addp))]
pub fn svaddp_u32_x(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svaddp_u32_m(pg, op1, op2)
}
#[doc = "Add pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddp[_u64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addp))]
pub fn svaddp_u64_m(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    unsafe { svaddp_s64_m(pg, op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Add pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddp[_u64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(addp))]
pub fn svaddp_u64_x(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svaddp_u64_m(pg, op1, op2)
}
#[doc = "Add wide (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddwb[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddwb))]
pub fn svaddwb_s16(op1: svint16_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.saddwb.nxv8i16")]
        fn _svaddwb_s16(op1: svint16_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svaddwb_s16(op1, op2) }
}
#[doc = "Add wide (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddwb[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddwb))]
pub fn svaddwb_n_s16(op1: svint16_t, op2: i8) -> svint16_t {
    svaddwb_s16(op1, svdup_n_s8(op2))
}
#[doc = "Add wide (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddwb[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddwb))]
pub fn svaddwb_s32(op1: svint32_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.saddwb.nxv4i32")]
        fn _svaddwb_s32(op1: svint32_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svaddwb_s32(op1, op2) }
}
#[doc = "Add wide (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddwb[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddwb))]
pub fn svaddwb_n_s32(op1: svint32_t, op2: i16) -> svint32_t {
    svaddwb_s32(op1, svdup_n_s16(op2))
}
#[doc = "Add wide (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddwb[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddwb))]
pub fn svaddwb_s64(op1: svint64_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.saddwb.nxv2i64")]
        fn _svaddwb_s64(op1: svint64_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svaddwb_s64(op1, op2) }
}
#[doc = "Add wide (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddwb[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddwb))]
pub fn svaddwb_n_s64(op1: svint64_t, op2: i32) -> svint64_t {
    svaddwb_s64(op1, svdup_n_s32(op2))
}
#[doc = "Add wide (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddwb[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaddwb))]
pub fn svaddwb_u16(op1: svuint16_t, op2: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uaddwb.nxv8i16")]
        fn _svaddwb_u16(op1: svint16_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svaddwb_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Add wide (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddwb[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaddwb))]
pub fn svaddwb_n_u16(op1: svuint16_t, op2: u8) -> svuint16_t {
    svaddwb_u16(op1, svdup_n_u8(op2))
}
#[doc = "Add wide (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddwb[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaddwb))]
pub fn svaddwb_u32(op1: svuint32_t, op2: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uaddwb.nxv4i32")]
        fn _svaddwb_u32(op1: svint32_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svaddwb_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Add wide (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddwb[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaddwb))]
pub fn svaddwb_n_u32(op1: svuint32_t, op2: u16) -> svuint32_t {
    svaddwb_u32(op1, svdup_n_u16(op2))
}
#[doc = "Add wide (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddwb[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaddwb))]
pub fn svaddwb_u64(op1: svuint64_t, op2: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uaddwb.nxv2i64")]
        fn _svaddwb_u64(op1: svint64_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svaddwb_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Add wide (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddwb[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaddwb))]
pub fn svaddwb_n_u64(op1: svuint64_t, op2: u32) -> svuint64_t {
    svaddwb_u64(op1, svdup_n_u32(op2))
}
#[doc = "Add wide (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddwt[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddwt))]
pub fn svaddwt_s16(op1: svint16_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.saddwt.nxv8i16")]
        fn _svaddwt_s16(op1: svint16_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svaddwt_s16(op1, op2) }
}
#[doc = "Add wide (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddwt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddwt))]
pub fn svaddwt_n_s16(op1: svint16_t, op2: i8) -> svint16_t {
    svaddwt_s16(op1, svdup_n_s8(op2))
}
#[doc = "Add wide (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddwt[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddwt))]
pub fn svaddwt_s32(op1: svint32_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.saddwt.nxv4i32")]
        fn _svaddwt_s32(op1: svint32_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svaddwt_s32(op1, op2) }
}
#[doc = "Add wide (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddwt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddwt))]
pub fn svaddwt_n_s32(op1: svint32_t, op2: i16) -> svint32_t {
    svaddwt_s32(op1, svdup_n_s16(op2))
}
#[doc = "Add wide (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddwt[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddwt))]
pub fn svaddwt_s64(op1: svint64_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.saddwt.nxv2i64")]
        fn _svaddwt_s64(op1: svint64_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svaddwt_s64(op1, op2) }
}
#[doc = "Add wide (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddwt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(saddwt))]
pub fn svaddwt_n_s64(op1: svint64_t, op2: i32) -> svint64_t {
    svaddwt_s64(op1, svdup_n_s32(op2))
}
#[doc = "Add wide (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddwt[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaddwt))]
pub fn svaddwt_u16(op1: svuint16_t, op2: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uaddwt.nxv8i16")]
        fn _svaddwt_u16(op1: svint16_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svaddwt_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Add wide (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddwt[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaddwt))]
pub fn svaddwt_n_u16(op1: svuint16_t, op2: u8) -> svuint16_t {
    svaddwt_u16(op1, svdup_n_u8(op2))
}
#[doc = "Add wide (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddwt[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaddwt))]
pub fn svaddwt_u32(op1: svuint32_t, op2: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uaddwt.nxv4i32")]
        fn _svaddwt_u32(op1: svint32_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svaddwt_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Add wide (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddwt[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaddwt))]
pub fn svaddwt_n_u32(op1: svuint32_t, op2: u16) -> svuint32_t {
    svaddwt_u32(op1, svdup_n_u16(op2))
}
#[doc = "Add wide (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddwt[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaddwt))]
pub fn svaddwt_u64(op1: svuint64_t, op2: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uaddwt.nxv2i64")]
        fn _svaddwt_u64(op1: svint64_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svaddwt_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Add wide (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaddwt[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uaddwt))]
pub fn svaddwt_n_u64(op1: svuint64_t, op2: u32) -> svuint64_t {
    svaddwt_u64(op1, svdup_n_u32(op2))
}
#[doc = "AES single round decryption"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaesd[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
#[cfg_attr(test, assert_instr(aesd))]
pub fn svaesd_u8(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.aesd")]
        fn _svaesd_u8(op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svaesd_u8(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "AES single round encryption"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaese[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
#[cfg_attr(test, assert_instr(aese))]
pub fn svaese_u8(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.aese")]
        fn _svaese_u8(op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svaese_u8(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "AES inverse mix columns"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaesimc[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
#[cfg_attr(test, assert_instr(aesimc))]
pub fn svaesimc_u8(op: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.aesimc")]
        fn _svaesimc_u8(op: svint8_t) -> svint8_t;
    }
    unsafe { _svaesimc_u8(op.as_signed()).as_unsigned() }
}
#[doc = "AES mix columns"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svaesmc[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
#[cfg_attr(test, assert_instr(aesmc))]
pub fn svaesmc_u8(op: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.aesmc")]
        fn _svaesmc_u8(op: svint8_t) -> svint8_t;
    }
    unsafe { _svaesmc_u8(op.as_signed()).as_unsigned() }
}
#[doc = "Bitwise clear and exclusive OR"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbcax[_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bcax))]
pub fn svbcax_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bcax.nxv16i8")]
        fn _svbcax_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t;
    }
    unsafe { _svbcax_s8(op1, op2, op3) }
}
#[doc = "Bitwise clear and exclusive OR"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbcax[_n_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bcax))]
pub fn svbcax_n_s8(op1: svint8_t, op2: svint8_t, op3: i8) -> svint8_t {
    svbcax_s8(op1, op2, svdup_n_s8(op3))
}
#[doc = "Bitwise clear and exclusive OR"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbcax[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bcax))]
pub fn svbcax_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bcax.nxv8i16")]
        fn _svbcax_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t;
    }
    unsafe { _svbcax_s16(op1, op2, op3) }
}
#[doc = "Bitwise clear and exclusive OR"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbcax[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bcax))]
pub fn svbcax_n_s16(op1: svint16_t, op2: svint16_t, op3: i16) -> svint16_t {
    svbcax_s16(op1, op2, svdup_n_s16(op3))
}
#[doc = "Bitwise clear and exclusive OR"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbcax[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bcax))]
pub fn svbcax_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bcax.nxv4i32")]
        fn _svbcax_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _svbcax_s32(op1, op2, op3) }
}
#[doc = "Bitwise clear and exclusive OR"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbcax[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bcax))]
pub fn svbcax_n_s32(op1: svint32_t, op2: svint32_t, op3: i32) -> svint32_t {
    svbcax_s32(op1, op2, svdup_n_s32(op3))
}
#[doc = "Bitwise clear and exclusive OR"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbcax[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bcax))]
pub fn svbcax_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bcax.nxv2i64")]
        fn _svbcax_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _svbcax_s64(op1, op2, op3) }
}
#[doc = "Bitwise clear and exclusive OR"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbcax[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bcax))]
pub fn svbcax_n_s64(op1: svint64_t, op2: svint64_t, op3: i64) -> svint64_t {
    svbcax_s64(op1, op2, svdup_n_s64(op3))
}
#[doc = "Bitwise clear and exclusive OR"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbcax[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bcax))]
pub fn svbcax_u8(op1: svuint8_t, op2: svuint8_t, op3: svuint8_t) -> svuint8_t {
    unsafe { svbcax_s8(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Bitwise clear and exclusive OR"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbcax[_n_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bcax))]
pub fn svbcax_n_u8(op1: svuint8_t, op2: svuint8_t, op3: u8) -> svuint8_t {
    svbcax_u8(op1, op2, svdup_n_u8(op3))
}
#[doc = "Bitwise clear and exclusive OR"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbcax[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bcax))]
pub fn svbcax_u16(op1: svuint16_t, op2: svuint16_t, op3: svuint16_t) -> svuint16_t {
    unsafe { svbcax_s16(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Bitwise clear and exclusive OR"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbcax[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bcax))]
pub fn svbcax_n_u16(op1: svuint16_t, op2: svuint16_t, op3: u16) -> svuint16_t {
    svbcax_u16(op1, op2, svdup_n_u16(op3))
}
#[doc = "Bitwise clear and exclusive OR"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbcax[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bcax))]
pub fn svbcax_u32(op1: svuint32_t, op2: svuint32_t, op3: svuint32_t) -> svuint32_t {
    unsafe { svbcax_s32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Bitwise clear and exclusive OR"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbcax[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bcax))]
pub fn svbcax_n_u32(op1: svuint32_t, op2: svuint32_t, op3: u32) -> svuint32_t {
    svbcax_u32(op1, op2, svdup_n_u32(op3))
}
#[doc = "Bitwise clear and exclusive OR"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbcax[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bcax))]
pub fn svbcax_u64(op1: svuint64_t, op2: svuint64_t, op3: svuint64_t) -> svuint64_t {
    unsafe { svbcax_s64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Bitwise clear and exclusive OR"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbcax[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bcax))]
pub fn svbcax_n_u64(op1: svuint64_t, op2: svuint64_t, op3: u64) -> svuint64_t {
    svbcax_u64(op1, op2, svdup_n_u64(op3))
}
#[doc = "Scatter lower bits into positions selected by bitmask"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbdep[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
#[cfg_attr(test, assert_instr(bdep))]
pub fn svbdep_u8(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bdep.x.nxv16i8")]
        fn _svbdep_u8(op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svbdep_u8(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Scatter lower bits into positions selected by bitmask"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbdep[_n_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
#[cfg_attr(test, assert_instr(bdep))]
pub fn svbdep_n_u8(op1: svuint8_t, op2: u8) -> svuint8_t {
    svbdep_u8(op1, svdup_n_u8(op2))
}
#[doc = "Scatter lower bits into positions selected by bitmask"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbdep[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
#[cfg_attr(test, assert_instr(bdep))]
pub fn svbdep_u16(op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bdep.x.nxv8i16")]
        fn _svbdep_u16(op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svbdep_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Scatter lower bits into positions selected by bitmask"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbdep[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
#[cfg_attr(test, assert_instr(bdep))]
pub fn svbdep_n_u16(op1: svuint16_t, op2: u16) -> svuint16_t {
    svbdep_u16(op1, svdup_n_u16(op2))
}
#[doc = "Scatter lower bits into positions selected by bitmask"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbdep[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
#[cfg_attr(test, assert_instr(bdep))]
pub fn svbdep_u32(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bdep.x.nxv4i32")]
        fn _svbdep_u32(op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svbdep_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Scatter lower bits into positions selected by bitmask"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbdep[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
#[cfg_attr(test, assert_instr(bdep))]
pub fn svbdep_n_u32(op1: svuint32_t, op2: u32) -> svuint32_t {
    svbdep_u32(op1, svdup_n_u32(op2))
}
#[doc = "Scatter lower bits into positions selected by bitmask"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbdep[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
#[cfg_attr(test, assert_instr(bdep))]
pub fn svbdep_u64(op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bdep.x.nxv2i64")]
        fn _svbdep_u64(op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svbdep_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Scatter lower bits into positions selected by bitmask"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbdep[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
#[cfg_attr(test, assert_instr(bdep))]
pub fn svbdep_n_u64(op1: svuint64_t, op2: u64) -> svuint64_t {
    svbdep_u64(op1, svdup_n_u64(op2))
}
#[doc = "Gather lower bits from positions selected by bitmask"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbext[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
#[cfg_attr(test, assert_instr(bext))]
pub fn svbext_u8(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bext.x.nxv16i8")]
        fn _svbext_u8(op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svbext_u8(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Gather lower bits from positions selected by bitmask"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbext[_n_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
#[cfg_attr(test, assert_instr(bext))]
pub fn svbext_n_u8(op1: svuint8_t, op2: u8) -> svuint8_t {
    svbext_u8(op1, svdup_n_u8(op2))
}
#[doc = "Gather lower bits from positions selected by bitmask"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbext[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
#[cfg_attr(test, assert_instr(bext))]
pub fn svbext_u16(op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bext.x.nxv8i16")]
        fn _svbext_u16(op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svbext_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Gather lower bits from positions selected by bitmask"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbext[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
#[cfg_attr(test, assert_instr(bext))]
pub fn svbext_n_u16(op1: svuint16_t, op2: u16) -> svuint16_t {
    svbext_u16(op1, svdup_n_u16(op2))
}
#[doc = "Gather lower bits from positions selected by bitmask"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbext[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
#[cfg_attr(test, assert_instr(bext))]
pub fn svbext_u32(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bext.x.nxv4i32")]
        fn _svbext_u32(op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svbext_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Gather lower bits from positions selected by bitmask"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbext[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
#[cfg_attr(test, assert_instr(bext))]
pub fn svbext_n_u32(op1: svuint32_t, op2: u32) -> svuint32_t {
    svbext_u32(op1, svdup_n_u32(op2))
}
#[doc = "Gather lower bits from positions selected by bitmask"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbext[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
#[cfg_attr(test, assert_instr(bext))]
pub fn svbext_u64(op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bext.x.nxv2i64")]
        fn _svbext_u64(op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svbext_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Gather lower bits from positions selected by bitmask"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbext[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
#[cfg_attr(test, assert_instr(bext))]
pub fn svbext_n_u64(op1: svuint64_t, op2: u64) -> svuint64_t {
    svbext_u64(op1, svdup_n_u64(op2))
}
#[doc = "Group bits to right or left as selected by bitmask"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbgrp[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
#[cfg_attr(test, assert_instr(bgrp))]
pub fn svbgrp_u8(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bgrp.x.nxv16i8")]
        fn _svbgrp_u8(op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svbgrp_u8(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Group bits to right or left as selected by bitmask"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbgrp[_n_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
#[cfg_attr(test, assert_instr(bgrp))]
pub fn svbgrp_n_u8(op1: svuint8_t, op2: u8) -> svuint8_t {
    svbgrp_u8(op1, svdup_n_u8(op2))
}
#[doc = "Group bits to right or left as selected by bitmask"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbgrp[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
#[cfg_attr(test, assert_instr(bgrp))]
pub fn svbgrp_u16(op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bgrp.x.nxv8i16")]
        fn _svbgrp_u16(op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svbgrp_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Group bits to right or left as selected by bitmask"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbgrp[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
#[cfg_attr(test, assert_instr(bgrp))]
pub fn svbgrp_n_u16(op1: svuint16_t, op2: u16) -> svuint16_t {
    svbgrp_u16(op1, svdup_n_u16(op2))
}
#[doc = "Group bits to right or left as selected by bitmask"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbgrp[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
#[cfg_attr(test, assert_instr(bgrp))]
pub fn svbgrp_u32(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bgrp.x.nxv4i32")]
        fn _svbgrp_u32(op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svbgrp_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Group bits to right or left as selected by bitmask"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbgrp[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
#[cfg_attr(test, assert_instr(bgrp))]
pub fn svbgrp_n_u32(op1: svuint32_t, op2: u32) -> svuint32_t {
    svbgrp_u32(op1, svdup_n_u32(op2))
}
#[doc = "Group bits to right or left as selected by bitmask"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbgrp[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
#[cfg_attr(test, assert_instr(bgrp))]
pub fn svbgrp_u64(op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bgrp.x.nxv2i64")]
        fn _svbgrp_u64(op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svbgrp_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Group bits to right or left as selected by bitmask"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbgrp[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
#[cfg_attr(test, assert_instr(bgrp))]
pub fn svbgrp_n_u64(op1: svuint64_t, op2: u64) -> svuint64_t {
    svbgrp_u64(op1, svdup_n_u64(op2))
}
#[doc = "Bitwise select with first input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl1n[_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl1n))]
pub fn svbsl1n_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bsl1n.nxv16i8")]
        fn _svbsl1n_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t;
    }
    unsafe { _svbsl1n_s8(op1, op2, op3) }
}
#[doc = "Bitwise select with first input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl1n[_n_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl1n))]
pub fn svbsl1n_n_s8(op1: svint8_t, op2: svint8_t, op3: i8) -> svint8_t {
    svbsl1n_s8(op1, op2, svdup_n_s8(op3))
}
#[doc = "Bitwise select with first input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl1n[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl1n))]
pub fn svbsl1n_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bsl1n.nxv8i16")]
        fn _svbsl1n_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t;
    }
    unsafe { _svbsl1n_s16(op1, op2, op3) }
}
#[doc = "Bitwise select with first input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl1n[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl1n))]
pub fn svbsl1n_n_s16(op1: svint16_t, op2: svint16_t, op3: i16) -> svint16_t {
    svbsl1n_s16(op1, op2, svdup_n_s16(op3))
}
#[doc = "Bitwise select with first input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl1n[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl1n))]
pub fn svbsl1n_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bsl1n.nxv4i32")]
        fn _svbsl1n_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _svbsl1n_s32(op1, op2, op3) }
}
#[doc = "Bitwise select with first input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl1n[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl1n))]
pub fn svbsl1n_n_s32(op1: svint32_t, op2: svint32_t, op3: i32) -> svint32_t {
    svbsl1n_s32(op1, op2, svdup_n_s32(op3))
}
#[doc = "Bitwise select with first input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl1n[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl1n))]
pub fn svbsl1n_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bsl1n.nxv2i64")]
        fn _svbsl1n_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _svbsl1n_s64(op1, op2, op3) }
}
#[doc = "Bitwise select with first input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl1n[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl1n))]
pub fn svbsl1n_n_s64(op1: svint64_t, op2: svint64_t, op3: i64) -> svint64_t {
    svbsl1n_s64(op1, op2, svdup_n_s64(op3))
}
#[doc = "Bitwise select with first input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl1n[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl1n))]
pub fn svbsl1n_u8(op1: svuint8_t, op2: svuint8_t, op3: svuint8_t) -> svuint8_t {
    unsafe { svbsl1n_s8(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Bitwise select with first input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl1n[_n_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl1n))]
pub fn svbsl1n_n_u8(op1: svuint8_t, op2: svuint8_t, op3: u8) -> svuint8_t {
    svbsl1n_u8(op1, op2, svdup_n_u8(op3))
}
#[doc = "Bitwise select with first input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl1n[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl1n))]
pub fn svbsl1n_u16(op1: svuint16_t, op2: svuint16_t, op3: svuint16_t) -> svuint16_t {
    unsafe { svbsl1n_s16(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Bitwise select with first input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl1n[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl1n))]
pub fn svbsl1n_n_u16(op1: svuint16_t, op2: svuint16_t, op3: u16) -> svuint16_t {
    svbsl1n_u16(op1, op2, svdup_n_u16(op3))
}
#[doc = "Bitwise select with first input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl1n[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl1n))]
pub fn svbsl1n_u32(op1: svuint32_t, op2: svuint32_t, op3: svuint32_t) -> svuint32_t {
    unsafe { svbsl1n_s32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Bitwise select with first input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl1n[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl1n))]
pub fn svbsl1n_n_u32(op1: svuint32_t, op2: svuint32_t, op3: u32) -> svuint32_t {
    svbsl1n_u32(op1, op2, svdup_n_u32(op3))
}
#[doc = "Bitwise select with first input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl1n[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl1n))]
pub fn svbsl1n_u64(op1: svuint64_t, op2: svuint64_t, op3: svuint64_t) -> svuint64_t {
    unsafe { svbsl1n_s64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Bitwise select with first input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl1n[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl1n))]
pub fn svbsl1n_n_u64(op1: svuint64_t, op2: svuint64_t, op3: u64) -> svuint64_t {
    svbsl1n_u64(op1, op2, svdup_n_u64(op3))
}
#[doc = "Bitwise select with second input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl2n[_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl2n))]
pub fn svbsl2n_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bsl2n.nxv16i8")]
        fn _svbsl2n_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t;
    }
    unsafe { _svbsl2n_s8(op1, op2, op3) }
}
#[doc = "Bitwise select with second input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl2n[_n_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl2n))]
pub fn svbsl2n_n_s8(op1: svint8_t, op2: svint8_t, op3: i8) -> svint8_t {
    svbsl2n_s8(op1, op2, svdup_n_s8(op3))
}
#[doc = "Bitwise select with second input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl2n[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl2n))]
pub fn svbsl2n_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bsl2n.nxv8i16")]
        fn _svbsl2n_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t;
    }
    unsafe { _svbsl2n_s16(op1, op2, op3) }
}
#[doc = "Bitwise select with second input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl2n[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl2n))]
pub fn svbsl2n_n_s16(op1: svint16_t, op2: svint16_t, op3: i16) -> svint16_t {
    svbsl2n_s16(op1, op2, svdup_n_s16(op3))
}
#[doc = "Bitwise select with second input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl2n[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl2n))]
pub fn svbsl2n_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bsl2n.nxv4i32")]
        fn _svbsl2n_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _svbsl2n_s32(op1, op2, op3) }
}
#[doc = "Bitwise select with second input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl2n[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl2n))]
pub fn svbsl2n_n_s32(op1: svint32_t, op2: svint32_t, op3: i32) -> svint32_t {
    svbsl2n_s32(op1, op2, svdup_n_s32(op3))
}
#[doc = "Bitwise select with second input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl2n[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl2n))]
pub fn svbsl2n_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bsl2n.nxv2i64")]
        fn _svbsl2n_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _svbsl2n_s64(op1, op2, op3) }
}
#[doc = "Bitwise select with second input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl2n[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl2n))]
pub fn svbsl2n_n_s64(op1: svint64_t, op2: svint64_t, op3: i64) -> svint64_t {
    svbsl2n_s64(op1, op2, svdup_n_s64(op3))
}
#[doc = "Bitwise select with second input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl2n[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl2n))]
pub fn svbsl2n_u8(op1: svuint8_t, op2: svuint8_t, op3: svuint8_t) -> svuint8_t {
    unsafe { svbsl2n_s8(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Bitwise select with second input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl2n[_n_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl2n))]
pub fn svbsl2n_n_u8(op1: svuint8_t, op2: svuint8_t, op3: u8) -> svuint8_t {
    svbsl2n_u8(op1, op2, svdup_n_u8(op3))
}
#[doc = "Bitwise select with second input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl2n[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl2n))]
pub fn svbsl2n_u16(op1: svuint16_t, op2: svuint16_t, op3: svuint16_t) -> svuint16_t {
    unsafe { svbsl2n_s16(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Bitwise select with second input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl2n[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl2n))]
pub fn svbsl2n_n_u16(op1: svuint16_t, op2: svuint16_t, op3: u16) -> svuint16_t {
    svbsl2n_u16(op1, op2, svdup_n_u16(op3))
}
#[doc = "Bitwise select with second input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl2n[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl2n))]
pub fn svbsl2n_u32(op1: svuint32_t, op2: svuint32_t, op3: svuint32_t) -> svuint32_t {
    unsafe { svbsl2n_s32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Bitwise select with second input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl2n[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl2n))]
pub fn svbsl2n_n_u32(op1: svuint32_t, op2: svuint32_t, op3: u32) -> svuint32_t {
    svbsl2n_u32(op1, op2, svdup_n_u32(op3))
}
#[doc = "Bitwise select with second input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl2n[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl2n))]
pub fn svbsl2n_u64(op1: svuint64_t, op2: svuint64_t, op3: svuint64_t) -> svuint64_t {
    unsafe { svbsl2n_s64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Bitwise select with second input inverted"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl2n[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl2n))]
pub fn svbsl2n_n_u64(op1: svuint64_t, op2: svuint64_t, op3: u64) -> svuint64_t {
    svbsl2n_u64(op1, op2, svdup_n_u64(op3))
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl[_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl))]
pub fn svbsl_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bsl.nxv16i8")]
        fn _svbsl_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t;
    }
    unsafe { _svbsl_s8(op1, op2, op3) }
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl[_n_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl))]
pub fn svbsl_n_s8(op1: svint8_t, op2: svint8_t, op3: i8) -> svint8_t {
    svbsl_s8(op1, op2, svdup_n_s8(op3))
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl))]
pub fn svbsl_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bsl.nxv8i16")]
        fn _svbsl_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t;
    }
    unsafe { _svbsl_s16(op1, op2, op3) }
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl))]
pub fn svbsl_n_s16(op1: svint16_t, op2: svint16_t, op3: i16) -> svint16_t {
    svbsl_s16(op1, op2, svdup_n_s16(op3))
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl))]
pub fn svbsl_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bsl.nxv4i32")]
        fn _svbsl_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _svbsl_s32(op1, op2, op3) }
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl))]
pub fn svbsl_n_s32(op1: svint32_t, op2: svint32_t, op3: i32) -> svint32_t {
    svbsl_s32(op1, op2, svdup_n_s32(op3))
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl))]
pub fn svbsl_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.bsl.nxv2i64")]
        fn _svbsl_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _svbsl_s64(op1, op2, op3) }
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl))]
pub fn svbsl_n_s64(op1: svint64_t, op2: svint64_t, op3: i64) -> svint64_t {
    svbsl_s64(op1, op2, svdup_n_s64(op3))
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl))]
pub fn svbsl_u8(op1: svuint8_t, op2: svuint8_t, op3: svuint8_t) -> svuint8_t {
    unsafe { svbsl_s8(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl[_n_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl))]
pub fn svbsl_n_u8(op1: svuint8_t, op2: svuint8_t, op3: u8) -> svuint8_t {
    svbsl_u8(op1, op2, svdup_n_u8(op3))
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl))]
pub fn svbsl_u16(op1: svuint16_t, op2: svuint16_t, op3: svuint16_t) -> svuint16_t {
    unsafe { svbsl_s16(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl))]
pub fn svbsl_n_u16(op1: svuint16_t, op2: svuint16_t, op3: u16) -> svuint16_t {
    svbsl_u16(op1, op2, svdup_n_u16(op3))
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl))]
pub fn svbsl_u32(op1: svuint32_t, op2: svuint32_t, op3: svuint32_t) -> svuint32_t {
    unsafe { svbsl_s32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl))]
pub fn svbsl_n_u32(op1: svuint32_t, op2: svuint32_t, op3: u32) -> svuint32_t {
    svbsl_u32(op1, op2, svdup_n_u32(op3))
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl))]
pub fn svbsl_u64(op1: svuint64_t, op2: svuint64_t, op3: svuint64_t) -> svuint64_t {
    unsafe { svbsl_s64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svbsl[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(bsl))]
pub fn svbsl_n_u64(op1: svuint64_t, op2: svuint64_t, op3: u64) -> svuint64_t {
    svbsl_u64(op1, op2, svdup_n_u64(op3))
}
#[doc = "Complex add with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcadd[_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(cadd, IMM_ROTATION = 90))]
pub fn svcadd_s8<const IMM_ROTATION: i32>(op1: svint8_t, op2: svint8_t) -> svint8_t {
    static_assert!(IMM_ROTATION == 90 || IMM_ROTATION == 270);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.cadd.x.nxv16i8")]
        fn _svcadd_s8(op1: svint8_t, op2: svint8_t, imm_rotation: i32) -> svint8_t;
    }
    unsafe { _svcadd_s8(op1, op2, IMM_ROTATION) }
}
#[doc = "Complex add with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcadd[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(cadd, IMM_ROTATION = 90))]
pub fn svcadd_s16<const IMM_ROTATION: i32>(op1: svint16_t, op2: svint16_t) -> svint16_t {
    static_assert!(IMM_ROTATION == 90 || IMM_ROTATION == 270);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.cadd.x.nxv8i16")]
        fn _svcadd_s16(op1: svint16_t, op2: svint16_t, imm_rotation: i32) -> svint16_t;
    }
    unsafe { _svcadd_s16(op1, op2, IMM_ROTATION) }
}
#[doc = "Complex add with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcadd[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(cadd, IMM_ROTATION = 90))]
pub fn svcadd_s32<const IMM_ROTATION: i32>(op1: svint32_t, op2: svint32_t) -> svint32_t {
    static_assert!(IMM_ROTATION == 90 || IMM_ROTATION == 270);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.cadd.x.nxv4i32")]
        fn _svcadd_s32(op1: svint32_t, op2: svint32_t, imm_rotation: i32) -> svint32_t;
    }
    unsafe { _svcadd_s32(op1, op2, IMM_ROTATION) }
}
#[doc = "Complex add with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcadd[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(cadd, IMM_ROTATION = 90))]
pub fn svcadd_s64<const IMM_ROTATION: i32>(op1: svint64_t, op2: svint64_t) -> svint64_t {
    static_assert!(IMM_ROTATION == 90 || IMM_ROTATION == 270);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.cadd.x.nxv2i64")]
        fn _svcadd_s64(op1: svint64_t, op2: svint64_t, imm_rotation: i32) -> svint64_t;
    }
    unsafe { _svcadd_s64(op1, op2, IMM_ROTATION) }
}
#[doc = "Complex add with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcadd[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(cadd, IMM_ROTATION = 90))]
pub fn svcadd_u8<const IMM_ROTATION: i32>(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    static_assert!(IMM_ROTATION == 90 || IMM_ROTATION == 270);
    unsafe { svcadd_s8::<IMM_ROTATION>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Complex add with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcadd[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(cadd, IMM_ROTATION = 90))]
pub fn svcadd_u16<const IMM_ROTATION: i32>(op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    static_assert!(IMM_ROTATION == 90 || IMM_ROTATION == 270);
    unsafe { svcadd_s16::<IMM_ROTATION>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Complex add with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcadd[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(cadd, IMM_ROTATION = 90))]
pub fn svcadd_u32<const IMM_ROTATION: i32>(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    static_assert!(IMM_ROTATION == 90 || IMM_ROTATION == 270);
    unsafe { svcadd_s32::<IMM_ROTATION>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Complex add with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcadd[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(cadd, IMM_ROTATION = 90))]
pub fn svcadd_u64<const IMM_ROTATION: i32>(op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    static_assert!(IMM_ROTATION == 90 || IMM_ROTATION == 270);
    unsafe { svcadd_s64::<IMM_ROTATION>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Complex dot product"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcdot_lane[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(cdot, IMM_INDEX = 0, IMM_ROTATION = 90))]
pub fn svcdot_lane_s32<const IMM_INDEX: i32, const IMM_ROTATION: i32>(
    op1: svint32_t,
    op2: svint8_t,
    op3: svint8_t,
) -> svint32_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.cdot.lane.nxv4i32"
        )]
        fn _svcdot_lane_s32(
            op1: svint32_t,
            op2: svint8_t,
            op3: svint8_t,
            imm_index: i32,
            imm_rotation: i32,
        ) -> svint32_t;
    }
    unsafe { _svcdot_lane_s32(op1, op2, op3, IMM_INDEX, IMM_ROTATION) }
}
#[doc = "Complex dot product"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcdot_lane[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(cdot, IMM_INDEX = 0, IMM_ROTATION = 90))]
pub fn svcdot_lane_s64<const IMM_INDEX: i32, const IMM_ROTATION: i32>(
    op1: svint64_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint64_t {
    static_assert_range!(IMM_INDEX, 0, 1);
    static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.cdot.lane.nxv2i64"
        )]
        fn _svcdot_lane_s64(
            op1: svint64_t,
            op2: svint16_t,
            op3: svint16_t,
            imm_index: i32,
            imm_rotation: i32,
        ) -> svint64_t;
    }
    unsafe { _svcdot_lane_s64(op1, op2, op3, IMM_INDEX, IMM_ROTATION) }
}
#[doc = "Complex dot product"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcdot[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(cdot, IMM_ROTATION = 90))]
pub fn svcdot_s32<const IMM_ROTATION: i32>(
    op1: svint32_t,
    op2: svint8_t,
    op3: svint8_t,
) -> svint32_t {
    static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.cdot.nxv4i32")]
        fn _svcdot_s32(
            op1: svint32_t,
            op2: svint8_t,
            op3: svint8_t,
            imm_rotation: i32,
        ) -> svint32_t;
    }
    unsafe { _svcdot_s32(op1, op2, op3, IMM_ROTATION) }
}
#[doc = "Complex dot product"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcdot[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(cdot, IMM_ROTATION = 90))]
pub fn svcdot_s64<const IMM_ROTATION: i32>(
    op1: svint64_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint64_t {
    static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.cdot.nxv2i64")]
        fn _svcdot_s64(
            op1: svint64_t,
            op2: svint16_t,
            op3: svint16_t,
            imm_rotation: i32,
        ) -> svint64_t;
    }
    unsafe { _svcdot_s64(op1, op2, op3, IMM_ROTATION) }
}
#[doc = "Complex multiply-add with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcmla_lane[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(cmla, IMM_INDEX = 0, IMM_ROTATION = 90))]
pub fn svcmla_lane_s16<const IMM_INDEX: i32, const IMM_ROTATION: i32>(
    op1: svint16_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint16_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.cmla.lane.x.nxv8i16"
        )]
        fn _svcmla_lane_s16(
            op1: svint16_t,
            op2: svint16_t,
            op3: svint16_t,
            imm_index: i32,
            imm_rotation: i32,
        ) -> svint16_t;
    }
    unsafe { _svcmla_lane_s16(op1, op2, op3, IMM_INDEX, IMM_ROTATION) }
}
#[doc = "Complex multiply-add with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcmla_lane[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(cmla, IMM_INDEX = 0, IMM_ROTATION = 90))]
pub fn svcmla_lane_s32<const IMM_INDEX: i32, const IMM_ROTATION: i32>(
    op1: svint32_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint32_t {
    static_assert_range!(IMM_INDEX, 0, 1);
    static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.cmla.lane.x.nxv4i32"
        )]
        fn _svcmla_lane_s32(
            op1: svint32_t,
            op2: svint32_t,
            op3: svint32_t,
            imm_index: i32,
            imm_rotation: i32,
        ) -> svint32_t;
    }
    unsafe { _svcmla_lane_s32(op1, op2, op3, IMM_INDEX, IMM_ROTATION) }
}
#[doc = "Complex multiply-add with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcmla_lane[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(cmla, IMM_INDEX = 0, IMM_ROTATION = 90))]
pub fn svcmla_lane_u16<const IMM_INDEX: i32, const IMM_ROTATION: i32>(
    op1: svuint16_t,
    op2: svuint16_t,
    op3: svuint16_t,
) -> svuint16_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );
    unsafe {
        svcmla_lane_s16::<IMM_INDEX, IMM_ROTATION>(
            op1.as_signed(),
            op2.as_signed(),
            op3.as_signed(),
        )
        .as_unsigned()
    }
}
#[doc = "Complex multiply-add with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcmla_lane[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(cmla, IMM_INDEX = 0, IMM_ROTATION = 90))]
pub fn svcmla_lane_u32<const IMM_INDEX: i32, const IMM_ROTATION: i32>(
    op1: svuint32_t,
    op2: svuint32_t,
    op3: svuint32_t,
) -> svuint32_t {
    static_assert_range!(IMM_INDEX, 0, 1);
    static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );
    unsafe {
        svcmla_lane_s32::<IMM_INDEX, IMM_ROTATION>(
            op1.as_signed(),
            op2.as_signed(),
            op3.as_signed(),
        )
        .as_unsigned()
    }
}
#[doc = "Complex multiply-add with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcmla[_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(cmla, IMM_ROTATION = 90))]
pub fn svcmla_s8<const IMM_ROTATION: i32>(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t {
    static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.cmla.x.nxv16i8")]
        fn _svcmla_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t, imm_rotation: i32) -> svint8_t;
    }
    unsafe { _svcmla_s8(op1, op2, op3, IMM_ROTATION) }
}
#[doc = "Complex multiply-add with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcmla[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(cmla, IMM_ROTATION = 90))]
pub fn svcmla_s16<const IMM_ROTATION: i32>(
    op1: svint16_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint16_t {
    static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.cmla.x.nxv8i16")]
        fn _svcmla_s16(
            op1: svint16_t,
            op2: svint16_t,
            op3: svint16_t,
            imm_rotation: i32,
        ) -> svint16_t;
    }
    unsafe { _svcmla_s16(op1, op2, op3, IMM_ROTATION) }
}
#[doc = "Complex multiply-add with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcmla[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(cmla, IMM_ROTATION = 90))]
pub fn svcmla_s32<const IMM_ROTATION: i32>(
    op1: svint32_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint32_t {
    static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.cmla.x.nxv4i32")]
        fn _svcmla_s32(
            op1: svint32_t,
            op2: svint32_t,
            op3: svint32_t,
            imm_rotation: i32,
        ) -> svint32_t;
    }
    unsafe { _svcmla_s32(op1, op2, op3, IMM_ROTATION) }
}
#[doc = "Complex multiply-add with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcmla[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(cmla, IMM_ROTATION = 90))]
pub fn svcmla_s64<const IMM_ROTATION: i32>(
    op1: svint64_t,
    op2: svint64_t,
    op3: svint64_t,
) -> svint64_t {
    static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.cmla.x.nxv2i64")]
        fn _svcmla_s64(
            op1: svint64_t,
            op2: svint64_t,
            op3: svint64_t,
            imm_rotation: i32,
        ) -> svint64_t;
    }
    unsafe { _svcmla_s64(op1, op2, op3, IMM_ROTATION) }
}
#[doc = "Complex multiply-add with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcmla[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(cmla, IMM_ROTATION = 90))]
pub fn svcmla_u8<const IMM_ROTATION: i32>(
    op1: svuint8_t,
    op2: svuint8_t,
    op3: svuint8_t,
) -> svuint8_t {
    static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );
    unsafe {
        svcmla_s8::<IMM_ROTATION>(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned()
    }
}
#[doc = "Complex multiply-add with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcmla[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(cmla, IMM_ROTATION = 90))]
pub fn svcmla_u16<const IMM_ROTATION: i32>(
    op1: svuint16_t,
    op2: svuint16_t,
    op3: svuint16_t,
) -> svuint16_t {
    static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );
    unsafe {
        svcmla_s16::<IMM_ROTATION>(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned()
    }
}
#[doc = "Complex multiply-add with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcmla[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(cmla, IMM_ROTATION = 90))]
pub fn svcmla_u32<const IMM_ROTATION: i32>(
    op1: svuint32_t,
    op2: svuint32_t,
    op3: svuint32_t,
) -> svuint32_t {
    static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );
    unsafe {
        svcmla_s32::<IMM_ROTATION>(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned()
    }
}
#[doc = "Complex multiply-add with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcmla[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(cmla, IMM_ROTATION = 90))]
pub fn svcmla_u64<const IMM_ROTATION: i32>(
    op1: svuint64_t,
    op2: svuint64_t,
    op3: svuint64_t,
) -> svuint64_t {
    static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );
    unsafe {
        svcmla_s64::<IMM_ROTATION>(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned()
    }
}
#[doc = "Up convert long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcvtlt_f64[_f32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(fcvtlt))]
pub fn svcvtlt_f64_f32_m(inactive: svfloat64_t, pg: svbool_t, op: svfloat32_t) -> svfloat64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.fcvtlt.f64f32")]
        fn _svcvtlt_f64_f32_m(inactive: svfloat64_t, pg: svbool2_t, op: svfloat32_t)
            -> svfloat64_t;
    }
    unsafe { _svcvtlt_f64_f32_m(inactive, pg.into(), op) }
}
#[doc = "Up convert long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcvtlt_f64[_f32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(fcvtlt))]
pub fn svcvtlt_f64_f32_x(pg: svbool_t, op: svfloat32_t) -> svfloat64_t {
    unsafe { svcvtlt_f64_f32_m(simd_reinterpret(op), pg, op) }
}
#[doc = "Down convert and narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcvtnt_f32[_f64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(fcvtnt))]
pub fn svcvtnt_f32_f64_m(even: svfloat32_t, pg: svbool_t, op: svfloat64_t) -> svfloat32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.fcvtnt.f32f64")]
        fn _svcvtnt_f32_f64_m(even: svfloat32_t, pg: svbool2_t, op: svfloat64_t) -> svfloat32_t;
    }
    unsafe { _svcvtnt_f32_f64_m(even, pg.into(), op) }
}
#[doc = "Down convert and narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcvtnt_f32[_f64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(fcvtnt))]
pub fn svcvtnt_f32_f64_x(even: svfloat32_t, pg: svbool_t, op: svfloat64_t) -> svfloat32_t {
    svcvtnt_f32_f64_m(even, pg, op)
}
#[doc = "Down convert, rounding to odd"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcvtx_f32[_f64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(fcvtx))]
pub fn svcvtx_f32_f64_m(inactive: svfloat32_t, pg: svbool_t, op: svfloat64_t) -> svfloat32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.fcvtx.f32f64")]
        fn _svcvtx_f32_f64_m(inactive: svfloat32_t, pg: svbool2_t, op: svfloat64_t) -> svfloat32_t;
    }
    unsafe { _svcvtx_f32_f64_m(inactive, pg.into(), op) }
}
#[doc = "Down convert, rounding to odd"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcvtx_f32[_f64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(fcvtx))]
pub fn svcvtx_f32_f64_x(pg: svbool_t, op: svfloat64_t) -> svfloat32_t {
    unsafe { svcvtx_f32_f64_m(simd_reinterpret(op), pg, op) }
}
#[doc = "Down convert, rounding to odd"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcvtx_f32[_f64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(fcvtx))]
pub fn svcvtx_f32_f64_z(pg: svbool_t, op: svfloat64_t) -> svfloat32_t {
    svcvtx_f32_f64_m(svdup_n_f32(0.0), pg, op)
}
#[doc = "Down convert, rounding to odd (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcvtxnt_f32[_f64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(fcvtxnt))]
pub fn svcvtxnt_f32_f64_m(even: svfloat32_t, pg: svbool_t, op: svfloat64_t) -> svfloat32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.fcvtxnt.f32f64")]
        fn _svcvtxnt_f32_f64_m(even: svfloat32_t, pg: svbool2_t, op: svfloat64_t) -> svfloat32_t;
    }
    unsafe { _svcvtxnt_f32_f64_m(even, pg.into(), op) }
}
#[doc = "Down convert, rounding to odd (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svcvtxnt_f32[_f64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(fcvtxnt))]
pub fn svcvtxnt_f32_f64_x(even: svfloat32_t, pg: svbool_t, op: svfloat64_t) -> svfloat32_t {
    svcvtxnt_f32_f64_m(even, pg, op)
}
#[doc = "Bitwise exclusive OR of three vectors"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveor3[_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eor3))]
pub fn sveor3_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.eor3.nxv16i8")]
        fn _sveor3_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t;
    }
    unsafe { _sveor3_s8(op1, op2, op3) }
}
#[doc = "Bitwise exclusive OR of three vectors"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveor3[_n_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eor3))]
pub fn sveor3_n_s8(op1: svint8_t, op2: svint8_t, op3: i8) -> svint8_t {
    sveor3_s8(op1, op2, svdup_n_s8(op3))
}
#[doc = "Bitwise exclusive OR of three vectors"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveor3[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eor3))]
pub fn sveor3_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.eor3.nxv8i16")]
        fn _sveor3_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t;
    }
    unsafe { _sveor3_s16(op1, op2, op3) }
}
#[doc = "Bitwise exclusive OR of three vectors"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveor3[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eor3))]
pub fn sveor3_n_s16(op1: svint16_t, op2: svint16_t, op3: i16) -> svint16_t {
    sveor3_s16(op1, op2, svdup_n_s16(op3))
}
#[doc = "Bitwise exclusive OR of three vectors"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveor3[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eor3))]
pub fn sveor3_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.eor3.nxv4i32")]
        fn _sveor3_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _sveor3_s32(op1, op2, op3) }
}
#[doc = "Bitwise exclusive OR of three vectors"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveor3[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eor3))]
pub fn sveor3_n_s32(op1: svint32_t, op2: svint32_t, op3: i32) -> svint32_t {
    sveor3_s32(op1, op2, svdup_n_s32(op3))
}
#[doc = "Bitwise exclusive OR of three vectors"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveor3[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eor3))]
pub fn sveor3_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.eor3.nxv2i64")]
        fn _sveor3_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _sveor3_s64(op1, op2, op3) }
}
#[doc = "Bitwise exclusive OR of three vectors"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveor3[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eor3))]
pub fn sveor3_n_s64(op1: svint64_t, op2: svint64_t, op3: i64) -> svint64_t {
    sveor3_s64(op1, op2, svdup_n_s64(op3))
}
#[doc = "Bitwise exclusive OR of three vectors"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveor3[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eor3))]
pub fn sveor3_u8(op1: svuint8_t, op2: svuint8_t, op3: svuint8_t) -> svuint8_t {
    unsafe { sveor3_s8(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Bitwise exclusive OR of three vectors"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveor3[_n_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eor3))]
pub fn sveor3_n_u8(op1: svuint8_t, op2: svuint8_t, op3: u8) -> svuint8_t {
    sveor3_u8(op1, op2, svdup_n_u8(op3))
}
#[doc = "Bitwise exclusive OR of three vectors"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveor3[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eor3))]
pub fn sveor3_u16(op1: svuint16_t, op2: svuint16_t, op3: svuint16_t) -> svuint16_t {
    unsafe { sveor3_s16(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Bitwise exclusive OR of three vectors"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveor3[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eor3))]
pub fn sveor3_n_u16(op1: svuint16_t, op2: svuint16_t, op3: u16) -> svuint16_t {
    sveor3_u16(op1, op2, svdup_n_u16(op3))
}
#[doc = "Bitwise exclusive OR of three vectors"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveor3[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eor3))]
pub fn sveor3_u32(op1: svuint32_t, op2: svuint32_t, op3: svuint32_t) -> svuint32_t {
    unsafe { sveor3_s32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Bitwise exclusive OR of three vectors"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveor3[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eor3))]
pub fn sveor3_n_u32(op1: svuint32_t, op2: svuint32_t, op3: u32) -> svuint32_t {
    sveor3_u32(op1, op2, svdup_n_u32(op3))
}
#[doc = "Bitwise exclusive OR of three vectors"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveor3[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eor3))]
pub fn sveor3_u64(op1: svuint64_t, op2: svuint64_t, op3: svuint64_t) -> svuint64_t {
    unsafe { sveor3_s64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Bitwise exclusive OR of three vectors"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveor3[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eor3))]
pub fn sveor3_n_u64(op1: svuint64_t, op2: svuint64_t, op3: u64) -> svuint64_t {
    sveor3_u64(op1, op2, svdup_n_u64(op3))
}
#[doc = "Interleaving exclusive OR (bottom, top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveorbt[_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eorbt))]
pub fn sveorbt_s8(odd: svint8_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.eorbt.nxv16i8")]
        fn _sveorbt_s8(odd: svint8_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _sveorbt_s8(odd, op1, op2) }
}
#[doc = "Interleaving exclusive OR (bottom, top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveorbt[_n_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eorbt))]
pub fn sveorbt_n_s8(odd: svint8_t, op1: svint8_t, op2: i8) -> svint8_t {
    sveorbt_s8(odd, op1, svdup_n_s8(op2))
}
#[doc = "Interleaving exclusive OR (bottom, top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveorbt[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eorbt))]
pub fn sveorbt_s16(odd: svint16_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.eorbt.nxv8i16")]
        fn _sveorbt_s16(odd: svint16_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _sveorbt_s16(odd, op1, op2) }
}
#[doc = "Interleaving exclusive OR (bottom, top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveorbt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eorbt))]
pub fn sveorbt_n_s16(odd: svint16_t, op1: svint16_t, op2: i16) -> svint16_t {
    sveorbt_s16(odd, op1, svdup_n_s16(op2))
}
#[doc = "Interleaving exclusive OR (bottom, top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveorbt[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eorbt))]
pub fn sveorbt_s32(odd: svint32_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.eorbt.nxv4i32")]
        fn _sveorbt_s32(odd: svint32_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _sveorbt_s32(odd, op1, op2) }
}
#[doc = "Interleaving exclusive OR (bottom, top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveorbt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eorbt))]
pub fn sveorbt_n_s32(odd: svint32_t, op1: svint32_t, op2: i32) -> svint32_t {
    sveorbt_s32(odd, op1, svdup_n_s32(op2))
}
#[doc = "Interleaving exclusive OR (bottom, top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveorbt[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eorbt))]
pub fn sveorbt_s64(odd: svint64_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.eorbt.nxv2i64")]
        fn _sveorbt_s64(odd: svint64_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _sveorbt_s64(odd, op1, op2) }
}
#[doc = "Interleaving exclusive OR (bottom, top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveorbt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eorbt))]
pub fn sveorbt_n_s64(odd: svint64_t, op1: svint64_t, op2: i64) -> svint64_t {
    sveorbt_s64(odd, op1, svdup_n_s64(op2))
}
#[doc = "Interleaving exclusive OR (bottom, top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveorbt[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eorbt))]
pub fn sveorbt_u8(odd: svuint8_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    unsafe { sveorbt_s8(odd.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Interleaving exclusive OR (bottom, top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveorbt[_n_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eorbt))]
pub fn sveorbt_n_u8(odd: svuint8_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    sveorbt_u8(odd, op1, svdup_n_u8(op2))
}
#[doc = "Interleaving exclusive OR (bottom, top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveorbt[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eorbt))]
pub fn sveorbt_u16(odd: svuint16_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    unsafe { sveorbt_s16(odd.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Interleaving exclusive OR (bottom, top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveorbt[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eorbt))]
pub fn sveorbt_n_u16(odd: svuint16_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    sveorbt_u16(odd, op1, svdup_n_u16(op2))
}
#[doc = "Interleaving exclusive OR (bottom, top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveorbt[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eorbt))]
pub fn sveorbt_u32(odd: svuint32_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    unsafe { sveorbt_s32(odd.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Interleaving exclusive OR (bottom, top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveorbt[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eorbt))]
pub fn sveorbt_n_u32(odd: svuint32_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    sveorbt_u32(odd, op1, svdup_n_u32(op2))
}
#[doc = "Interleaving exclusive OR (bottom, top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveorbt[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eorbt))]
pub fn sveorbt_u64(odd: svuint64_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    unsafe { sveorbt_s64(odd.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Interleaving exclusive OR (bottom, top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveorbt[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eorbt))]
pub fn sveorbt_n_u64(odd: svuint64_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    sveorbt_u64(odd, op1, svdup_n_u64(op2))
}
#[doc = "Interleaving exclusive OR (top, bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveortb[_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eortb))]
pub fn sveortb_s8(even: svint8_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.eortb.nxv16i8")]
        fn _sveortb_s8(even: svint8_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _sveortb_s8(even, op1, op2) }
}
#[doc = "Interleaving exclusive OR (top, bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveortb[_n_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eortb))]
pub fn sveortb_n_s8(even: svint8_t, op1: svint8_t, op2: i8) -> svint8_t {
    sveortb_s8(even, op1, svdup_n_s8(op2))
}
#[doc = "Interleaving exclusive OR (top, bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveortb[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eortb))]
pub fn sveortb_s16(even: svint16_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.eortb.nxv8i16")]
        fn _sveortb_s16(even: svint16_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _sveortb_s16(even, op1, op2) }
}
#[doc = "Interleaving exclusive OR (top, bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveortb[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eortb))]
pub fn sveortb_n_s16(even: svint16_t, op1: svint16_t, op2: i16) -> svint16_t {
    sveortb_s16(even, op1, svdup_n_s16(op2))
}
#[doc = "Interleaving exclusive OR (top, bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveortb[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eortb))]
pub fn sveortb_s32(even: svint32_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.eortb.nxv4i32")]
        fn _sveortb_s32(even: svint32_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _sveortb_s32(even, op1, op2) }
}
#[doc = "Interleaving exclusive OR (top, bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveortb[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eortb))]
pub fn sveortb_n_s32(even: svint32_t, op1: svint32_t, op2: i32) -> svint32_t {
    sveortb_s32(even, op1, svdup_n_s32(op2))
}
#[doc = "Interleaving exclusive OR (top, bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveortb[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eortb))]
pub fn sveortb_s64(even: svint64_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.eortb.nxv2i64")]
        fn _sveortb_s64(even: svint64_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _sveortb_s64(even, op1, op2) }
}
#[doc = "Interleaving exclusive OR (top, bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveortb[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eortb))]
pub fn sveortb_n_s64(even: svint64_t, op1: svint64_t, op2: i64) -> svint64_t {
    sveortb_s64(even, op1, svdup_n_s64(op2))
}
#[doc = "Interleaving exclusive OR (top, bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveortb[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eortb))]
pub fn sveortb_u8(even: svuint8_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    unsafe { sveortb_s8(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Interleaving exclusive OR (top, bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveortb[_n_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eortb))]
pub fn sveortb_n_u8(even: svuint8_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    sveortb_u8(even, op1, svdup_n_u8(op2))
}
#[doc = "Interleaving exclusive OR (top, bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveortb[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eortb))]
pub fn sveortb_u16(even: svuint16_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    unsafe { sveortb_s16(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Interleaving exclusive OR (top, bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveortb[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eortb))]
pub fn sveortb_n_u16(even: svuint16_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    sveortb_u16(even, op1, svdup_n_u16(op2))
}
#[doc = "Interleaving exclusive OR (top, bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveortb[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eortb))]
pub fn sveortb_u32(even: svuint32_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    unsafe { sveortb_s32(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Interleaving exclusive OR (top, bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveortb[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eortb))]
pub fn sveortb_n_u32(even: svuint32_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    sveortb_u32(even, op1, svdup_n_u32(op2))
}
#[doc = "Interleaving exclusive OR (top, bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveortb[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eortb))]
pub fn sveortb_u64(even: svuint64_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    unsafe { sveortb_s64(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Interleaving exclusive OR (top, bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/sveortb[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(eortb))]
pub fn sveortb_n_u64(even: svuint64_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    sveortb_u64(even, op1, svdup_n_u64(op2))
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shadd))]
pub fn svhadd_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.shadd.nxv16i8")]
        fn _svhadd_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svhadd_s8_m(pg, op1, op2) }
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_n_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shadd))]
pub fn svhadd_n_s8_m(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svhadd_s8_m(pg, op1, svdup_n_s8(op2))
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shadd))]
pub fn svhadd_s8_x(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svhadd_s8_m(pg, op1, op2)
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_n_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shadd))]
pub fn svhadd_n_s8_x(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svhadd_s8_x(pg, op1, svdup_n_s8(op2))
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_s8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shadd))]
pub fn svhadd_s8_z(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svhadd_s8_m(pg, svsel_s8(pg, op1, svdup_n_s8(0)), op2)
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_n_s8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shadd))]
pub fn svhadd_n_s8_z(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svhadd_s8_z(pg, op1, svdup_n_s8(op2))
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shadd))]
pub fn svhadd_s16_m(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.shadd.nxv8i16")]
        fn _svhadd_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svhadd_s16_m(pg.into(), op1, op2) }
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_n_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shadd))]
pub fn svhadd_n_s16_m(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svhadd_s16_m(pg, op1, svdup_n_s16(op2))
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shadd))]
pub fn svhadd_s16_x(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svhadd_s16_m(pg, op1, op2)
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_n_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shadd))]
pub fn svhadd_n_s16_x(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svhadd_s16_x(pg, op1, svdup_n_s16(op2))
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_s16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shadd))]
pub fn svhadd_s16_z(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svhadd_s16_m(pg, svsel_s16(pg, op1, svdup_n_s16(0)), op2)
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_n_s16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shadd))]
pub fn svhadd_n_s16_z(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svhadd_s16_z(pg, op1, svdup_n_s16(op2))
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shadd))]
pub fn svhadd_s32_m(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.shadd.nxv4i32")]
        fn _svhadd_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svhadd_s32_m(pg.into(), op1, op2) }
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_n_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shadd))]
pub fn svhadd_n_s32_m(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svhadd_s32_m(pg, op1, svdup_n_s32(op2))
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shadd))]
pub fn svhadd_s32_x(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svhadd_s32_m(pg, op1, op2)
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_n_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shadd))]
pub fn svhadd_n_s32_x(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svhadd_s32_x(pg, op1, svdup_n_s32(op2))
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shadd))]
pub fn svhadd_s32_z(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svhadd_s32_m(pg, svsel_s32(pg, op1, svdup_n_s32(0)), op2)
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_n_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shadd))]
pub fn svhadd_n_s32_z(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svhadd_s32_z(pg, op1, svdup_n_s32(op2))
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shadd))]
pub fn svhadd_s64_m(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.shadd.nxv2i64")]
        fn _svhadd_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svhadd_s64_m(pg.into(), op1, op2) }
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_n_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shadd))]
pub fn svhadd_n_s64_m(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svhadd_s64_m(pg, op1, svdup_n_s64(op2))
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shadd))]
pub fn svhadd_s64_x(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svhadd_s64_m(pg, op1, op2)
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_n_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shadd))]
pub fn svhadd_n_s64_x(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svhadd_s64_x(pg, op1, svdup_n_s64(op2))
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shadd))]
pub fn svhadd_s64_z(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svhadd_s64_m(pg, svsel_s64(pg, op1, svdup_n_s64(0)), op2)
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_n_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shadd))]
pub fn svhadd_n_s64_z(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svhadd_s64_z(pg, op1, svdup_n_s64(op2))
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_u8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhadd))]
pub fn svhadd_u8_m(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uhadd.nxv16i8")]
        fn _svhadd_u8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svhadd_u8_m(pg, op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_n_u8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhadd))]
pub fn svhadd_n_u8_m(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svhadd_u8_m(pg, op1, svdup_n_u8(op2))
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_u8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhadd))]
pub fn svhadd_u8_x(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svhadd_u8_m(pg, op1, op2)
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_n_u8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhadd))]
pub fn svhadd_n_u8_x(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svhadd_u8_x(pg, op1, svdup_n_u8(op2))
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_u8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhadd))]
pub fn svhadd_u8_z(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svhadd_u8_m(pg, svsel_u8(pg, op1, svdup_n_u8(0)), op2)
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_n_u8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhadd))]
pub fn svhadd_n_u8_z(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svhadd_u8_z(pg, op1, svdup_n_u8(op2))
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_u16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhadd))]
pub fn svhadd_u16_m(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uhadd.nxv8i16")]
        fn _svhadd_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svhadd_u16_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_n_u16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhadd))]
pub fn svhadd_n_u16_m(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svhadd_u16_m(pg, op1, svdup_n_u16(op2))
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_u16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhadd))]
pub fn svhadd_u16_x(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svhadd_u16_m(pg, op1, op2)
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_n_u16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhadd))]
pub fn svhadd_n_u16_x(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svhadd_u16_x(pg, op1, svdup_n_u16(op2))
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_u16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhadd))]
pub fn svhadd_u16_z(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svhadd_u16_m(pg, svsel_u16(pg, op1, svdup_n_u16(0)), op2)
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_n_u16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhadd))]
pub fn svhadd_n_u16_z(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svhadd_u16_z(pg, op1, svdup_n_u16(op2))
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhadd))]
pub fn svhadd_u32_m(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uhadd.nxv4i32")]
        fn _svhadd_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svhadd_u32_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_n_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhadd))]
pub fn svhadd_n_u32_m(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svhadd_u32_m(pg, op1, svdup_n_u32(op2))
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhadd))]
pub fn svhadd_u32_x(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svhadd_u32_m(pg, op1, op2)
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_n_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhadd))]
pub fn svhadd_n_u32_x(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svhadd_u32_x(pg, op1, svdup_n_u32(op2))
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_u32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhadd))]
pub fn svhadd_u32_z(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svhadd_u32_m(pg, svsel_u32(pg, op1, svdup_n_u32(0)), op2)
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_n_u32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhadd))]
pub fn svhadd_n_u32_z(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svhadd_u32_z(pg, op1, svdup_n_u32(op2))
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_u64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhadd))]
pub fn svhadd_u64_m(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uhadd.nxv2i64")]
        fn _svhadd_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svhadd_u64_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_n_u64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhadd))]
pub fn svhadd_n_u64_m(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svhadd_u64_m(pg, op1, svdup_n_u64(op2))
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_u64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhadd))]
pub fn svhadd_u64_x(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svhadd_u64_m(pg, op1, op2)
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_n_u64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhadd))]
pub fn svhadd_n_u64_x(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svhadd_u64_x(pg, op1, svdup_n_u64(op2))
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_u64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhadd))]
pub fn svhadd_u64_z(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svhadd_u64_m(pg, svsel_u64(pg, op1, svdup_n_u64(0)), op2)
}
#[doc = "Halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhadd[_n_u64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhadd))]
pub fn svhadd_n_u64_z(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svhadd_u64_z(pg, op1, svdup_n_u64(op2))
}
#[doc = "Count matching elements"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhistcnt[_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(histcnt))]
pub fn svhistcnt_s32_z(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.histcnt.nxv4i32"
        )]
        fn _svhistcnt_s32_z(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svhistcnt_s32_z(pg.into(), op1, op2).as_unsigned() }
}
#[doc = "Count matching elements"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhistcnt[_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(histcnt))]
pub fn svhistcnt_s64_z(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.histcnt.nxv2i64"
        )]
        fn _svhistcnt_s64_z(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svhistcnt_s64_z(pg.into(), op1, op2).as_unsigned() }
}
#[doc = "Count matching elements"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhistcnt[_u32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(histcnt))]
pub fn svhistcnt_u32_z(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    unsafe { svhistcnt_s32_z(pg, op1.as_signed(), op2.as_signed()) }
}
#[doc = "Count matching elements"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhistcnt[_u64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(histcnt))]
pub fn svhistcnt_u64_z(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    unsafe { svhistcnt_s64_z(pg, op1.as_signed(), op2.as_signed()) }
}
#[doc = "Count matching elements in 128-bit segments"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhistseg[_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(histseg))]
pub fn svhistseg_s8(op1: svint8_t, op2: svint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.histseg.nxv16i8"
        )]
        fn _svhistseg_s8(op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svhistseg_s8(op1, op2).as_unsigned() }
}
#[doc = "Count matching elements in 128-bit segments"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhistseg[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(histseg))]
pub fn svhistseg_u8(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    unsafe { svhistseg_s8(op1.as_signed(), op2.as_signed()) }
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsub_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.shsub.nxv16i8")]
        fn _svhsub_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svhsub_s8_m(pg, op1, op2) }
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_n_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsub_n_s8_m(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svhsub_s8_m(pg, op1, svdup_n_s8(op2))
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsub_s8_x(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svhsub_s8_m(pg, op1, op2)
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_n_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsub_n_s8_x(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svhsub_s8_x(pg, op1, svdup_n_s8(op2))
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_s8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsub_s8_z(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svhsub_s8_m(pg, svsel_s8(pg, op1, svdup_n_s8(0)), op2)
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_n_s8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsub_n_s8_z(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svhsub_s8_z(pg, op1, svdup_n_s8(op2))
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsub_s16_m(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.shsub.nxv8i16")]
        fn _svhsub_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svhsub_s16_m(pg.into(), op1, op2) }
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_n_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsub_n_s16_m(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svhsub_s16_m(pg, op1, svdup_n_s16(op2))
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsub_s16_x(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svhsub_s16_m(pg, op1, op2)
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_n_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsub_n_s16_x(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svhsub_s16_x(pg, op1, svdup_n_s16(op2))
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_s16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsub_s16_z(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svhsub_s16_m(pg, svsel_s16(pg, op1, svdup_n_s16(0)), op2)
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_n_s16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsub_n_s16_z(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svhsub_s16_z(pg, op1, svdup_n_s16(op2))
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsub_s32_m(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.shsub.nxv4i32")]
        fn _svhsub_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svhsub_s32_m(pg.into(), op1, op2) }
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_n_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsub_n_s32_m(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svhsub_s32_m(pg, op1, svdup_n_s32(op2))
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsub_s32_x(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svhsub_s32_m(pg, op1, op2)
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_n_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsub_n_s32_x(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svhsub_s32_x(pg, op1, svdup_n_s32(op2))
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsub_s32_z(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svhsub_s32_m(pg, svsel_s32(pg, op1, svdup_n_s32(0)), op2)
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_n_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsub_n_s32_z(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svhsub_s32_z(pg, op1, svdup_n_s32(op2))
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsub_s64_m(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.shsub.nxv2i64")]
        fn _svhsub_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svhsub_s64_m(pg.into(), op1, op2) }
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_n_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsub_n_s64_m(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svhsub_s64_m(pg, op1, svdup_n_s64(op2))
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsub_s64_x(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svhsub_s64_m(pg, op1, op2)
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_n_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsub_n_s64_x(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svhsub_s64_x(pg, op1, svdup_n_s64(op2))
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsub_s64_z(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svhsub_s64_m(pg, svsel_s64(pg, op1, svdup_n_s64(0)), op2)
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_n_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsub_n_s64_z(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svhsub_s64_z(pg, op1, svdup_n_s64(op2))
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_u8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsub_u8_m(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uhsub.nxv16i8")]
        fn _svhsub_u8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svhsub_u8_m(pg, op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_n_u8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsub_n_u8_m(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svhsub_u8_m(pg, op1, svdup_n_u8(op2))
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_u8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsub_u8_x(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svhsub_u8_m(pg, op1, op2)
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_n_u8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsub_n_u8_x(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svhsub_u8_x(pg, op1, svdup_n_u8(op2))
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_u8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsub_u8_z(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svhsub_u8_m(pg, svsel_u8(pg, op1, svdup_n_u8(0)), op2)
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_n_u8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsub_n_u8_z(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svhsub_u8_z(pg, op1, svdup_n_u8(op2))
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_u16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsub_u16_m(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uhsub.nxv8i16")]
        fn _svhsub_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svhsub_u16_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_n_u16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsub_n_u16_m(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svhsub_u16_m(pg, op1, svdup_n_u16(op2))
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_u16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsub_u16_x(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svhsub_u16_m(pg, op1, op2)
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_n_u16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsub_n_u16_x(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svhsub_u16_x(pg, op1, svdup_n_u16(op2))
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_u16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsub_u16_z(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svhsub_u16_m(pg, svsel_u16(pg, op1, svdup_n_u16(0)), op2)
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_n_u16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsub_n_u16_z(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svhsub_u16_z(pg, op1, svdup_n_u16(op2))
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsub_u32_m(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uhsub.nxv4i32")]
        fn _svhsub_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svhsub_u32_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_n_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsub_n_u32_m(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svhsub_u32_m(pg, op1, svdup_n_u32(op2))
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsub_u32_x(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svhsub_u32_m(pg, op1, op2)
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_n_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsub_n_u32_x(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svhsub_u32_x(pg, op1, svdup_n_u32(op2))
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_u32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsub_u32_z(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svhsub_u32_m(pg, svsel_u32(pg, op1, svdup_n_u32(0)), op2)
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_n_u32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsub_n_u32_z(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svhsub_u32_z(pg, op1, svdup_n_u32(op2))
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_u64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsub_u64_m(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uhsub.nxv2i64")]
        fn _svhsub_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svhsub_u64_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_n_u64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsub_n_u64_m(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svhsub_u64_m(pg, op1, svdup_n_u64(op2))
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_u64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsub_u64_x(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svhsub_u64_m(pg, op1, op2)
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_n_u64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsub_n_u64_x(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svhsub_u64_x(pg, op1, svdup_n_u64(op2))
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_u64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsub_u64_z(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svhsub_u64_m(pg, svsel_u64(pg, op1, svdup_n_u64(0)), op2)
}
#[doc = "Halving subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsub[_n_u64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsub_n_u64_z(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svhsub_u64_z(pg, op1, svdup_n_u64(op2))
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsubr_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.shsubr.nxv16i8")]
        fn _svhsubr_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svhsubr_s8_m(pg, op1, op2) }
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_n_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsubr_n_s8_m(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svhsubr_s8_m(pg, op1, svdup_n_s8(op2))
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsubr_s8_x(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svhsubr_s8_m(pg, op1, op2)
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_n_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsubr_n_s8_x(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svhsubr_s8_x(pg, op1, svdup_n_s8(op2))
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_s8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsubr_s8_z(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svhsubr_s8_m(pg, svsel_s8(pg, op1, svdup_n_s8(0)), op2)
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_n_s8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsubr_n_s8_z(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svhsubr_s8_z(pg, op1, svdup_n_s8(op2))
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsubr_s16_m(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.shsubr.nxv8i16")]
        fn _svhsubr_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svhsubr_s16_m(pg.into(), op1, op2) }
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_n_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsubr_n_s16_m(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svhsubr_s16_m(pg, op1, svdup_n_s16(op2))
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsubr_s16_x(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svhsubr_s16_m(pg, op1, op2)
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_n_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsubr_n_s16_x(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svhsubr_s16_x(pg, op1, svdup_n_s16(op2))
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_s16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsubr_s16_z(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svhsubr_s16_m(pg, svsel_s16(pg, op1, svdup_n_s16(0)), op2)
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_n_s16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsubr_n_s16_z(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svhsubr_s16_z(pg, op1, svdup_n_s16(op2))
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsubr_s32_m(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.shsubr.nxv4i32")]
        fn _svhsubr_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svhsubr_s32_m(pg.into(), op1, op2) }
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_n_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsubr_n_s32_m(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svhsubr_s32_m(pg, op1, svdup_n_s32(op2))
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsubr_s32_x(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svhsubr_s32_m(pg, op1, op2)
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_n_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsubr_n_s32_x(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svhsubr_s32_x(pg, op1, svdup_n_s32(op2))
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsubr_s32_z(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svhsubr_s32_m(pg, svsel_s32(pg, op1, svdup_n_s32(0)), op2)
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_n_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsubr_n_s32_z(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svhsubr_s32_z(pg, op1, svdup_n_s32(op2))
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsubr_s64_m(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.shsubr.nxv2i64")]
        fn _svhsubr_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svhsubr_s64_m(pg.into(), op1, op2) }
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_n_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsubr_n_s64_m(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svhsubr_s64_m(pg, op1, svdup_n_s64(op2))
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsubr_s64_x(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svhsubr_s64_m(pg, op1, op2)
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_n_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsubr_n_s64_x(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svhsubr_s64_x(pg, op1, svdup_n_s64(op2))
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsubr_s64_z(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svhsubr_s64_m(pg, svsel_s64(pg, op1, svdup_n_s64(0)), op2)
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_n_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shsub))]
pub fn svhsubr_n_s64_z(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svhsubr_s64_z(pg, op1, svdup_n_s64(op2))
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_u8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsubr_u8_m(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uhsubr.nxv16i8")]
        fn _svhsubr_u8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svhsubr_u8_m(pg, op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_n_u8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsubr_n_u8_m(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svhsubr_u8_m(pg, op1, svdup_n_u8(op2))
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_u8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsubr_u8_x(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svhsubr_u8_m(pg, op1, op2)
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_n_u8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsubr_n_u8_x(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svhsubr_u8_x(pg, op1, svdup_n_u8(op2))
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_u8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsubr_u8_z(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svhsubr_u8_m(pg, svsel_u8(pg, op1, svdup_n_u8(0)), op2)
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_n_u8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsubr_n_u8_z(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svhsubr_u8_z(pg, op1, svdup_n_u8(op2))
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_u16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsubr_u16_m(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uhsubr.nxv8i16")]
        fn _svhsubr_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svhsubr_u16_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_n_u16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsubr_n_u16_m(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svhsubr_u16_m(pg, op1, svdup_n_u16(op2))
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_u16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsubr_u16_x(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svhsubr_u16_m(pg, op1, op2)
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_n_u16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsubr_n_u16_x(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svhsubr_u16_x(pg, op1, svdup_n_u16(op2))
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_u16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsubr_u16_z(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svhsubr_u16_m(pg, svsel_u16(pg, op1, svdup_n_u16(0)), op2)
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_n_u16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsubr_n_u16_z(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svhsubr_u16_z(pg, op1, svdup_n_u16(op2))
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsubr_u32_m(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uhsubr.nxv4i32")]
        fn _svhsubr_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svhsubr_u32_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_n_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsubr_n_u32_m(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svhsubr_u32_m(pg, op1, svdup_n_u32(op2))
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsubr_u32_x(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svhsubr_u32_m(pg, op1, op2)
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_n_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsubr_n_u32_x(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svhsubr_u32_x(pg, op1, svdup_n_u32(op2))
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_u32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsubr_u32_z(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svhsubr_u32_m(pg, svsel_u32(pg, op1, svdup_n_u32(0)), op2)
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_n_u32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsubr_n_u32_z(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svhsubr_u32_z(pg, op1, svdup_n_u32(op2))
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_u64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsubr_u64_m(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uhsubr.nxv2i64")]
        fn _svhsubr_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svhsubr_u64_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_n_u64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsubr_n_u64_m(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svhsubr_u64_m(pg, op1, svdup_n_u64(op2))
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_u64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsubr_u64_x(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svhsubr_u64_m(pg, op1, op2)
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_n_u64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsubr_n_u64_x(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svhsubr_u64_x(pg, op1, svdup_n_u64(op2))
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_u64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsubr_u64_z(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svhsubr_u64_m(pg, svsel_u64(pg, op1, svdup_n_u64(0)), op2)
}
#[doc = "Halving subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svhsubr[_n_u64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uhsub))]
pub fn svhsubr_n_u64_z(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svhsubr_u64_z(pg, op1, svdup_n_u64(op2))
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather_[s64]index[_f64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_s64index_f64(
    pg: svbool_t,
    base: *const f64,
    indices: svint64_t,
) -> svfloat64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.index.nxv2f64"
        )]
        fn _svldnt1_gather_s64index_f64(
            pg: svbool2_t,
            base: *const f64,
            indices: svint64_t,
        ) -> svfloat64_t;
    }
    _svldnt1_gather_s64index_f64(pg.into(), base, indices)
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather_[s64]index[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_s64index_s64(
    pg: svbool_t,
    base: *const i64,
    indices: svint64_t,
) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.index.nxv2i64"
        )]
        fn _svldnt1_gather_s64index_s64(
            pg: svbool2_t,
            base: *const i64,
            indices: svint64_t,
        ) -> svint64_t;
    }
    _svldnt1_gather_s64index_s64(pg.into(), base, indices)
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather_[s64]index[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_s64index_u64(
    pg: svbool_t,
    base: *const u64,
    indices: svint64_t,
) -> svuint64_t {
    svldnt1_gather_s64index_s64(pg, base.as_signed(), indices).as_unsigned()
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather_[u64]index[_f64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64index_f64(
    pg: svbool_t,
    base: *const f64,
    indices: svuint64_t,
) -> svfloat64_t {
    svldnt1_gather_s64index_f64(pg, base, indices.as_signed())
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather_[u64]index[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64index_s64(
    pg: svbool_t,
    base: *const i64,
    indices: svuint64_t,
) -> svint64_t {
    svldnt1_gather_s64index_s64(pg, base, indices.as_signed())
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather_[u64]index[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64index_u64(
    pg: svbool_t,
    base: *const u64,
    indices: svuint64_t,
) -> svuint64_t {
    svldnt1_gather_s64index_s64(pg, base.as_signed(), indices.as_signed()).as_unsigned()
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather_[s64]offset[_f64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_s64offset_f64(
    pg: svbool_t,
    base: *const f64,
    offsets: svint64_t,
) -> svfloat64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.nxv2f64"
        )]
        fn _svldnt1_gather_s64offset_f64(
            pg: svbool2_t,
            base: *const f64,
            offsets: svint64_t,
        ) -> svfloat64_t;
    }
    _svldnt1_gather_s64offset_f64(pg.into(), base, offsets)
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather_[s64]offset[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_s64offset_s64(
    pg: svbool_t,
    base: *const i64,
    offsets: svint64_t,
) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.nxv2i64"
        )]
        fn _svldnt1_gather_s64offset_s64(
            pg: svbool2_t,
            base: *const i64,
            offsets: svint64_t,
        ) -> svint64_t;
    }
    _svldnt1_gather_s64offset_s64(pg.into(), base, offsets)
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather_[s64]offset[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_s64offset_u64(
    pg: svbool_t,
    base: *const u64,
    offsets: svint64_t,
) -> svuint64_t {
    svldnt1_gather_s64offset_s64(pg, base.as_signed(), offsets).as_unsigned()
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather_[u32]offset[_f32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1_gather_u32offset_f32(
    pg: svbool_t,
    base: *const f32,
    offsets: svuint32_t,
) -> svfloat32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.uxtw.nxv4f32"
        )]
        fn _svldnt1_gather_u32offset_f32(
            pg: svbool4_t,
            base: *const f32,
            offsets: svint32_t,
        ) -> svfloat32_t;
    }
    _svldnt1_gather_u32offset_f32(pg.into(), base, offsets.as_signed())
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather_[u32]offset[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1_gather_u32offset_s32(
    pg: svbool_t,
    base: *const i32,
    offsets: svuint32_t,
) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.uxtw.nxv4i32"
        )]
        fn _svldnt1_gather_u32offset_s32(
            pg: svbool4_t,
            base: *const i32,
            offsets: svint32_t,
        ) -> svint32_t;
    }
    _svldnt1_gather_u32offset_s32(pg.into(), base, offsets.as_signed())
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather_[u32]offset[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1_gather_u32offset_u32(
    pg: svbool_t,
    base: *const u32,
    offsets: svuint32_t,
) -> svuint32_t {
    svldnt1_gather_u32offset_s32(pg, base.as_signed(), offsets).as_unsigned()
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather_[u64]offset[_f64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64offset_f64(
    pg: svbool_t,
    base: *const f64,
    offsets: svuint64_t,
) -> svfloat64_t {
    svldnt1_gather_s64offset_f64(pg, base, offsets.as_signed())
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather_[u64]offset[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64offset_s64(
    pg: svbool_t,
    base: *const i64,
    offsets: svuint64_t,
) -> svint64_t {
    svldnt1_gather_s64offset_s64(pg, base, offsets.as_signed())
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather_[u64]offset[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64offset_u64(
    pg: svbool_t,
    base: *const u64,
    offsets: svuint64_t,
) -> svuint64_t {
    svldnt1_gather_s64offset_s64(pg, base.as_signed(), offsets.as_signed()).as_unsigned()
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather[_u32base]_f32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1_gather_u32base_f32(pg: svbool_t, bases: svuint32_t) -> svfloat32_t {
    svldnt1_gather_u32base_offset_f32(pg, bases, 0)
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather[_u32base]_s32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1_gather_u32base_s32(pg: svbool_t, bases: svuint32_t) -> svint32_t {
    svldnt1_gather_u32base_offset_s32(pg, bases, 0)
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather[_u32base]_u32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1_gather_u32base_u32(pg: svbool_t, bases: svuint32_t) -> svuint32_t {
    svldnt1_gather_u32base_offset_u32(pg, bases, 0)
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather[_u64base]_f64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64base_f64(pg: svbool_t, bases: svuint64_t) -> svfloat64_t {
    svldnt1_gather_u64base_offset_f64(pg, bases, 0)
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather[_u64base]_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64base_s64(pg: svbool_t, bases: svuint64_t) -> svint64_t {
    svldnt1_gather_u64base_offset_s64(pg, bases, 0)
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather[_u64base]_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64base_u64(pg: svbool_t, bases: svuint64_t) -> svuint64_t {
    svldnt1_gather_u64base_offset_u64(pg, bases, 0)
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather[_u32base]_index_f32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1_gather_u32base_index_f32(
    pg: svbool_t,
    bases: svuint32_t,
    index: i64,
) -> svfloat32_t {
    svldnt1_gather_u32base_offset_f32(pg, bases, index.unchecked_shl(2))
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather[_u32base]_index_s32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1_gather_u32base_index_s32(
    pg: svbool_t,
    bases: svuint32_t,
    index: i64,
) -> svint32_t {
    svldnt1_gather_u32base_offset_s32(pg, bases, index.unchecked_shl(2))
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather[_u32base]_index_u32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1_gather_u32base_index_u32(
    pg: svbool_t,
    bases: svuint32_t,
    index: i64,
) -> svuint32_t {
    svldnt1_gather_u32base_offset_u32(pg, bases, index.unchecked_shl(2))
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather[_u64base]_index_f64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64base_index_f64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
) -> svfloat64_t {
    svldnt1_gather_u64base_offset_f64(pg, bases, index.unchecked_shl(3))
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather[_u64base]_index_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64base_index_s64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
) -> svint64_t {
    svldnt1_gather_u64base_offset_s64(pg, bases, index.unchecked_shl(3))
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather[_u64base]_index_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64base_index_u64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
) -> svuint64_t {
    svldnt1_gather_u64base_offset_u64(pg, bases, index.unchecked_shl(3))
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather[_u32base]_offset_f32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1_gather_u32base_offset_f32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
) -> svfloat32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.scalar.offset.nxv4f32.nxv4i32"
        )]
        fn _svldnt1_gather_u32base_offset_f32(
            pg: svbool4_t,
            bases: svint32_t,
            offset: i64,
        ) -> svfloat32_t;
    }
    _svldnt1_gather_u32base_offset_f32(pg.into(), bases.as_signed(), offset)
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather[_u32base]_offset_s32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1_gather_u32base_offset_s32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.scalar.offset.nxv4i32.nxv4i32"
        )]
        fn _svldnt1_gather_u32base_offset_s32(
            pg: svbool4_t,
            bases: svint32_t,
            offset: i64,
        ) -> svint32_t;
    }
    _svldnt1_gather_u32base_offset_s32(pg.into(), bases.as_signed(), offset)
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather[_u32base]_offset_u32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1_gather_u32base_offset_u32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
) -> svuint32_t {
    svldnt1_gather_u32base_offset_s32(pg, bases, offset).as_unsigned()
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather[_u64base]_offset_f64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64base_offset_f64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svfloat64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.scalar.offset.nxv2f64.nxv2i64"
        )]
        fn _svldnt1_gather_u64base_offset_f64(
            pg: svbool2_t,
            bases: svint64_t,
            offset: i64,
        ) -> svfloat64_t;
    }
    _svldnt1_gather_u64base_offset_f64(pg.into(), bases.as_signed(), offset)
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather[_u64base]_offset_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64base_offset_s64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.scalar.offset.nxv2i64.nxv2i64"
        )]
        fn _svldnt1_gather_u64base_offset_s64(
            pg: svbool2_t,
            bases: svint64_t,
            offset: i64,
        ) -> svint64_t;
    }
    _svldnt1_gather_u64base_offset_s64(pg.into(), bases.as_signed(), offset)
}
#[doc = "Unextended load, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1_gather[_u64base]_offset_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64base_offset_u64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svuint64_t {
    svldnt1_gather_u64base_offset_s64(pg, bases, offset).as_unsigned()
}
#[doc = "Load 8-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sb_gather_[s64]offset_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_s64offset_s64(
    pg: svbool_t,
    base: *const i8,
    offsets: svint64_t,
) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.nxv2i8"
        )]
        fn _svldnt1sb_gather_s64offset_s64(
            pg: svbool2_t,
            base: *const i8,
            offsets: svint64_t,
        ) -> nxv2i8;
    }
    simd_cast(_svldnt1sb_gather_s64offset_s64(pg.into(), base, offsets))
}
#[doc = "Load 16-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sh_gather_[s64]offset_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_s64offset_s64(
    pg: svbool_t,
    base: *const i16,
    offsets: svint64_t,
) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.nxv2i16"
        )]
        fn _svldnt1sh_gather_s64offset_s64(
            pg: svbool2_t,
            base: *const i16,
            offsets: svint64_t,
        ) -> nxv2i16;
    }
    simd_cast(_svldnt1sh_gather_s64offset_s64(pg.into(), base, offsets))
}
#[doc = "Load 32-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sw_gather_[s64]offset_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_s64offset_s64(
    pg: svbool_t,
    base: *const i32,
    offsets: svint64_t,
) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.nxv2i32"
        )]
        fn _svldnt1sw_gather_s64offset_s64(
            pg: svbool2_t,
            base: *const i32,
            offsets: svint64_t,
        ) -> nxv2i32;
    }
    simd_cast(_svldnt1sw_gather_s64offset_s64(pg.into(), base, offsets))
}
#[doc = "Load 8-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sb_gather_[s64]offset_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_s64offset_u64(
    pg: svbool_t,
    base: *const i8,
    offsets: svint64_t,
) -> svuint64_t {
    svldnt1sb_gather_s64offset_s64(pg, base, offsets).as_unsigned()
}
#[doc = "Load 16-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sh_gather_[s64]offset_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_s64offset_u64(
    pg: svbool_t,
    base: *const i16,
    offsets: svint64_t,
) -> svuint64_t {
    svldnt1sh_gather_s64offset_s64(pg, base, offsets).as_unsigned()
}
#[doc = "Load 32-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sw_gather_[s64]offset_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_s64offset_u64(
    pg: svbool_t,
    base: *const i32,
    offsets: svint64_t,
) -> svuint64_t {
    svldnt1sw_gather_s64offset_s64(pg, base, offsets).as_unsigned()
}
#[doc = "Load 8-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sb_gather_[u32]offset_s32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_u32offset_s32(
    pg: svbool_t,
    base: *const i8,
    offsets: svuint32_t,
) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.uxtw.nxv4i8"
        )]
        fn _svldnt1sb_gather_u32offset_s32(
            pg: svbool4_t,
            base: *const i8,
            offsets: svint32_t,
        ) -> nxv4i8;
    }
    simd_cast(_svldnt1sb_gather_u32offset_s32(
        pg.into(),
        base,
        offsets.as_signed(),
    ))
}
#[doc = "Load 16-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sh_gather_[u32]offset_s32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u32offset_s32(
    pg: svbool_t,
    base: *const i16,
    offsets: svuint32_t,
) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.uxtw.nxv4i16"
        )]
        fn _svldnt1sh_gather_u32offset_s32(
            pg: svbool4_t,
            base: *const i16,
            offsets: svint32_t,
        ) -> nxv4i16;
    }
    simd_cast(_svldnt1sh_gather_u32offset_s32(
        pg.into(),
        base,
        offsets.as_signed(),
    ))
}
#[doc = "Load 8-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sb_gather_[u32]offset_u32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_u32offset_u32(
    pg: svbool_t,
    base: *const i8,
    offsets: svuint32_t,
) -> svuint32_t {
    svldnt1sb_gather_u32offset_s32(pg, base, offsets).as_unsigned()
}
#[doc = "Load 16-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sh_gather_[u32]offset_u32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u32offset_u32(
    pg: svbool_t,
    base: *const i16,
    offsets: svuint32_t,
) -> svuint32_t {
    svldnt1sh_gather_u32offset_s32(pg, base, offsets).as_unsigned()
}
#[doc = "Load 8-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sb_gather_[u64]offset_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_u64offset_s64(
    pg: svbool_t,
    base: *const i8,
    offsets: svuint64_t,
) -> svint64_t {
    svldnt1sb_gather_s64offset_s64(pg, base, offsets.as_signed())
}
#[doc = "Load 16-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sh_gather_[u64]offset_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u64offset_s64(
    pg: svbool_t,
    base: *const i16,
    offsets: svuint64_t,
) -> svint64_t {
    svldnt1sh_gather_s64offset_s64(pg, base, offsets.as_signed())
}
#[doc = "Load 32-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sw_gather_[u64]offset_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_u64offset_s64(
    pg: svbool_t,
    base: *const i32,
    offsets: svuint64_t,
) -> svint64_t {
    svldnt1sw_gather_s64offset_s64(pg, base, offsets.as_signed())
}
#[doc = "Load 8-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sb_gather_[u64]offset_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_u64offset_u64(
    pg: svbool_t,
    base: *const i8,
    offsets: svuint64_t,
) -> svuint64_t {
    svldnt1sb_gather_s64offset_s64(pg, base, offsets.as_signed()).as_unsigned()
}
#[doc = "Load 16-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sh_gather_[u64]offset_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u64offset_u64(
    pg: svbool_t,
    base: *const i16,
    offsets: svuint64_t,
) -> svuint64_t {
    svldnt1sh_gather_s64offset_s64(pg, base, offsets.as_signed()).as_unsigned()
}
#[doc = "Load 32-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sw_gather_[u64]offset_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_u64offset_u64(
    pg: svbool_t,
    base: *const i32,
    offsets: svuint64_t,
) -> svuint64_t {
    svldnt1sw_gather_s64offset_s64(pg, base, offsets.as_signed()).as_unsigned()
}
#[doc = "Load 8-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sb_gather[_u32base]_offset_s32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_u32base_offset_s32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.scalar.offset.nxv4i8.nxv4i32"
        )]
        fn _svldnt1sb_gather_u32base_offset_s32(
            pg: svbool4_t,
            bases: svint32_t,
            offset: i64,
        ) -> nxv4i8;
    }
    simd_cast(_svldnt1sb_gather_u32base_offset_s32(
        pg.into(),
        bases.as_signed(),
        offset,
    ))
}
#[doc = "Load 16-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sh_gather[_u32base]_offset_s32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u32base_offset_s32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.scalar.offset.nxv4i16.nxv4i32"
        )]
        fn _svldnt1sh_gather_u32base_offset_s32(
            pg: svbool4_t,
            bases: svint32_t,
            offset: i64,
        ) -> nxv4i16;
    }
    simd_cast(_svldnt1sh_gather_u32base_offset_s32(
        pg.into(),
        bases.as_signed(),
        offset,
    ))
}
#[doc = "Load 8-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sb_gather[_u32base]_offset_u32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_u32base_offset_u32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
) -> svuint32_t {
    svldnt1sb_gather_u32base_offset_s32(pg, bases, offset).as_unsigned()
}
#[doc = "Load 16-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sh_gather[_u32base]_offset_u32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u32base_offset_u32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
) -> svuint32_t {
    svldnt1sh_gather_u32base_offset_s32(pg, bases, offset).as_unsigned()
}
#[doc = "Load 8-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sb_gather[_u64base]_offset_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_u64base_offset_s64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.scalar.offset.nxv2i8.nxv2i64"
        )]
        fn _svldnt1sb_gather_u64base_offset_s64(
            pg: svbool2_t,
            bases: svint64_t,
            offset: i64,
        ) -> nxv2i8;
    }
    simd_cast(_svldnt1sb_gather_u64base_offset_s64(
        pg.into(),
        bases.as_signed(),
        offset,
    ))
}
#[doc = "Load 16-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sh_gather[_u64base]_offset_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u64base_offset_s64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.scalar.offset.nxv2i16.nxv2i64"
        )]
        fn _svldnt1sh_gather_u64base_offset_s64(
            pg: svbool2_t,
            bases: svint64_t,
            offset: i64,
        ) -> nxv2i16;
    }
    simd_cast(_svldnt1sh_gather_u64base_offset_s64(
        pg.into(),
        bases.as_signed(),
        offset,
    ))
}
#[doc = "Load 32-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sw_gather[_u64base]_offset_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_u64base_offset_s64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.scalar.offset.nxv2i32.nxv2i64"
        )]
        fn _svldnt1sw_gather_u64base_offset_s64(
            pg: svbool2_t,
            bases: svint64_t,
            offset: i64,
        ) -> nxv2i32;
    }
    simd_cast(_svldnt1sw_gather_u64base_offset_s64(
        pg.into(),
        bases.as_signed(),
        offset,
    ))
}
#[doc = "Load 8-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sb_gather[_u64base]_offset_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_u64base_offset_u64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svuint64_t {
    svldnt1sb_gather_u64base_offset_s64(pg, bases, offset).as_unsigned()
}
#[doc = "Load 16-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sh_gather[_u64base]_offset_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u64base_offset_u64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svuint64_t {
    svldnt1sh_gather_u64base_offset_s64(pg, bases, offset).as_unsigned()
}
#[doc = "Load 32-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sw_gather[_u64base]_offset_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_u64base_offset_u64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svuint64_t {
    svldnt1sw_gather_u64base_offset_s64(pg, bases, offset).as_unsigned()
}
#[doc = "Load 8-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sb_gather[_u32base]_s32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_u32base_s32(pg: svbool_t, bases: svuint32_t) -> svint32_t {
    svldnt1sb_gather_u32base_offset_s32(pg, bases, 0)
}
#[doc = "Load 16-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sh_gather[_u32base]_s32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u32base_s32(pg: svbool_t, bases: svuint32_t) -> svint32_t {
    svldnt1sh_gather_u32base_offset_s32(pg, bases, 0)
}
#[doc = "Load 8-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sb_gather[_u32base]_u32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_u32base_u32(pg: svbool_t, bases: svuint32_t) -> svuint32_t {
    svldnt1sb_gather_u32base_offset_u32(pg, bases, 0)
}
#[doc = "Load 16-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sh_gather[_u32base]_u32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u32base_u32(pg: svbool_t, bases: svuint32_t) -> svuint32_t {
    svldnt1sh_gather_u32base_offset_u32(pg, bases, 0)
}
#[doc = "Load 8-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sb_gather[_u64base]_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_u64base_s64(pg: svbool_t, bases: svuint64_t) -> svint64_t {
    svldnt1sb_gather_u64base_offset_s64(pg, bases, 0)
}
#[doc = "Load 16-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sh_gather[_u64base]_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u64base_s64(pg: svbool_t, bases: svuint64_t) -> svint64_t {
    svldnt1sh_gather_u64base_offset_s64(pg, bases, 0)
}
#[doc = "Load 32-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sw_gather[_u64base]_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_u64base_s64(pg: svbool_t, bases: svuint64_t) -> svint64_t {
    svldnt1sw_gather_u64base_offset_s64(pg, bases, 0)
}
#[doc = "Load 8-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sb_gather[_u64base]_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_u64base_u64(pg: svbool_t, bases: svuint64_t) -> svuint64_t {
    svldnt1sb_gather_u64base_offset_u64(pg, bases, 0)
}
#[doc = "Load 16-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sh_gather[_u64base]_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u64base_u64(pg: svbool_t, bases: svuint64_t) -> svuint64_t {
    svldnt1sh_gather_u64base_offset_u64(pg, bases, 0)
}
#[doc = "Load 32-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sw_gather[_u64base]_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_u64base_u64(pg: svbool_t, bases: svuint64_t) -> svuint64_t {
    svldnt1sw_gather_u64base_offset_u64(pg, bases, 0)
}
#[doc = "Load 16-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sh_gather_[s64]index_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_s64index_s64(
    pg: svbool_t,
    base: *const i16,
    indices: svint64_t,
) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.index.nxv2i16"
        )]
        fn _svldnt1sh_gather_s64index_s64(
            pg: svbool2_t,
            base: *const i16,
            indices: svint64_t,
        ) -> nxv2i16;
    }
    simd_cast(_svldnt1sh_gather_s64index_s64(pg.into(), base, indices))
}
#[doc = "Load 32-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sw_gather_[s64]index_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_s64index_s64(
    pg: svbool_t,
    base: *const i32,
    indices: svint64_t,
) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.index.nxv2i32"
        )]
        fn _svldnt1sw_gather_s64index_s64(
            pg: svbool2_t,
            base: *const i32,
            indices: svint64_t,
        ) -> nxv2i32;
    }
    simd_cast(_svldnt1sw_gather_s64index_s64(pg.into(), base, indices))
}
#[doc = "Load 16-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sh_gather_[s64]index_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_s64index_u64(
    pg: svbool_t,
    base: *const i16,
    indices: svint64_t,
) -> svuint64_t {
    svldnt1sh_gather_s64index_s64(pg, base, indices).as_unsigned()
}
#[doc = "Load 32-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sw_gather_[s64]index_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_s64index_u64(
    pg: svbool_t,
    base: *const i32,
    indices: svint64_t,
) -> svuint64_t {
    svldnt1sw_gather_s64index_s64(pg, base, indices).as_unsigned()
}
#[doc = "Load 16-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sh_gather_[u64]index_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u64index_s64(
    pg: svbool_t,
    base: *const i16,
    indices: svuint64_t,
) -> svint64_t {
    svldnt1sh_gather_s64index_s64(pg, base, indices.as_signed())
}
#[doc = "Load 32-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sw_gather_[u64]index_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_u64index_s64(
    pg: svbool_t,
    base: *const i32,
    indices: svuint64_t,
) -> svint64_t {
    svldnt1sw_gather_s64index_s64(pg, base, indices.as_signed())
}
#[doc = "Load 16-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sh_gather_[u64]index_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u64index_u64(
    pg: svbool_t,
    base: *const i16,
    indices: svuint64_t,
) -> svuint64_t {
    svldnt1sh_gather_s64index_s64(pg, base, indices.as_signed()).as_unsigned()
}
#[doc = "Load 32-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sw_gather_[u64]index_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_u64index_u64(
    pg: svbool_t,
    base: *const i32,
    indices: svuint64_t,
) -> svuint64_t {
    svldnt1sw_gather_s64index_s64(pg, base, indices.as_signed()).as_unsigned()
}
#[doc = "Load 16-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sh_gather[_u32base]_index_s32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u32base_index_s32(
    pg: svbool_t,
    bases: svuint32_t,
    index: i64,
) -> svint32_t {
    svldnt1sh_gather_u32base_offset_s32(pg, bases, index.unchecked_shl(1))
}
#[doc = "Load 16-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sh_gather[_u32base]_index_u32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u32base_index_u32(
    pg: svbool_t,
    bases: svuint32_t,
    index: i64,
) -> svuint32_t {
    svldnt1sh_gather_u32base_offset_u32(pg, bases, index.unchecked_shl(1))
}
#[doc = "Load 16-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sh_gather[_u64base]_index_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u64base_index_s64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
) -> svint64_t {
    svldnt1sh_gather_u64base_offset_s64(pg, bases, index.unchecked_shl(1))
}
#[doc = "Load 32-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sw_gather[_u64base]_index_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_u64base_index_s64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
) -> svint64_t {
    svldnt1sw_gather_u64base_offset_s64(pg, bases, index.unchecked_shl(2))
}
#[doc = "Load 16-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sh_gather[_u64base]_index_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u64base_index_u64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
) -> svuint64_t {
    svldnt1sh_gather_u64base_offset_u64(pg, bases, index.unchecked_shl(1))
}
#[doc = "Load 32-bit data and sign-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1sw_gather[_u64base]_index_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_u64base_index_u64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
) -> svuint64_t {
    svldnt1sw_gather_u64base_offset_u64(pg, bases, index.unchecked_shl(2))
}
#[doc = "Load 8-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1ub_gather_[s64]offset_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_s64offset_s64(
    pg: svbool_t,
    base: *const u8,
    offsets: svint64_t,
) -> svint64_t {
    svldnt1ub_gather_s64offset_u64(pg, base, offsets).as_signed()
}
#[doc = "Load 16-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uh_gather_[s64]offset_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_s64offset_s64(
    pg: svbool_t,
    base: *const u16,
    offsets: svint64_t,
) -> svint64_t {
    svldnt1uh_gather_s64offset_u64(pg, base, offsets).as_signed()
}
#[doc = "Load 32-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uw_gather_[s64]offset_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_s64offset_s64(
    pg: svbool_t,
    base: *const u32,
    offsets: svint64_t,
) -> svint64_t {
    svldnt1uw_gather_s64offset_u64(pg, base, offsets).as_signed()
}
#[doc = "Load 8-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1ub_gather_[s64]offset_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_s64offset_u64(
    pg: svbool_t,
    base: *const u8,
    offsets: svint64_t,
) -> svuint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.nxv2i8"
        )]
        fn _svldnt1ub_gather_s64offset_u64(
            pg: svbool2_t,
            base: *const i8,
            offsets: svint64_t,
        ) -> nxv2i8;
    }
    simd_cast::<nxv2u8, _>(
        _svldnt1ub_gather_s64offset_u64(pg.into(), base.as_signed(), offsets).as_unsigned(),
    )
}
#[doc = "Load 16-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uh_gather_[s64]offset_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_s64offset_u64(
    pg: svbool_t,
    base: *const u16,
    offsets: svint64_t,
) -> svuint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.nxv2i16"
        )]
        fn _svldnt1uh_gather_s64offset_u64(
            pg: svbool2_t,
            base: *const i16,
            offsets: svint64_t,
        ) -> nxv2i16;
    }
    simd_cast::<nxv2u16, _>(
        _svldnt1uh_gather_s64offset_u64(pg.into(), base.as_signed(), offsets).as_unsigned(),
    )
}
#[doc = "Load 32-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uw_gather_[s64]offset_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_s64offset_u64(
    pg: svbool_t,
    base: *const u32,
    offsets: svint64_t,
) -> svuint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.nxv2i32"
        )]
        fn _svldnt1uw_gather_s64offset_u64(
            pg: svbool2_t,
            base: *const i32,
            offsets: svint64_t,
        ) -> nxv2i32;
    }
    simd_cast::<nxv2u32, _>(
        _svldnt1uw_gather_s64offset_u64(pg.into(), base.as_signed(), offsets).as_unsigned(),
    )
}
#[doc = "Load 8-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1ub_gather_[u32]offset_s32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_u32offset_s32(
    pg: svbool_t,
    base: *const u8,
    offsets: svuint32_t,
) -> svint32_t {
    svldnt1ub_gather_u32offset_u32(pg, base, offsets).as_signed()
}
#[doc = "Load 16-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uh_gather_[u32]offset_s32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u32offset_s32(
    pg: svbool_t,
    base: *const u16,
    offsets: svuint32_t,
) -> svint32_t {
    svldnt1uh_gather_u32offset_u32(pg, base, offsets).as_signed()
}
#[doc = "Load 8-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1ub_gather_[u32]offset_u32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_u32offset_u32(
    pg: svbool_t,
    base: *const u8,
    offsets: svuint32_t,
) -> svuint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.uxtw.nxv4i8"
        )]
        fn _svldnt1ub_gather_u32offset_u32(
            pg: svbool4_t,
            base: *const i8,
            offsets: svint32_t,
        ) -> nxv4i8;
    }
    simd_cast::<nxv4u8, _>(
        _svldnt1ub_gather_u32offset_u32(pg.into(), base.as_signed(), offsets.as_signed())
            .as_unsigned(),
    )
}
#[doc = "Load 16-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uh_gather_[u32]offset_u32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u32offset_u32(
    pg: svbool_t,
    base: *const u16,
    offsets: svuint32_t,
) -> svuint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.uxtw.nxv4i16"
        )]
        fn _svldnt1uh_gather_u32offset_u32(
            pg: svbool4_t,
            base: *const i16,
            offsets: svint32_t,
        ) -> nxv4i16;
    }
    simd_cast::<nxv4u16, _>(
        _svldnt1uh_gather_u32offset_u32(pg.into(), base.as_signed(), offsets.as_signed())
            .as_unsigned(),
    )
}
#[doc = "Load 8-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1ub_gather_[u64]offset_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_u64offset_s64(
    pg: svbool_t,
    base: *const u8,
    offsets: svuint64_t,
) -> svint64_t {
    svldnt1ub_gather_s64offset_u64(pg, base, offsets.as_signed()).as_signed()
}
#[doc = "Load 16-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uh_gather_[u64]offset_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u64offset_s64(
    pg: svbool_t,
    base: *const u16,
    offsets: svuint64_t,
) -> svint64_t {
    svldnt1uh_gather_s64offset_u64(pg, base, offsets.as_signed()).as_signed()
}
#[doc = "Load 32-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uw_gather_[u64]offset_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_u64offset_s64(
    pg: svbool_t,
    base: *const u32,
    offsets: svuint64_t,
) -> svint64_t {
    svldnt1uw_gather_s64offset_u64(pg, base, offsets.as_signed()).as_signed()
}
#[doc = "Load 8-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1ub_gather_[u64]offset_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_u64offset_u64(
    pg: svbool_t,
    base: *const u8,
    offsets: svuint64_t,
) -> svuint64_t {
    svldnt1ub_gather_s64offset_u64(pg, base, offsets.as_signed())
}
#[doc = "Load 16-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uh_gather_[u64]offset_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u64offset_u64(
    pg: svbool_t,
    base: *const u16,
    offsets: svuint64_t,
) -> svuint64_t {
    svldnt1uh_gather_s64offset_u64(pg, base, offsets.as_signed())
}
#[doc = "Load 32-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uw_gather_[u64]offset_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_u64offset_u64(
    pg: svbool_t,
    base: *const u32,
    offsets: svuint64_t,
) -> svuint64_t {
    svldnt1uw_gather_s64offset_u64(pg, base, offsets.as_signed())
}
#[doc = "Load 8-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1ub_gather[_u32base]_offset_s32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_u32base_offset_s32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
) -> svint32_t {
    svldnt1ub_gather_u32base_offset_u32(pg, bases, offset).as_signed()
}
#[doc = "Load 16-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uh_gather[_u32base]_offset_s32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u32base_offset_s32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
) -> svint32_t {
    svldnt1uh_gather_u32base_offset_u32(pg, bases, offset).as_signed()
}
#[doc = "Load 8-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1ub_gather[_u32base]_offset_u32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_u32base_offset_u32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
) -> svuint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.scalar.offset.nxv4i8.nxv4i32"
        )]
        fn _svldnt1ub_gather_u32base_offset_u32(
            pg: svbool4_t,
            bases: svint32_t,
            offset: i64,
        ) -> nxv4i8;
    }
    simd_cast::<nxv4u8, _>(
        _svldnt1ub_gather_u32base_offset_u32(pg.into(), bases.as_signed(), offset).as_unsigned(),
    )
}
#[doc = "Load 16-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uh_gather[_u32base]_offset_u32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u32base_offset_u32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
) -> svuint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.scalar.offset.nxv4i16.nxv4i32"
        )]
        fn _svldnt1uh_gather_u32base_offset_u32(
            pg: svbool4_t,
            bases: svint32_t,
            offset: i64,
        ) -> nxv4i16;
    }
    simd_cast::<nxv4u16, _>(
        _svldnt1uh_gather_u32base_offset_u32(pg.into(), bases.as_signed(), offset).as_unsigned(),
    )
}
#[doc = "Load 8-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1ub_gather[_u64base]_offset_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_u64base_offset_s64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svint64_t {
    svldnt1ub_gather_u64base_offset_u64(pg, bases, offset).as_signed()
}
#[doc = "Load 16-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uh_gather[_u64base]_offset_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u64base_offset_s64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svint64_t {
    svldnt1uh_gather_u64base_offset_u64(pg, bases, offset).as_signed()
}
#[doc = "Load 32-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uw_gather[_u64base]_offset_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_u64base_offset_s64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svint64_t {
    svldnt1uw_gather_u64base_offset_u64(pg, bases, offset).as_signed()
}
#[doc = "Load 8-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1ub_gather[_u64base]_offset_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_u64base_offset_u64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svuint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.scalar.offset.nxv2i8.nxv2i64"
        )]
        fn _svldnt1ub_gather_u64base_offset_u64(
            pg: svbool2_t,
            bases: svint64_t,
            offset: i64,
        ) -> nxv2i8;
    }
    simd_cast::<nxv2u8, _>(
        _svldnt1ub_gather_u64base_offset_u64(pg.into(), bases.as_signed(), offset).as_unsigned(),
    )
}
#[doc = "Load 16-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uh_gather[_u64base]_offset_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u64base_offset_u64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svuint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.scalar.offset.nxv2i16.nxv2i64"
        )]
        fn _svldnt1uh_gather_u64base_offset_u64(
            pg: svbool2_t,
            bases: svint64_t,
            offset: i64,
        ) -> nxv2i16;
    }
    simd_cast::<nxv2u16, _>(
        _svldnt1uh_gather_u64base_offset_u64(pg.into(), bases.as_signed(), offset).as_unsigned(),
    )
}
#[doc = "Load 32-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uw_gather[_u64base]_offset_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_u64base_offset_u64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svuint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.scalar.offset.nxv2i32.nxv2i64"
        )]
        fn _svldnt1uw_gather_u64base_offset_u64(
            pg: svbool2_t,
            bases: svint64_t,
            offset: i64,
        ) -> nxv2i32;
    }
    simd_cast::<nxv2u32, _>(
        _svldnt1uw_gather_u64base_offset_u64(pg.into(), bases.as_signed(), offset).as_unsigned(),
    )
}
#[doc = "Load 8-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1ub_gather[_u32base]_s32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_u32base_s32(pg: svbool_t, bases: svuint32_t) -> svint32_t {
    svldnt1ub_gather_u32base_offset_s32(pg, bases, 0)
}
#[doc = "Load 16-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uh_gather[_u32base]_s32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u32base_s32(pg: svbool_t, bases: svuint32_t) -> svint32_t {
    svldnt1uh_gather_u32base_offset_s32(pg, bases, 0)
}
#[doc = "Load 8-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1ub_gather[_u32base]_u32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_u32base_u32(pg: svbool_t, bases: svuint32_t) -> svuint32_t {
    svldnt1ub_gather_u32base_offset_u32(pg, bases, 0)
}
#[doc = "Load 16-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uh_gather[_u32base]_u32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u32base_u32(pg: svbool_t, bases: svuint32_t) -> svuint32_t {
    svldnt1uh_gather_u32base_offset_u32(pg, bases, 0)
}
#[doc = "Load 8-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1ub_gather[_u64base]_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_u64base_s64(pg: svbool_t, bases: svuint64_t) -> svint64_t {
    svldnt1ub_gather_u64base_offset_s64(pg, bases, 0)
}
#[doc = "Load 16-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uh_gather[_u64base]_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u64base_s64(pg: svbool_t, bases: svuint64_t) -> svint64_t {
    svldnt1uh_gather_u64base_offset_s64(pg, bases, 0)
}
#[doc = "Load 32-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uw_gather[_u64base]_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_u64base_s64(pg: svbool_t, bases: svuint64_t) -> svint64_t {
    svldnt1uw_gather_u64base_offset_s64(pg, bases, 0)
}
#[doc = "Load 8-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1ub_gather[_u64base]_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_u64base_u64(pg: svbool_t, bases: svuint64_t) -> svuint64_t {
    svldnt1ub_gather_u64base_offset_u64(pg, bases, 0)
}
#[doc = "Load 16-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uh_gather[_u64base]_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u64base_u64(pg: svbool_t, bases: svuint64_t) -> svuint64_t {
    svldnt1uh_gather_u64base_offset_u64(pg, bases, 0)
}
#[doc = "Load 32-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uw_gather[_u64base]_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_u64base_u64(pg: svbool_t, bases: svuint64_t) -> svuint64_t {
    svldnt1uw_gather_u64base_offset_u64(pg, bases, 0)
}
#[doc = "Load 16-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uh_gather_[s64]index_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_s64index_s64(
    pg: svbool_t,
    base: *const u16,
    indices: svint64_t,
) -> svint64_t {
    svldnt1uh_gather_s64index_u64(pg, base, indices).as_signed()
}
#[doc = "Load 32-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uw_gather_[s64]index_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_s64index_s64(
    pg: svbool_t,
    base: *const u32,
    indices: svint64_t,
) -> svint64_t {
    svldnt1uw_gather_s64index_u64(pg, base, indices).as_signed()
}
#[doc = "Load 16-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uh_gather_[s64]index_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_s64index_u64(
    pg: svbool_t,
    base: *const u16,
    indices: svint64_t,
) -> svuint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.index.nxv2i16"
        )]
        fn _svldnt1uh_gather_s64index_u64(
            pg: svbool2_t,
            base: *const i16,
            indices: svint64_t,
        ) -> nxv2i16;
    }
    simd_cast::<nxv2u16, _>(
        _svldnt1uh_gather_s64index_u64(pg.into(), base.as_signed(), indices).as_unsigned(),
    )
}
#[doc = "Load 32-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uw_gather_[s64]index_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_s64index_u64(
    pg: svbool_t,
    base: *const u32,
    indices: svint64_t,
) -> svuint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ldnt1.gather.index.nxv2i32"
        )]
        fn _svldnt1uw_gather_s64index_u64(
            pg: svbool2_t,
            base: *const i32,
            indices: svint64_t,
        ) -> nxv2i32;
    }
    simd_cast::<nxv2u32, _>(
        _svldnt1uw_gather_s64index_u64(pg.into(), base.as_signed(), indices).as_unsigned(),
    )
}
#[doc = "Load 16-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uh_gather_[u64]index_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u64index_s64(
    pg: svbool_t,
    base: *const u16,
    indices: svuint64_t,
) -> svint64_t {
    svldnt1uh_gather_s64index_u64(pg, base, indices.as_signed()).as_signed()
}
#[doc = "Load 32-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uw_gather_[u64]index_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_u64index_s64(
    pg: svbool_t,
    base: *const u32,
    indices: svuint64_t,
) -> svint64_t {
    svldnt1uw_gather_s64index_u64(pg, base, indices.as_signed()).as_signed()
}
#[doc = "Load 16-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uh_gather_[u64]index_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u64index_u64(
    pg: svbool_t,
    base: *const u16,
    indices: svuint64_t,
) -> svuint64_t {
    svldnt1uh_gather_s64index_u64(pg, base, indices.as_signed())
}
#[doc = "Load 32-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uw_gather_[u64]index_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_u64index_u64(
    pg: svbool_t,
    base: *const u32,
    indices: svuint64_t,
) -> svuint64_t {
    svldnt1uw_gather_s64index_u64(pg, base, indices.as_signed())
}
#[doc = "Load 16-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uh_gather[_u32base]_index_s32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u32base_index_s32(
    pg: svbool_t,
    bases: svuint32_t,
    index: i64,
) -> svint32_t {
    svldnt1uh_gather_u32base_offset_s32(pg, bases, index.unchecked_shl(1))
}
#[doc = "Load 16-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uh_gather[_u32base]_index_u32)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u32base_index_u32(
    pg: svbool_t,
    bases: svuint32_t,
    index: i64,
) -> svuint32_t {
    svldnt1uh_gather_u32base_offset_u32(pg, bases, index.unchecked_shl(1))
}
#[doc = "Load 16-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uh_gather[_u64base]_index_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u64base_index_s64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
) -> svint64_t {
    svldnt1uh_gather_u64base_offset_s64(pg, bases, index.unchecked_shl(1))
}
#[doc = "Load 32-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uw_gather[_u64base]_index_s64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_u64base_index_s64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
) -> svint64_t {
    svldnt1uw_gather_u64base_offset_s64(pg, bases, index.unchecked_shl(2))
}
#[doc = "Load 16-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uh_gather[_u64base]_index_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u64base_index_u64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
) -> svuint64_t {
    svldnt1uh_gather_u64base_offset_u64(pg, bases, index.unchecked_shl(1))
}
#[doc = "Load 32-bit data and zero-extend, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svldnt1uw_gather[_u64base]_index_u64)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_u64base_index_u64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
) -> svuint64_t {
    svldnt1uw_gather_u64base_offset_u64(pg, bases, index.unchecked_shl(2))
}
#[doc = "Base 2 logarithm as integer"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svlogb[_f32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(flogb))]
pub fn svlogb_f32_m(inactive: svint32_t, pg: svbool_t, op: svfloat32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.flogb.nxv4f32")]
        fn _svlogb_f32_m(inactive: svint32_t, pg: svbool4_t, op: svfloat32_t) -> svint32_t;
    }
    unsafe { _svlogb_f32_m(inactive, pg.into(), op) }
}
#[doc = "Base 2 logarithm as integer"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svlogb[_f32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(flogb))]
pub fn svlogb_f32_x(pg: svbool_t, op: svfloat32_t) -> svint32_t {
    unsafe { svlogb_f32_m(simd_reinterpret(op), pg, op) }
}
#[doc = "Base 2 logarithm as integer"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svlogb[_f32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(flogb))]
pub fn svlogb_f32_z(pg: svbool_t, op: svfloat32_t) -> svint32_t {
    svlogb_f32_m(svdup_n_s32(0), pg, op)
}
#[doc = "Base 2 logarithm as integer"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svlogb[_f64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(flogb))]
pub fn svlogb_f64_m(inactive: svint64_t, pg: svbool_t, op: svfloat64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.flogb.nxv2f64")]
        fn _svlogb_f64_m(inactive: svint64_t, pg: svbool2_t, op: svfloat64_t) -> svint64_t;
    }
    unsafe { _svlogb_f64_m(inactive, pg.into(), op) }
}
#[doc = "Base 2 logarithm as integer"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svlogb[_f64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(flogb))]
pub fn svlogb_f64_x(pg: svbool_t, op: svfloat64_t) -> svint64_t {
    unsafe { svlogb_f64_m(simd_reinterpret(op), pg, op) }
}
#[doc = "Base 2 logarithm as integer"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svlogb[_f64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(flogb))]
pub fn svlogb_f64_z(pg: svbool_t, op: svfloat64_t) -> svint64_t {
    svlogb_f64_m(svdup_n_s64(0), pg, op)
}
#[doc = "Detect any matching elements"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmatch[_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(match))]
pub fn svmatch_s8(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svbool_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.match.nxv16i8")]
        fn _svmatch_s8(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svbool_t;
    }
    unsafe { _svmatch_s8(pg, op1, op2) }
}
#[doc = "Detect any matching elements"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmatch[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(match))]
pub fn svmatch_s16(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svbool_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.match.nxv8i16")]
        fn _svmatch_s16(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svbool8_t;
    }
    unsafe { _svmatch_s16(pg.into(), op1, op2).into() }
}
#[doc = "Detect any matching elements"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmatch[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(match))]
pub fn svmatch_u8(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svbool_t {
    unsafe { svmatch_s8(pg, op1.as_signed(), op2.as_signed()) }
}
#[doc = "Detect any matching elements"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmatch[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(match))]
pub fn svmatch_u16(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svbool_t {
    unsafe { svmatch_s16(pg, op1.as_signed(), op2.as_signed()) }
}
#[doc = "Maximum number pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmaxnmp[_f32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(fmaxnmp))]
pub fn svmaxnmp_f32_m(pg: svbool_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.fmaxnmp.nxv4f32"
        )]
        fn _svmaxnmp_f32_m(pg: svbool4_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t;
    }
    unsafe { _svmaxnmp_f32_m(pg.into(), op1, op2) }
}
#[doc = "Maximum number pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmaxnmp[_f32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(fmaxnmp))]
pub fn svmaxnmp_f32_x(pg: svbool_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t {
    svmaxnmp_f32_m(pg, op1, op2)
}
#[doc = "Maximum number pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmaxnmp[_f64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(fmaxnmp))]
pub fn svmaxnmp_f64_m(pg: svbool_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.fmaxnmp.nxv2f64"
        )]
        fn _svmaxnmp_f64_m(pg: svbool2_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t;
    }
    unsafe { _svmaxnmp_f64_m(pg.into(), op1, op2) }
}
#[doc = "Maximum number pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmaxnmp[_f64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(fmaxnmp))]
pub fn svmaxnmp_f64_x(pg: svbool_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t {
    svmaxnmp_f64_m(pg, op1, op2)
}
#[doc = "Maximum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmaxp[_f32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(fmaxp))]
pub fn svmaxp_f32_m(pg: svbool_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.fmaxp.nxv4f32")]
        fn _svmaxp_f32_m(pg: svbool4_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t;
    }
    unsafe { _svmaxp_f32_m(pg.into(), op1, op2) }
}
#[doc = "Maximum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmaxp[_f32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(fmaxp))]
pub fn svmaxp_f32_x(pg: svbool_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t {
    svmaxp_f32_m(pg, op1, op2)
}
#[doc = "Maximum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmaxp[_f64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(fmaxp))]
pub fn svmaxp_f64_m(pg: svbool_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.fmaxp.nxv2f64")]
        fn _svmaxp_f64_m(pg: svbool2_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t;
    }
    unsafe { _svmaxp_f64_m(pg.into(), op1, op2) }
}
#[doc = "Maximum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmaxp[_f64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(fmaxp))]
pub fn svmaxp_f64_x(pg: svbool_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t {
    svmaxp_f64_m(pg, op1, op2)
}
#[doc = "Maximum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmaxp[_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smaxp))]
pub fn svmaxp_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.smaxp.nxv16i8")]
        fn _svmaxp_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svmaxp_s8_m(pg, op1, op2) }
}
#[doc = "Maximum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmaxp[_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smaxp))]
pub fn svmaxp_s8_x(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svmaxp_s8_m(pg, op1, op2)
}
#[doc = "Maximum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmaxp[_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smaxp))]
pub fn svmaxp_s16_m(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.smaxp.nxv8i16")]
        fn _svmaxp_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svmaxp_s16_m(pg.into(), op1, op2) }
}
#[doc = "Maximum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmaxp[_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smaxp))]
pub fn svmaxp_s16_x(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svmaxp_s16_m(pg, op1, op2)
}
#[doc = "Maximum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmaxp[_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smaxp))]
pub fn svmaxp_s32_m(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.smaxp.nxv4i32")]
        fn _svmaxp_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svmaxp_s32_m(pg.into(), op1, op2) }
}
#[doc = "Maximum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmaxp[_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smaxp))]
pub fn svmaxp_s32_x(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svmaxp_s32_m(pg, op1, op2)
}
#[doc = "Maximum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmaxp[_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smaxp))]
pub fn svmaxp_s64_m(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.smaxp.nxv2i64")]
        fn _svmaxp_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svmaxp_s64_m(pg.into(), op1, op2) }
}
#[doc = "Maximum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmaxp[_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smaxp))]
pub fn svmaxp_s64_x(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svmaxp_s64_m(pg, op1, op2)
}
#[doc = "Maximum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmaxp[_u8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umaxp))]
pub fn svmaxp_u8_m(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.umaxp.nxv16i8")]
        fn _svmaxp_u8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svmaxp_u8_m(pg, op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Maximum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmaxp[_u8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umaxp))]
pub fn svmaxp_u8_x(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svmaxp_u8_m(pg, op1, op2)
}
#[doc = "Maximum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmaxp[_u16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umaxp))]
pub fn svmaxp_u16_m(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.umaxp.nxv8i16")]
        fn _svmaxp_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svmaxp_u16_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Maximum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmaxp[_u16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umaxp))]
pub fn svmaxp_u16_x(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svmaxp_u16_m(pg, op1, op2)
}
#[doc = "Maximum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmaxp[_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umaxp))]
pub fn svmaxp_u32_m(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.umaxp.nxv4i32")]
        fn _svmaxp_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svmaxp_u32_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Maximum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmaxp[_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umaxp))]
pub fn svmaxp_u32_x(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svmaxp_u32_m(pg, op1, op2)
}
#[doc = "Maximum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmaxp[_u64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umaxp))]
pub fn svmaxp_u64_m(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.umaxp.nxv2i64")]
        fn _svmaxp_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svmaxp_u64_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Maximum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmaxp[_u64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umaxp))]
pub fn svmaxp_u64_x(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svmaxp_u64_m(pg, op1, op2)
}
#[doc = "Minimum number pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svminnmp[_f32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(fminnmp))]
pub fn svminnmp_f32_m(pg: svbool_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.fminnmp.nxv4f32"
        )]
        fn _svminnmp_f32_m(pg: svbool4_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t;
    }
    unsafe { _svminnmp_f32_m(pg.into(), op1, op2) }
}
#[doc = "Minimum number pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svminnmp[_f32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(fminnmp))]
pub fn svminnmp_f32_x(pg: svbool_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t {
    svminnmp_f32_m(pg, op1, op2)
}
#[doc = "Minimum number pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svminnmp[_f64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(fminnmp))]
pub fn svminnmp_f64_m(pg: svbool_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.fminnmp.nxv2f64"
        )]
        fn _svminnmp_f64_m(pg: svbool2_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t;
    }
    unsafe { _svminnmp_f64_m(pg.into(), op1, op2) }
}
#[doc = "Minimum number pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svminnmp[_f64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(fminnmp))]
pub fn svminnmp_f64_x(pg: svbool_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t {
    svminnmp_f64_m(pg, op1, op2)
}
#[doc = "Minimum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svminp[_f32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(fminp))]
pub fn svminp_f32_m(pg: svbool_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.fminp.nxv4f32")]
        fn _svminp_f32_m(pg: svbool4_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t;
    }
    unsafe { _svminp_f32_m(pg.into(), op1, op2) }
}
#[doc = "Minimum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svminp[_f32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(fminp))]
pub fn svminp_f32_x(pg: svbool_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t {
    svminp_f32_m(pg, op1, op2)
}
#[doc = "Minimum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svminp[_f64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(fminp))]
pub fn svminp_f64_m(pg: svbool_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.fminp.nxv2f64")]
        fn _svminp_f64_m(pg: svbool2_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t;
    }
    unsafe { _svminp_f64_m(pg.into(), op1, op2) }
}
#[doc = "Minimum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svminp[_f64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(fminp))]
pub fn svminp_f64_x(pg: svbool_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t {
    svminp_f64_m(pg, op1, op2)
}
#[doc = "Minimum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svminp[_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sminp))]
pub fn svminp_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sminp.nxv16i8")]
        fn _svminp_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svminp_s8_m(pg, op1, op2) }
}
#[doc = "Minimum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svminp[_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sminp))]
pub fn svminp_s8_x(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svminp_s8_m(pg, op1, op2)
}
#[doc = "Minimum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svminp[_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sminp))]
pub fn svminp_s16_m(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sminp.nxv8i16")]
        fn _svminp_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svminp_s16_m(pg.into(), op1, op2) }
}
#[doc = "Minimum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svminp[_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sminp))]
pub fn svminp_s16_x(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svminp_s16_m(pg, op1, op2)
}
#[doc = "Minimum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svminp[_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sminp))]
pub fn svminp_s32_m(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sminp.nxv4i32")]
        fn _svminp_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svminp_s32_m(pg.into(), op1, op2) }
}
#[doc = "Minimum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svminp[_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sminp))]
pub fn svminp_s32_x(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svminp_s32_m(pg, op1, op2)
}
#[doc = "Minimum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svminp[_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sminp))]
pub fn svminp_s64_m(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sminp.nxv2i64")]
        fn _svminp_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svminp_s64_m(pg.into(), op1, op2) }
}
#[doc = "Minimum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svminp[_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sminp))]
pub fn svminp_s64_x(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svminp_s64_m(pg, op1, op2)
}
#[doc = "Minimum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svminp[_u8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uminp))]
pub fn svminp_u8_m(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uminp.nxv16i8")]
        fn _svminp_u8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svminp_u8_m(pg, op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Minimum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svminp[_u8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uminp))]
pub fn svminp_u8_x(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svminp_u8_m(pg, op1, op2)
}
#[doc = "Minimum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svminp[_u16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uminp))]
pub fn svminp_u16_m(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uminp.nxv8i16")]
        fn _svminp_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svminp_u16_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Minimum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svminp[_u16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uminp))]
pub fn svminp_u16_x(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svminp_u16_m(pg, op1, op2)
}
#[doc = "Minimum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svminp[_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uminp))]
pub fn svminp_u32_m(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uminp.nxv4i32")]
        fn _svminp_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svminp_u32_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Minimum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svminp[_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uminp))]
pub fn svminp_u32_x(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svminp_u32_m(pg, op1, op2)
}
#[doc = "Minimum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svminp[_u64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uminp))]
pub fn svminp_u64_m(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uminp.nxv2i64")]
        fn _svminp_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svminp_u64_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Minimum pairwise"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svminp[_u64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uminp))]
pub fn svminp_u64_x(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svminp_u64_m(pg, op1, op2)
}
#[doc = "Multiply-add, addend first"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmla_lane[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(mla, IMM_INDEX = 0))]
pub fn svmla_lane_s16<const IMM_INDEX: i32>(
    op1: svint16_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint16_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.mla.lane.nxv8i16"
        )]
        fn _svmla_lane_s16(
            op1: svint16_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint16_t;
    }
    unsafe { _svmla_lane_s16(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Multiply-add, addend first"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmla_lane[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(mla, IMM_INDEX = 0))]
pub fn svmla_lane_s32<const IMM_INDEX: i32>(
    op1: svint32_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint32_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.mla.lane.nxv4i32"
        )]
        fn _svmla_lane_s32(
            op1: svint32_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe { _svmla_lane_s32(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Multiply-add, addend first"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmla_lane[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(mla, IMM_INDEX = 0))]
pub fn svmla_lane_s64<const IMM_INDEX: i32>(
    op1: svint64_t,
    op2: svint64_t,
    op3: svint64_t,
) -> svint64_t {
    static_assert_range!(IMM_INDEX, 0, 1);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.mla.lane.nxv2i64"
        )]
        fn _svmla_lane_s64(
            op1: svint64_t,
            op2: svint64_t,
            op3: svint64_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe { _svmla_lane_s64(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Multiply-add, addend first"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmla_lane[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(mla, IMM_INDEX = 0))]
pub fn svmla_lane_u16<const IMM_INDEX: i32>(
    op1: svuint16_t,
    op2: svuint16_t,
    op3: svuint16_t,
) -> svuint16_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    unsafe {
        svmla_lane_s16::<IMM_INDEX>(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned()
    }
}
#[doc = "Multiply-add, addend first"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmla_lane[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(mla, IMM_INDEX = 0))]
pub fn svmla_lane_u32<const IMM_INDEX: i32>(
    op1: svuint32_t,
    op2: svuint32_t,
    op3: svuint32_t,
) -> svuint32_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    unsafe {
        svmla_lane_s32::<IMM_INDEX>(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned()
    }
}
#[doc = "Multiply-add, addend first"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmla_lane[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(mla, IMM_INDEX = 0))]
pub fn svmla_lane_u64<const IMM_INDEX: i32>(
    op1: svuint64_t,
    op2: svuint64_t,
    op3: svuint64_t,
) -> svuint64_t {
    static_assert_range!(IMM_INDEX, 0, 1);
    unsafe {
        svmla_lane_s64::<IMM_INDEX>(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned()
    }
}
#[doc = "Multiply-add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalb_lane[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlalb, IMM_INDEX = 0))]
pub fn svmlalb_lane_s32<const IMM_INDEX: i32>(
    op1: svint32_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint32_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.smlalb.lane.nxv4i32"
        )]
        fn _svmlalb_lane_s32(
            op1: svint32_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe { _svmlalb_lane_s32(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Multiply-add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalb_lane[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlalb, IMM_INDEX = 0))]
pub fn svmlalb_lane_s64<const IMM_INDEX: i32>(
    op1: svint64_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint64_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.smlalb.lane.nxv2i64"
        )]
        fn _svmlalb_lane_s64(
            op1: svint64_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe { _svmlalb_lane_s64(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Multiply-add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalb_lane[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlalb, IMM_INDEX = 0))]
pub fn svmlalb_lane_u32<const IMM_INDEX: i32>(
    op1: svuint32_t,
    op2: svuint16_t,
    op3: svuint16_t,
) -> svuint32_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.umlalb.lane.nxv4i32"
        )]
        fn _svmlalb_lane_u32(
            op1: svint32_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe {
        _svmlalb_lane_u32(op1.as_signed(), op2.as_signed(), op3.as_signed(), IMM_INDEX)
            .as_unsigned()
    }
}
#[doc = "Multiply-add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalb_lane[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlalb, IMM_INDEX = 0))]
pub fn svmlalb_lane_u64<const IMM_INDEX: i32>(
    op1: svuint64_t,
    op2: svuint32_t,
    op3: svuint32_t,
) -> svuint64_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.umlalb.lane.nxv2i64"
        )]
        fn _svmlalb_lane_u64(
            op1: svint64_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe {
        _svmlalb_lane_u64(op1.as_signed(), op2.as_signed(), op3.as_signed(), IMM_INDEX)
            .as_unsigned()
    }
}
#[doc = "Multiply-add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalb[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlalb))]
pub fn svmlalb_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.smlalb.nxv8i16")]
        fn _svmlalb_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svmlalb_s16(op1, op2, op3) }
}
#[doc = "Multiply-add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalb[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlalb))]
pub fn svmlalb_n_s16(op1: svint16_t, op2: svint8_t, op3: i8) -> svint16_t {
    svmlalb_s16(op1, op2, svdup_n_s8(op3))
}
#[doc = "Multiply-add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalb[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlalb))]
pub fn svmlalb_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.smlalb.nxv4i32")]
        fn _svmlalb_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svmlalb_s32(op1, op2, op3) }
}
#[doc = "Multiply-add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalb[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlalb))]
pub fn svmlalb_n_s32(op1: svint32_t, op2: svint16_t, op3: i16) -> svint32_t {
    svmlalb_s32(op1, op2, svdup_n_s16(op3))
}
#[doc = "Multiply-add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalb[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlalb))]
pub fn svmlalb_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.smlalb.nxv2i64")]
        fn _svmlalb_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svmlalb_s64(op1, op2, op3) }
}
#[doc = "Multiply-add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalb[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlalb))]
pub fn svmlalb_n_s64(op1: svint64_t, op2: svint32_t, op3: i32) -> svint64_t {
    svmlalb_s64(op1, op2, svdup_n_s32(op3))
}
#[doc = "Multiply-add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalb[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlalb))]
pub fn svmlalb_u16(op1: svuint16_t, op2: svuint8_t, op3: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.umlalb.nxv8i16")]
        fn _svmlalb_u16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svmlalb_u16(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Multiply-add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalb[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlalb))]
pub fn svmlalb_n_u16(op1: svuint16_t, op2: svuint8_t, op3: u8) -> svuint16_t {
    svmlalb_u16(op1, op2, svdup_n_u8(op3))
}
#[doc = "Multiply-add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalb[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlalb))]
pub fn svmlalb_u32(op1: svuint32_t, op2: svuint16_t, op3: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.umlalb.nxv4i32")]
        fn _svmlalb_u32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svmlalb_u32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Multiply-add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalb[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlalb))]
pub fn svmlalb_n_u32(op1: svuint32_t, op2: svuint16_t, op3: u16) -> svuint32_t {
    svmlalb_u32(op1, op2, svdup_n_u16(op3))
}
#[doc = "Multiply-add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalb[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlalb))]
pub fn svmlalb_u64(op1: svuint64_t, op2: svuint32_t, op3: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.umlalb.nxv2i64")]
        fn _svmlalb_u64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svmlalb_u64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Multiply-add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalb[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlalb))]
pub fn svmlalb_n_u64(op1: svuint64_t, op2: svuint32_t, op3: u32) -> svuint64_t {
    svmlalb_u64(op1, op2, svdup_n_u32(op3))
}
#[doc = "Multiply-add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalt_lane[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlalt, IMM_INDEX = 0))]
pub fn svmlalt_lane_s32<const IMM_INDEX: i32>(
    op1: svint32_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint32_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.smlalt.lane.nxv4i32"
        )]
        fn _svmlalt_lane_s32(
            op1: svint32_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe { _svmlalt_lane_s32(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Multiply-add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalt_lane[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlalt, IMM_INDEX = 0))]
pub fn svmlalt_lane_s64<const IMM_INDEX: i32>(
    op1: svint64_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint64_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.smlalt.lane.nxv2i64"
        )]
        fn _svmlalt_lane_s64(
            op1: svint64_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe { _svmlalt_lane_s64(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Multiply-add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalt_lane[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlalt, IMM_INDEX = 0))]
pub fn svmlalt_lane_u32<const IMM_INDEX: i32>(
    op1: svuint32_t,
    op2: svuint16_t,
    op3: svuint16_t,
) -> svuint32_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.umlalt.lane.nxv4i32"
        )]
        fn _svmlalt_lane_u32(
            op1: svint32_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe {
        _svmlalt_lane_u32(op1.as_signed(), op2.as_signed(), op3.as_signed(), IMM_INDEX)
            .as_unsigned()
    }
}
#[doc = "Multiply-add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalt_lane[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlalt, IMM_INDEX = 0))]
pub fn svmlalt_lane_u64<const IMM_INDEX: i32>(
    op1: svuint64_t,
    op2: svuint32_t,
    op3: svuint32_t,
) -> svuint64_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.umlalt.lane.nxv2i64"
        )]
        fn _svmlalt_lane_u64(
            op1: svint64_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe {
        _svmlalt_lane_u64(op1.as_signed(), op2.as_signed(), op3.as_signed(), IMM_INDEX)
            .as_unsigned()
    }
}
#[doc = "Multiply-add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalt[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlalt))]
pub fn svmlalt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.smlalt.nxv8i16")]
        fn _svmlalt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svmlalt_s16(op1, op2, op3) }
}
#[doc = "Multiply-add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlalt))]
pub fn svmlalt_n_s16(op1: svint16_t, op2: svint8_t, op3: i8) -> svint16_t {
    svmlalt_s16(op1, op2, svdup_n_s8(op3))
}
#[doc = "Multiply-add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalt[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlalt))]
pub fn svmlalt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.smlalt.nxv4i32")]
        fn _svmlalt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svmlalt_s32(op1, op2, op3) }
}
#[doc = "Multiply-add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlalt))]
pub fn svmlalt_n_s32(op1: svint32_t, op2: svint16_t, op3: i16) -> svint32_t {
    svmlalt_s32(op1, op2, svdup_n_s16(op3))
}
#[doc = "Multiply-add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalt[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlalt))]
pub fn svmlalt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.smlalt.nxv2i64")]
        fn _svmlalt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svmlalt_s64(op1, op2, op3) }
}
#[doc = "Multiply-add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlalt))]
pub fn svmlalt_n_s64(op1: svint64_t, op2: svint32_t, op3: i32) -> svint64_t {
    svmlalt_s64(op1, op2, svdup_n_s32(op3))
}
#[doc = "Multiply-add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalt[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlalt))]
pub fn svmlalt_u16(op1: svuint16_t, op2: svuint8_t, op3: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.umlalt.nxv8i16")]
        fn _svmlalt_u16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svmlalt_u16(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Multiply-add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalt[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlalt))]
pub fn svmlalt_n_u16(op1: svuint16_t, op2: svuint8_t, op3: u8) -> svuint16_t {
    svmlalt_u16(op1, op2, svdup_n_u8(op3))
}
#[doc = "Multiply-add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalt[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlalt))]
pub fn svmlalt_u32(op1: svuint32_t, op2: svuint16_t, op3: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.umlalt.nxv4i32")]
        fn _svmlalt_u32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svmlalt_u32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Multiply-add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalt[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlalt))]
pub fn svmlalt_n_u32(op1: svuint32_t, op2: svuint16_t, op3: u16) -> svuint32_t {
    svmlalt_u32(op1, op2, svdup_n_u16(op3))
}
#[doc = "Multiply-add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalt[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlalt))]
pub fn svmlalt_u64(op1: svuint64_t, op2: svuint32_t, op3: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.umlalt.nxv2i64")]
        fn _svmlalt_u64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svmlalt_u64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Multiply-add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlalt[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlalt))]
pub fn svmlalt_n_u64(op1: svuint64_t, op2: svuint32_t, op3: u32) -> svuint64_t {
    svmlalt_u64(op1, op2, svdup_n_u32(op3))
}
#[doc = "Multiply-subtract, minuend first"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmls_lane[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(mls, IMM_INDEX = 0))]
pub fn svmls_lane_s16<const IMM_INDEX: i32>(
    op1: svint16_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint16_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.mls.lane.nxv8i16"
        )]
        fn _svmls_lane_s16(
            op1: svint16_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint16_t;
    }
    unsafe { _svmls_lane_s16(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Multiply-subtract, minuend first"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmls_lane[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(mls, IMM_INDEX = 0))]
pub fn svmls_lane_s32<const IMM_INDEX: i32>(
    op1: svint32_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint32_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.mls.lane.nxv4i32"
        )]
        fn _svmls_lane_s32(
            op1: svint32_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe { _svmls_lane_s32(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Multiply-subtract, minuend first"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmls_lane[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(mls, IMM_INDEX = 0))]
pub fn svmls_lane_s64<const IMM_INDEX: i32>(
    op1: svint64_t,
    op2: svint64_t,
    op3: svint64_t,
) -> svint64_t {
    static_assert_range!(IMM_INDEX, 0, 1);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.mls.lane.nxv2i64"
        )]
        fn _svmls_lane_s64(
            op1: svint64_t,
            op2: svint64_t,
            op3: svint64_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe { _svmls_lane_s64(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Multiply-subtract, minuend first"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmls_lane[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(mls, IMM_INDEX = 0))]
pub fn svmls_lane_u16<const IMM_INDEX: i32>(
    op1: svuint16_t,
    op2: svuint16_t,
    op3: svuint16_t,
) -> svuint16_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    unsafe {
        svmls_lane_s16::<IMM_INDEX>(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned()
    }
}
#[doc = "Multiply-subtract, minuend first"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmls_lane[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(mls, IMM_INDEX = 0))]
pub fn svmls_lane_u32<const IMM_INDEX: i32>(
    op1: svuint32_t,
    op2: svuint32_t,
    op3: svuint32_t,
) -> svuint32_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    unsafe {
        svmls_lane_s32::<IMM_INDEX>(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned()
    }
}
#[doc = "Multiply-subtract, minuend first"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmls_lane[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(mls, IMM_INDEX = 0))]
pub fn svmls_lane_u64<const IMM_INDEX: i32>(
    op1: svuint64_t,
    op2: svuint64_t,
    op3: svuint64_t,
) -> svuint64_t {
    static_assert_range!(IMM_INDEX, 0, 1);
    unsafe {
        svmls_lane_s64::<IMM_INDEX>(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned()
    }
}
#[doc = "Multiply-subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslb_lane[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlslb, IMM_INDEX = 0))]
pub fn svmlslb_lane_s32<const IMM_INDEX: i32>(
    op1: svint32_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint32_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.smlslb.lane.nxv4i32"
        )]
        fn _svmlslb_lane_s32(
            op1: svint32_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe { _svmlslb_lane_s32(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Multiply-subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslb_lane[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlslb, IMM_INDEX = 0))]
pub fn svmlslb_lane_s64<const IMM_INDEX: i32>(
    op1: svint64_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint64_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.smlslb.lane.nxv2i64"
        )]
        fn _svmlslb_lane_s64(
            op1: svint64_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe { _svmlslb_lane_s64(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Multiply-subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslb_lane[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlslb, IMM_INDEX = 0))]
pub fn svmlslb_lane_u32<const IMM_INDEX: i32>(
    op1: svuint32_t,
    op2: svuint16_t,
    op3: svuint16_t,
) -> svuint32_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.umlslb.lane.nxv4i32"
        )]
        fn _svmlslb_lane_u32(
            op1: svint32_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe {
        _svmlslb_lane_u32(op1.as_signed(), op2.as_signed(), op3.as_signed(), IMM_INDEX)
            .as_unsigned()
    }
}
#[doc = "Multiply-subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslb_lane[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlslb, IMM_INDEX = 0))]
pub fn svmlslb_lane_u64<const IMM_INDEX: i32>(
    op1: svuint64_t,
    op2: svuint32_t,
    op3: svuint32_t,
) -> svuint64_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.umlslb.lane.nxv2i64"
        )]
        fn _svmlslb_lane_u64(
            op1: svint64_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe {
        _svmlslb_lane_u64(op1.as_signed(), op2.as_signed(), op3.as_signed(), IMM_INDEX)
            .as_unsigned()
    }
}
#[doc = "Multiply-subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslb[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlslb))]
pub fn svmlslb_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.smlslb.nxv8i16")]
        fn _svmlslb_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svmlslb_s16(op1, op2, op3) }
}
#[doc = "Multiply-subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslb[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlslb))]
pub fn svmlslb_n_s16(op1: svint16_t, op2: svint8_t, op3: i8) -> svint16_t {
    svmlslb_s16(op1, op2, svdup_n_s8(op3))
}
#[doc = "Multiply-subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslb[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlslb))]
pub fn svmlslb_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.smlslb.nxv4i32")]
        fn _svmlslb_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svmlslb_s32(op1, op2, op3) }
}
#[doc = "Multiply-subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslb[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlslb))]
pub fn svmlslb_n_s32(op1: svint32_t, op2: svint16_t, op3: i16) -> svint32_t {
    svmlslb_s32(op1, op2, svdup_n_s16(op3))
}
#[doc = "Multiply-subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslb[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlslb))]
pub fn svmlslb_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.smlslb.nxv2i64")]
        fn _svmlslb_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svmlslb_s64(op1, op2, op3) }
}
#[doc = "Multiply-subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslb[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlslb))]
pub fn svmlslb_n_s64(op1: svint64_t, op2: svint32_t, op3: i32) -> svint64_t {
    svmlslb_s64(op1, op2, svdup_n_s32(op3))
}
#[doc = "Multiply-subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslb[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlslb))]
pub fn svmlslb_u16(op1: svuint16_t, op2: svuint8_t, op3: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.umlslb.nxv8i16")]
        fn _svmlslb_u16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svmlslb_u16(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Multiply-subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslb[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlslb))]
pub fn svmlslb_n_u16(op1: svuint16_t, op2: svuint8_t, op3: u8) -> svuint16_t {
    svmlslb_u16(op1, op2, svdup_n_u8(op3))
}
#[doc = "Multiply-subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslb[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlslb))]
pub fn svmlslb_u32(op1: svuint32_t, op2: svuint16_t, op3: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.umlslb.nxv4i32")]
        fn _svmlslb_u32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svmlslb_u32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Multiply-subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslb[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlslb))]
pub fn svmlslb_n_u32(op1: svuint32_t, op2: svuint16_t, op3: u16) -> svuint32_t {
    svmlslb_u32(op1, op2, svdup_n_u16(op3))
}
#[doc = "Multiply-subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslb[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlslb))]
pub fn svmlslb_u64(op1: svuint64_t, op2: svuint32_t, op3: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.umlslb.nxv2i64")]
        fn _svmlslb_u64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svmlslb_u64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Multiply-subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslb[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlslb))]
pub fn svmlslb_n_u64(op1: svuint64_t, op2: svuint32_t, op3: u32) -> svuint64_t {
    svmlslb_u64(op1, op2, svdup_n_u32(op3))
}
#[doc = "Multiply-subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslt_lane[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlslt, IMM_INDEX = 0))]
pub fn svmlslt_lane_s32<const IMM_INDEX: i32>(
    op1: svint32_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint32_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.smlslt.lane.nxv4i32"
        )]
        fn _svmlslt_lane_s32(
            op1: svint32_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe { _svmlslt_lane_s32(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Multiply-subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslt_lane[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlslt, IMM_INDEX = 0))]
pub fn svmlslt_lane_s64<const IMM_INDEX: i32>(
    op1: svint64_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint64_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.smlslt.lane.nxv2i64"
        )]
        fn _svmlslt_lane_s64(
            op1: svint64_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe { _svmlslt_lane_s64(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Multiply-subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslt_lane[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlslt, IMM_INDEX = 0))]
pub fn svmlslt_lane_u32<const IMM_INDEX: i32>(
    op1: svuint32_t,
    op2: svuint16_t,
    op3: svuint16_t,
) -> svuint32_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.umlslt.lane.nxv4i32"
        )]
        fn _svmlslt_lane_u32(
            op1: svint32_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe {
        _svmlslt_lane_u32(op1.as_signed(), op2.as_signed(), op3.as_signed(), IMM_INDEX)
            .as_unsigned()
    }
}
#[doc = "Multiply-subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslt_lane[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlslt, IMM_INDEX = 0))]
pub fn svmlslt_lane_u64<const IMM_INDEX: i32>(
    op1: svuint64_t,
    op2: svuint32_t,
    op3: svuint32_t,
) -> svuint64_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.umlslt.lane.nxv2i64"
        )]
        fn _svmlslt_lane_u64(
            op1: svint64_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe {
        _svmlslt_lane_u64(op1.as_signed(), op2.as_signed(), op3.as_signed(), IMM_INDEX)
            .as_unsigned()
    }
}
#[doc = "Multiply-subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslt[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlslt))]
pub fn svmlslt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.smlslt.nxv8i16")]
        fn _svmlslt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svmlslt_s16(op1, op2, op3) }
}
#[doc = "Multiply-subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlslt))]
pub fn svmlslt_n_s16(op1: svint16_t, op2: svint8_t, op3: i8) -> svint16_t {
    svmlslt_s16(op1, op2, svdup_n_s8(op3))
}
#[doc = "Multiply-subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslt[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlslt))]
pub fn svmlslt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.smlslt.nxv4i32")]
        fn _svmlslt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svmlslt_s32(op1, op2, op3) }
}
#[doc = "Multiply-subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlslt))]
pub fn svmlslt_n_s32(op1: svint32_t, op2: svint16_t, op3: i16) -> svint32_t {
    svmlslt_s32(op1, op2, svdup_n_s16(op3))
}
#[doc = "Multiply-subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslt[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlslt))]
pub fn svmlslt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.smlslt.nxv2i64")]
        fn _svmlslt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svmlslt_s64(op1, op2, op3) }
}
#[doc = "Multiply-subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smlslt))]
pub fn svmlslt_n_s64(op1: svint64_t, op2: svint32_t, op3: i32) -> svint64_t {
    svmlslt_s64(op1, op2, svdup_n_s32(op3))
}
#[doc = "Multiply-subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslt[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlslt))]
pub fn svmlslt_u16(op1: svuint16_t, op2: svuint8_t, op3: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.umlslt.nxv8i16")]
        fn _svmlslt_u16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svmlslt_u16(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Multiply-subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslt[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlslt))]
pub fn svmlslt_n_u16(op1: svuint16_t, op2: svuint8_t, op3: u8) -> svuint16_t {
    svmlslt_u16(op1, op2, svdup_n_u8(op3))
}
#[doc = "Multiply-subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslt[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlslt))]
pub fn svmlslt_u32(op1: svuint32_t, op2: svuint16_t, op3: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.umlslt.nxv4i32")]
        fn _svmlslt_u32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svmlslt_u32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Multiply-subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslt[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlslt))]
pub fn svmlslt_n_u32(op1: svuint32_t, op2: svuint16_t, op3: u16) -> svuint32_t {
    svmlslt_u32(op1, op2, svdup_n_u16(op3))
}
#[doc = "Multiply-subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslt[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlslt))]
pub fn svmlslt_u64(op1: svuint64_t, op2: svuint32_t, op3: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.umlslt.nxv2i64")]
        fn _svmlslt_u64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svmlslt_u64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Multiply-subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmlslt[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umlslt))]
pub fn svmlslt_n_u64(op1: svuint64_t, op2: svuint32_t, op3: u32) -> svuint64_t {
    svmlslt_u64(op1, op2, svdup_n_u32(op3))
}
#[doc = "Move long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmovlb[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sshllb))]
pub fn svmovlb_s16(op: svint8_t) -> svint16_t {
    svshllb_n_s16::<0>(op)
}
#[doc = "Move long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmovlb[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sshllb))]
pub fn svmovlb_s32(op: svint16_t) -> svint32_t {
    svshllb_n_s32::<0>(op)
}
#[doc = "Move long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmovlb[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sshllb))]
pub fn svmovlb_s64(op: svint32_t) -> svint64_t {
    svshllb_n_s64::<0>(op)
}
#[doc = "Move long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmovlb[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ushllb))]
pub fn svmovlb_u16(op: svuint8_t) -> svuint16_t {
    svshllb_n_u16::<0>(op)
}
#[doc = "Move long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmovlb[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ushllb))]
pub fn svmovlb_u32(op: svuint16_t) -> svuint32_t {
    svshllb_n_u32::<0>(op)
}
#[doc = "Move long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmovlb[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ushllb))]
pub fn svmovlb_u64(op: svuint32_t) -> svuint64_t {
    svshllb_n_u64::<0>(op)
}
#[doc = "Move long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmovlt[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sshllt))]
pub fn svmovlt_s16(op: svint8_t) -> svint16_t {
    svshllt_n_s16::<0>(op)
}
#[doc = "Move long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmovlt[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sshllt))]
pub fn svmovlt_s32(op: svint16_t) -> svint32_t {
    svshllt_n_s32::<0>(op)
}
#[doc = "Move long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmovlt[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sshllt))]
pub fn svmovlt_s64(op: svint32_t) -> svint64_t {
    svshllt_n_s64::<0>(op)
}
#[doc = "Move long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmovlt[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ushllt))]
pub fn svmovlt_u16(op: svuint8_t) -> svuint16_t {
    svshllt_n_u16::<0>(op)
}
#[doc = "Move long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmovlt[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ushllt))]
pub fn svmovlt_u32(op: svuint16_t) -> svuint32_t {
    svshllt_n_u32::<0>(op)
}
#[doc = "Move long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmovlt[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ushllt))]
pub fn svmovlt_u64(op: svuint32_t) -> svuint64_t {
    svshllt_n_u64::<0>(op)
}
#[doc = "Multiply"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmul_lane[_f32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(fmul, IMM_INDEX = 0))]
pub fn svmul_lane_f32<const IMM_INDEX: i32>(op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.fmul.lane.nxv4f32"
        )]
        fn _svmul_lane_f32(op1: svfloat32_t, op2: svfloat32_t, imm_index: i32) -> svfloat32_t;
    }
    unsafe { _svmul_lane_f32(op1, op2, IMM_INDEX) }
}
#[doc = "Multiply"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmul_lane[_f64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(fmul, IMM_INDEX = 0))]
pub fn svmul_lane_f64<const IMM_INDEX: i32>(op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t {
    static_assert_range!(IMM_INDEX, 0, 1);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.fmul.lane.nxv2f64"
        )]
        fn _svmul_lane_f64(op1: svfloat64_t, op2: svfloat64_t, imm_index: i32) -> svfloat64_t;
    }
    unsafe { _svmul_lane_f64(op1, op2, IMM_INDEX) }
}
#[doc = "Multiply"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmul_lane[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(mul, IMM_INDEX = 0))]
pub fn svmul_lane_s16<const IMM_INDEX: i32>(op1: svint16_t, op2: svint16_t) -> svint16_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.mul.lane.nxv8i16"
        )]
        fn _svmul_lane_s16(op1: svint16_t, op2: svint16_t, imm_index: i32) -> svint16_t;
    }
    unsafe { _svmul_lane_s16(op1, op2, IMM_INDEX) }
}
#[doc = "Multiply"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmul_lane[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(mul, IMM_INDEX = 0))]
pub fn svmul_lane_s32<const IMM_INDEX: i32>(op1: svint32_t, op2: svint32_t) -> svint32_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.mul.lane.nxv4i32"
        )]
        fn _svmul_lane_s32(op1: svint32_t, op2: svint32_t, imm_index: i32) -> svint32_t;
    }
    unsafe { _svmul_lane_s32(op1, op2, IMM_INDEX) }
}
#[doc = "Multiply"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmul_lane[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(mul, IMM_INDEX = 0))]
pub fn svmul_lane_s64<const IMM_INDEX: i32>(op1: svint64_t, op2: svint64_t) -> svint64_t {
    static_assert_range!(IMM_INDEX, 0, 1);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.mul.lane.nxv2i64"
        )]
        fn _svmul_lane_s64(op1: svint64_t, op2: svint64_t, imm_index: i32) -> svint64_t;
    }
    unsafe { _svmul_lane_s64(op1, op2, IMM_INDEX) }
}
#[doc = "Multiply"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmul_lane[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(mul, IMM_INDEX = 0))]
pub fn svmul_lane_u16<const IMM_INDEX: i32>(op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    unsafe { svmul_lane_s16::<IMM_INDEX>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Multiply"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmul_lane[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(mul, IMM_INDEX = 0))]
pub fn svmul_lane_u32<const IMM_INDEX: i32>(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    unsafe { svmul_lane_s32::<IMM_INDEX>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Multiply"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmul_lane[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(mul, IMM_INDEX = 0))]
pub fn svmul_lane_u64<const IMM_INDEX: i32>(op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    static_assert_range!(IMM_INDEX, 0, 1);
    unsafe { svmul_lane_s64::<IMM_INDEX>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullb_lane[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smullb, IMM_INDEX = 0))]
pub fn svmullb_lane_s32<const IMM_INDEX: i32>(op1: svint16_t, op2: svint16_t) -> svint32_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.smullb.lane.nxv4i32"
        )]
        fn _svmullb_lane_s32(op1: svint16_t, op2: svint16_t, imm_index: i32) -> svint32_t;
    }
    unsafe { _svmullb_lane_s32(op1, op2, IMM_INDEX) }
}
#[doc = "Multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullb_lane[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smullb, IMM_INDEX = 0))]
pub fn svmullb_lane_s64<const IMM_INDEX: i32>(op1: svint32_t, op2: svint32_t) -> svint64_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.smullb.lane.nxv2i64"
        )]
        fn _svmullb_lane_s64(op1: svint32_t, op2: svint32_t, imm_index: i32) -> svint64_t;
    }
    unsafe { _svmullb_lane_s64(op1, op2, IMM_INDEX) }
}
#[doc = "Multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullb_lane[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umullb, IMM_INDEX = 0))]
pub fn svmullb_lane_u32<const IMM_INDEX: i32>(op1: svuint16_t, op2: svuint16_t) -> svuint32_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.umullb.lane.nxv4i32"
        )]
        fn _svmullb_lane_u32(op1: svint16_t, op2: svint16_t, imm_index: i32) -> svint32_t;
    }
    unsafe { _svmullb_lane_u32(op1.as_signed(), op2.as_signed(), IMM_INDEX).as_unsigned() }
}
#[doc = "Multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullb_lane[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umullb, IMM_INDEX = 0))]
pub fn svmullb_lane_u64<const IMM_INDEX: i32>(op1: svuint32_t, op2: svuint32_t) -> svuint64_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.umullb.lane.nxv2i64"
        )]
        fn _svmullb_lane_u64(op1: svint32_t, op2: svint32_t, imm_index: i32) -> svint64_t;
    }
    unsafe { _svmullb_lane_u64(op1.as_signed(), op2.as_signed(), IMM_INDEX).as_unsigned() }
}
#[doc = "Multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullb[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smullb))]
pub fn svmullb_s16(op1: svint8_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.smullb.nxv8i16")]
        fn _svmullb_s16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svmullb_s16(op1, op2) }
}
#[doc = "Multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullb[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smullb))]
pub fn svmullb_n_s16(op1: svint8_t, op2: i8) -> svint16_t {
    svmullb_s16(op1, svdup_n_s8(op2))
}
#[doc = "Multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullb[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smullb))]
pub fn svmullb_s32(op1: svint16_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.smullb.nxv4i32")]
        fn _svmullb_s32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svmullb_s32(op1, op2) }
}
#[doc = "Multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullb[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smullb))]
pub fn svmullb_n_s32(op1: svint16_t, op2: i16) -> svint32_t {
    svmullb_s32(op1, svdup_n_s16(op2))
}
#[doc = "Multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullb[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smullb))]
pub fn svmullb_s64(op1: svint32_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.smullb.nxv2i64")]
        fn _svmullb_s64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svmullb_s64(op1, op2) }
}
#[doc = "Multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullb[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smullb))]
pub fn svmullb_n_s64(op1: svint32_t, op2: i32) -> svint64_t {
    svmullb_s64(op1, svdup_n_s32(op2))
}
#[doc = "Multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullb[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umullb))]
pub fn svmullb_u16(op1: svuint8_t, op2: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.umullb.nxv8i16")]
        fn _svmullb_u16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svmullb_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullb[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umullb))]
pub fn svmullb_n_u16(op1: svuint8_t, op2: u8) -> svuint16_t {
    svmullb_u16(op1, svdup_n_u8(op2))
}
#[doc = "Multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullb[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umullb))]
pub fn svmullb_u32(op1: svuint16_t, op2: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.umullb.nxv4i32")]
        fn _svmullb_u32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svmullb_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullb[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umullb))]
pub fn svmullb_n_u32(op1: svuint16_t, op2: u16) -> svuint32_t {
    svmullb_u32(op1, svdup_n_u16(op2))
}
#[doc = "Multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullb[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umullb))]
pub fn svmullb_u64(op1: svuint32_t, op2: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.umullb.nxv2i64")]
        fn _svmullb_u64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svmullb_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullb[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umullb))]
pub fn svmullb_n_u64(op1: svuint32_t, op2: u32) -> svuint64_t {
    svmullb_u64(op1, svdup_n_u32(op2))
}
#[doc = "Multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullt_lane[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smullt, IMM_INDEX = 0))]
pub fn svmullt_lane_s32<const IMM_INDEX: i32>(op1: svint16_t, op2: svint16_t) -> svint32_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.smullt.lane.nxv4i32"
        )]
        fn _svmullt_lane_s32(op1: svint16_t, op2: svint16_t, imm_index: i32) -> svint32_t;
    }
    unsafe { _svmullt_lane_s32(op1, op2, IMM_INDEX) }
}
#[doc = "Multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullt_lane[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smullt, IMM_INDEX = 0))]
pub fn svmullt_lane_s64<const IMM_INDEX: i32>(op1: svint32_t, op2: svint32_t) -> svint64_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.smullt.lane.nxv2i64"
        )]
        fn _svmullt_lane_s64(op1: svint32_t, op2: svint32_t, imm_index: i32) -> svint64_t;
    }
    unsafe { _svmullt_lane_s64(op1, op2, IMM_INDEX) }
}
#[doc = "Multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullt_lane[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umullt, IMM_INDEX = 0))]
pub fn svmullt_lane_u32<const IMM_INDEX: i32>(op1: svuint16_t, op2: svuint16_t) -> svuint32_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.umullt.lane.nxv4i32"
        )]
        fn _svmullt_lane_u32(op1: svint16_t, op2: svint16_t, imm_index: i32) -> svint32_t;
    }
    unsafe { _svmullt_lane_u32(op1.as_signed(), op2.as_signed(), IMM_INDEX).as_unsigned() }
}
#[doc = "Multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullt_lane[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umullt, IMM_INDEX = 0))]
pub fn svmullt_lane_u64<const IMM_INDEX: i32>(op1: svuint32_t, op2: svuint32_t) -> svuint64_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.umullt.lane.nxv2i64"
        )]
        fn _svmullt_lane_u64(op1: svint32_t, op2: svint32_t, imm_index: i32) -> svint64_t;
    }
    unsafe { _svmullt_lane_u64(op1.as_signed(), op2.as_signed(), IMM_INDEX).as_unsigned() }
}
#[doc = "Multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullt[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smullt))]
pub fn svmullt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.smullt.nxv8i16")]
        fn _svmullt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svmullt_s16(op1, op2) }
}
#[doc = "Multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smullt))]
pub fn svmullt_n_s16(op1: svint8_t, op2: i8) -> svint16_t {
    svmullt_s16(op1, svdup_n_s8(op2))
}
#[doc = "Multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullt[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smullt))]
pub fn svmullt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.smullt.nxv4i32")]
        fn _svmullt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svmullt_s32(op1, op2) }
}
#[doc = "Multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smullt))]
pub fn svmullt_n_s32(op1: svint16_t, op2: i16) -> svint32_t {
    svmullt_s32(op1, svdup_n_s16(op2))
}
#[doc = "Multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullt[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smullt))]
pub fn svmullt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.smullt.nxv2i64")]
        fn _svmullt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svmullt_s64(op1, op2) }
}
#[doc = "Multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(smullt))]
pub fn svmullt_n_s64(op1: svint32_t, op2: i32) -> svint64_t {
    svmullt_s64(op1, svdup_n_s32(op2))
}
#[doc = "Multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullt[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umullt))]
pub fn svmullt_u16(op1: svuint8_t, op2: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.umullt.nxv8i16")]
        fn _svmullt_u16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svmullt_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullt[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umullt))]
pub fn svmullt_n_u16(op1: svuint8_t, op2: u8) -> svuint16_t {
    svmullt_u16(op1, svdup_n_u8(op2))
}
#[doc = "Multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullt[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umullt))]
pub fn svmullt_u32(op1: svuint16_t, op2: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.umullt.nxv4i32")]
        fn _svmullt_u32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svmullt_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullt[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umullt))]
pub fn svmullt_n_u32(op1: svuint16_t, op2: u16) -> svuint32_t {
    svmullt_u32(op1, svdup_n_u16(op2))
}
#[doc = "Multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullt[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umullt))]
pub fn svmullt_u64(op1: svuint32_t, op2: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.umullt.nxv2i64")]
        fn _svmullt_u64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svmullt_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svmullt[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(umullt))]
pub fn svmullt_n_u64(op1: svuint32_t, op2: u32) -> svuint64_t {
    svmullt_u64(op1, svdup_n_u32(op2))
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svnbsl[_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(nbsl))]
pub fn svnbsl_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.nbsl.nxv16i8")]
        fn _svnbsl_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t;
    }
    unsafe { _svnbsl_s8(op1, op2, op3) }
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svnbsl[_n_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(nbsl))]
pub fn svnbsl_n_s8(op1: svint8_t, op2: svint8_t, op3: i8) -> svint8_t {
    svnbsl_s8(op1, op2, svdup_n_s8(op3))
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svnbsl[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(nbsl))]
pub fn svnbsl_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.nbsl.nxv8i16")]
        fn _svnbsl_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t;
    }
    unsafe { _svnbsl_s16(op1, op2, op3) }
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svnbsl[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(nbsl))]
pub fn svnbsl_n_s16(op1: svint16_t, op2: svint16_t, op3: i16) -> svint16_t {
    svnbsl_s16(op1, op2, svdup_n_s16(op3))
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svnbsl[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(nbsl))]
pub fn svnbsl_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.nbsl.nxv4i32")]
        fn _svnbsl_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _svnbsl_s32(op1, op2, op3) }
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svnbsl[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(nbsl))]
pub fn svnbsl_n_s32(op1: svint32_t, op2: svint32_t, op3: i32) -> svint32_t {
    svnbsl_s32(op1, op2, svdup_n_s32(op3))
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svnbsl[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(nbsl))]
pub fn svnbsl_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.nbsl.nxv2i64")]
        fn _svnbsl_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _svnbsl_s64(op1, op2, op3) }
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svnbsl[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(nbsl))]
pub fn svnbsl_n_s64(op1: svint64_t, op2: svint64_t, op3: i64) -> svint64_t {
    svnbsl_s64(op1, op2, svdup_n_s64(op3))
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svnbsl[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(nbsl))]
pub fn svnbsl_u8(op1: svuint8_t, op2: svuint8_t, op3: svuint8_t) -> svuint8_t {
    unsafe { svnbsl_s8(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svnbsl[_n_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(nbsl))]
pub fn svnbsl_n_u8(op1: svuint8_t, op2: svuint8_t, op3: u8) -> svuint8_t {
    svnbsl_u8(op1, op2, svdup_n_u8(op3))
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svnbsl[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(nbsl))]
pub fn svnbsl_u16(op1: svuint16_t, op2: svuint16_t, op3: svuint16_t) -> svuint16_t {
    unsafe { svnbsl_s16(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svnbsl[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(nbsl))]
pub fn svnbsl_n_u16(op1: svuint16_t, op2: svuint16_t, op3: u16) -> svuint16_t {
    svnbsl_u16(op1, op2, svdup_n_u16(op3))
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svnbsl[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(nbsl))]
pub fn svnbsl_u32(op1: svuint32_t, op2: svuint32_t, op3: svuint32_t) -> svuint32_t {
    unsafe { svnbsl_s32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svnbsl[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(nbsl))]
pub fn svnbsl_n_u32(op1: svuint32_t, op2: svuint32_t, op3: u32) -> svuint32_t {
    svnbsl_u32(op1, op2, svdup_n_u32(op3))
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svnbsl[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(nbsl))]
pub fn svnbsl_u64(op1: svuint64_t, op2: svuint64_t, op3: svuint64_t) -> svuint64_t {
    unsafe { svnbsl_s64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Bitwise select"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svnbsl[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(nbsl))]
pub fn svnbsl_n_u64(op1: svuint64_t, op2: svuint64_t, op3: u64) -> svuint64_t {
    svnbsl_u64(op1, op2, svdup_n_u64(op3))
}
#[doc = "Detect no matching elements"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svnmatch[_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(nmatch))]
pub fn svnmatch_s8(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svbool_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.nmatch.nxv16i8")]
        fn _svnmatch_s8(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svbool_t;
    }
    unsafe { _svnmatch_s8(pg, op1, op2) }
}
#[doc = "Detect no matching elements"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svnmatch[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(nmatch))]
pub fn svnmatch_s16(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svbool_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.nmatch.nxv8i16")]
        fn _svnmatch_s16(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svbool8_t;
    }
    unsafe { _svnmatch_s16(pg.into(), op1, op2).into() }
}
#[doc = "Detect no matching elements"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svnmatch[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(nmatch))]
pub fn svnmatch_u8(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svbool_t {
    unsafe { svnmatch_s8(pg, op1.as_signed(), op2.as_signed()) }
}
#[doc = "Detect no matching elements"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svnmatch[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(nmatch))]
pub fn svnmatch_u16(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svbool_t {
    unsafe { svnmatch_s16(pg, op1.as_signed(), op2.as_signed()) }
}
#[doc = "Polynomial multiply"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svpmul[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(pmul))]
pub fn svpmul_u8(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.pmul.nxv16i8")]
        fn _svpmul_u8(op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svpmul_u8(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Polynomial multiply"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svpmul[_n_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(pmul))]
pub fn svpmul_n_u8(op1: svuint8_t, op2: u8) -> svuint8_t {
    svpmul_u8(op1, svdup_n_u8(op2))
}
#[doc = "Polynomial multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svpmullb_pair[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
#[cfg_attr(test, assert_instr(pmullb))]
pub fn svpmullb_pair_u8(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.pmullb.pair.nxv16i8"
        )]
        fn _svpmullb_pair_u8(op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svpmullb_pair_u8(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Polynomial multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svpmullb_pair[_n_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
#[cfg_attr(test, assert_instr(pmullb))]
pub fn svpmullb_pair_n_u8(op1: svuint8_t, op2: u8) -> svuint8_t {
    svpmullb_pair_u8(op1, svdup_n_u8(op2))
}
#[doc = "Polynomial multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svpmullb_pair[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
#[cfg_attr(test, assert_instr(pmullb))]
pub fn svpmullb_pair_u32(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.pmullb.pair.nxv4i32"
        )]
        fn _svpmullb_pair_u32(op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svpmullb_pair_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Polynomial multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svpmullb_pair[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
#[cfg_attr(test, assert_instr(pmullb))]
pub fn svpmullb_pair_n_u32(op1: svuint32_t, op2: u32) -> svuint32_t {
    svpmullb_pair_u32(op1, svdup_n_u32(op2))
}
#[doc = "Polynomial multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svpmullb_pair[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
#[cfg_attr(test, assert_instr(pmullb))]
pub fn svpmullb_pair_u64(op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.pmullb.pair.nxv2i64"
        )]
        fn _svpmullb_pair_u64(op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svpmullb_pair_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Polynomial multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svpmullb_pair[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
#[cfg_attr(test, assert_instr(pmullb))]
pub fn svpmullb_pair_n_u64(op1: svuint64_t, op2: u64) -> svuint64_t {
    svpmullb_pair_u64(op1, svdup_n_u64(op2))
}
#[doc = "Polynomial multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svpmullb[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
#[cfg_attr(test, assert_instr(pmullb))]
pub fn svpmullb_u16(op1: svuint8_t, op2: svuint8_t) -> svuint16_t {
    unsafe { simd_reinterpret(svpmullb_pair_u8(op1, op2)) }
}
#[doc = "Polynomial multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svpmullb[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
#[cfg_attr(test, assert_instr(pmullb))]
pub fn svpmullb_n_u16(op1: svuint8_t, op2: u8) -> svuint16_t {
    svpmullb_u16(op1, svdup_n_u8(op2))
}
#[doc = "Polynomial multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svpmullb[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
#[cfg_attr(test, assert_instr(pmullb))]
pub fn svpmullb_u64(op1: svuint32_t, op2: svuint32_t) -> svuint64_t {
    unsafe { simd_reinterpret(svpmullb_pair_u32(op1, op2)) }
}
#[doc = "Polynomial multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svpmullb[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
#[cfg_attr(test, assert_instr(pmullb))]
pub fn svpmullb_n_u64(op1: svuint32_t, op2: u32) -> svuint64_t {
    svpmullb_u64(op1, svdup_n_u32(op2))
}
#[doc = "Polynomial multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svpmullt_pair[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
#[cfg_attr(test, assert_instr(pmullt))]
pub fn svpmullt_pair_u8(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.pmullt.pair.nxv16i8"
        )]
        fn _svpmullt_pair_u8(op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svpmullt_pair_u8(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Polynomial multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svpmullt_pair[_n_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
#[cfg_attr(test, assert_instr(pmullt))]
pub fn svpmullt_pair_n_u8(op1: svuint8_t, op2: u8) -> svuint8_t {
    svpmullt_pair_u8(op1, svdup_n_u8(op2))
}
#[doc = "Polynomial multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svpmullt_pair[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
#[cfg_attr(test, assert_instr(pmullt))]
pub fn svpmullt_pair_u32(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.pmullt.pair.nxv4i32"
        )]
        fn _svpmullt_pair_u32(op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svpmullt_pair_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Polynomial multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svpmullt_pair[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
#[cfg_attr(test, assert_instr(pmullt))]
pub fn svpmullt_pair_n_u32(op1: svuint32_t, op2: u32) -> svuint32_t {
    svpmullt_pair_u32(op1, svdup_n_u32(op2))
}
#[doc = "Polynomial multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svpmullt_pair[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
#[cfg_attr(test, assert_instr(pmullt))]
pub fn svpmullt_pair_u64(op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.pmullt.pair.nxv2i64"
        )]
        fn _svpmullt_pair_u64(op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svpmullt_pair_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Polynomial multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svpmullt_pair[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
#[cfg_attr(test, assert_instr(pmullt))]
pub fn svpmullt_pair_n_u64(op1: svuint64_t, op2: u64) -> svuint64_t {
    svpmullt_pair_u64(op1, svdup_n_u64(op2))
}
#[doc = "Polynomial multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svpmullt[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
#[cfg_attr(test, assert_instr(pmullt))]
pub fn svpmullt_u16(op1: svuint8_t, op2: svuint8_t) -> svuint16_t {
    unsafe { simd_reinterpret(svpmullt_pair_u8(op1, op2)) }
}
#[doc = "Polynomial multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svpmullt[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
#[cfg_attr(test, assert_instr(pmullt))]
pub fn svpmullt_n_u16(op1: svuint8_t, op2: u8) -> svuint16_t {
    svpmullt_u16(op1, svdup_n_u8(op2))
}
#[doc = "Polynomial multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svpmullt[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
#[cfg_attr(test, assert_instr(pmullt))]
pub fn svpmullt_u64(op1: svuint32_t, op2: svuint32_t) -> svuint64_t {
    unsafe { simd_reinterpret(svpmullt_pair_u32(op1, op2)) }
}
#[doc = "Polynomial multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svpmullt[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
#[cfg_attr(test, assert_instr(pmullt))]
pub fn svpmullt_n_u64(op1: svuint32_t, op2: u32) -> svuint64_t {
    svpmullt_u64(op1, svdup_n_u32(op2))
}
#[doc = "Saturating absolute value"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqabs[_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqabs))]
pub fn svqabs_s8_m(inactive: svint8_t, pg: svbool_t, op: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqabs.nxv16i8")]
        fn _svqabs_s8_m(inactive: svint8_t, pg: svbool_t, op: svint8_t) -> svint8_t;
    }
    unsafe { _svqabs_s8_m(inactive, pg, op) }
}
#[doc = "Saturating absolute value"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqabs[_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqabs))]
pub fn svqabs_s8_x(pg: svbool_t, op: svint8_t) -> svint8_t {
    svqabs_s8_m(op, pg, op)
}
#[doc = "Saturating absolute value"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqabs[_s8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqabs))]
pub fn svqabs_s8_z(pg: svbool_t, op: svint8_t) -> svint8_t {
    svqabs_s8_m(svdup_n_s8(0), pg, op)
}
#[doc = "Saturating absolute value"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqabs[_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqabs))]
pub fn svqabs_s16_m(inactive: svint16_t, pg: svbool_t, op: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqabs.nxv8i16")]
        fn _svqabs_s16_m(inactive: svint16_t, pg: svbool8_t, op: svint16_t) -> svint16_t;
    }
    unsafe { _svqabs_s16_m(inactive, pg.into(), op) }
}
#[doc = "Saturating absolute value"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqabs[_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqabs))]
pub fn svqabs_s16_x(pg: svbool_t, op: svint16_t) -> svint16_t {
    svqabs_s16_m(op, pg, op)
}
#[doc = "Saturating absolute value"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqabs[_s16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqabs))]
pub fn svqabs_s16_z(pg: svbool_t, op: svint16_t) -> svint16_t {
    svqabs_s16_m(svdup_n_s16(0), pg, op)
}
#[doc = "Saturating absolute value"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqabs[_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqabs))]
pub fn svqabs_s32_m(inactive: svint32_t, pg: svbool_t, op: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqabs.nxv4i32")]
        fn _svqabs_s32_m(inactive: svint32_t, pg: svbool4_t, op: svint32_t) -> svint32_t;
    }
    unsafe { _svqabs_s32_m(inactive, pg.into(), op) }
}
#[doc = "Saturating absolute value"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqabs[_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqabs))]
pub fn svqabs_s32_x(pg: svbool_t, op: svint32_t) -> svint32_t {
    svqabs_s32_m(op, pg, op)
}
#[doc = "Saturating absolute value"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqabs[_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqabs))]
pub fn svqabs_s32_z(pg: svbool_t, op: svint32_t) -> svint32_t {
    svqabs_s32_m(svdup_n_s32(0), pg, op)
}
#[doc = "Saturating absolute value"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqabs[_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqabs))]
pub fn svqabs_s64_m(inactive: svint64_t, pg: svbool_t, op: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqabs.nxv2i64")]
        fn _svqabs_s64_m(inactive: svint64_t, pg: svbool2_t, op: svint64_t) -> svint64_t;
    }
    unsafe { _svqabs_s64_m(inactive, pg.into(), op) }
}
#[doc = "Saturating absolute value"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqabs[_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqabs))]
pub fn svqabs_s64_x(pg: svbool_t, op: svint64_t) -> svint64_t {
    svqabs_s64_m(op, pg, op)
}
#[doc = "Saturating absolute value"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqabs[_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqabs))]
pub fn svqabs_s64_z(pg: svbool_t, op: svint64_t) -> svint64_t {
    svqabs_s64_m(svdup_n_s64(0), pg, op)
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqadd))]
pub fn svqadd_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqadd.nxv16i8")]
        fn _svqadd_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svqadd_s8_m(pg, op1, op2) }
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_n_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqadd))]
pub fn svqadd_n_s8_m(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqadd_s8_m(pg, op1, svdup_n_s8(op2))
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqadd))]
pub fn svqadd_s8_x(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svqadd_s8_m(pg, op1, op2)
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_n_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqadd))]
pub fn svqadd_n_s8_x(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqadd_s8_x(pg, op1, svdup_n_s8(op2))
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_s8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqadd))]
pub fn svqadd_s8_z(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svqadd_s8_m(pg, svsel_s8(pg, op1, svdup_n_s8(0)), op2)
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_n_s8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqadd))]
pub fn svqadd_n_s8_z(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqadd_s8_z(pg, op1, svdup_n_s8(op2))
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqadd))]
pub fn svqadd_s16_m(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqadd.nxv8i16")]
        fn _svqadd_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svqadd_s16_m(pg.into(), op1, op2) }
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_n_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqadd))]
pub fn svqadd_n_s16_m(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqadd_s16_m(pg, op1, svdup_n_s16(op2))
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqadd))]
pub fn svqadd_s16_x(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svqadd_s16_m(pg, op1, op2)
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_n_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqadd))]
pub fn svqadd_n_s16_x(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqadd_s16_x(pg, op1, svdup_n_s16(op2))
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_s16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqadd))]
pub fn svqadd_s16_z(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svqadd_s16_m(pg, svsel_s16(pg, op1, svdup_n_s16(0)), op2)
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_n_s16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqadd))]
pub fn svqadd_n_s16_z(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqadd_s16_z(pg, op1, svdup_n_s16(op2))
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqadd))]
pub fn svqadd_s32_m(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqadd.nxv4i32")]
        fn _svqadd_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svqadd_s32_m(pg.into(), op1, op2) }
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_n_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqadd))]
pub fn svqadd_n_s32_m(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqadd_s32_m(pg, op1, svdup_n_s32(op2))
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqadd))]
pub fn svqadd_s32_x(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svqadd_s32_m(pg, op1, op2)
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_n_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqadd))]
pub fn svqadd_n_s32_x(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqadd_s32_x(pg, op1, svdup_n_s32(op2))
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqadd))]
pub fn svqadd_s32_z(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svqadd_s32_m(pg, svsel_s32(pg, op1, svdup_n_s32(0)), op2)
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_n_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqadd))]
pub fn svqadd_n_s32_z(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqadd_s32_z(pg, op1, svdup_n_s32(op2))
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqadd))]
pub fn svqadd_s64_m(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqadd.nxv2i64")]
        fn _svqadd_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svqadd_s64_m(pg.into(), op1, op2) }
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_n_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqadd))]
pub fn svqadd_n_s64_m(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqadd_s64_m(pg, op1, svdup_n_s64(op2))
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqadd))]
pub fn svqadd_s64_x(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svqadd_s64_m(pg, op1, op2)
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_n_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqadd))]
pub fn svqadd_n_s64_x(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqadd_s64_x(pg, op1, svdup_n_s64(op2))
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqadd))]
pub fn svqadd_s64_z(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svqadd_s64_m(pg, svsel_s64(pg, op1, svdup_n_s64(0)), op2)
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_n_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqadd))]
pub fn svqadd_n_s64_z(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqadd_s64_z(pg, op1, svdup_n_s64(op2))
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_u8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqadd))]
pub fn svqadd_u8_m(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uqadd.nxv16i8")]
        fn _svqadd_u8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svqadd_u8_m(pg, op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_n_u8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqadd))]
pub fn svqadd_n_u8_m(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svqadd_u8_m(pg, op1, svdup_n_u8(op2))
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_u8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqadd))]
pub fn svqadd_u8_x(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svqadd_u8_m(pg, op1, op2)
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_n_u8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqadd))]
pub fn svqadd_n_u8_x(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svqadd_u8_x(pg, op1, svdup_n_u8(op2))
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_u8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqadd))]
pub fn svqadd_u8_z(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svqadd_u8_m(pg, svsel_u8(pg, op1, svdup_n_u8(0)), op2)
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_n_u8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqadd))]
pub fn svqadd_n_u8_z(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svqadd_u8_z(pg, op1, svdup_n_u8(op2))
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_u16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqadd))]
pub fn svqadd_u16_m(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uqadd.nxv8i16")]
        fn _svqadd_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svqadd_u16_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_n_u16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqadd))]
pub fn svqadd_n_u16_m(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svqadd_u16_m(pg, op1, svdup_n_u16(op2))
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_u16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqadd))]
pub fn svqadd_u16_x(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svqadd_u16_m(pg, op1, op2)
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_n_u16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqadd))]
pub fn svqadd_n_u16_x(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svqadd_u16_x(pg, op1, svdup_n_u16(op2))
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_u16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqadd))]
pub fn svqadd_u16_z(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svqadd_u16_m(pg, svsel_u16(pg, op1, svdup_n_u16(0)), op2)
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_n_u16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqadd))]
pub fn svqadd_n_u16_z(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svqadd_u16_z(pg, op1, svdup_n_u16(op2))
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqadd))]
pub fn svqadd_u32_m(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uqadd.nxv4i32")]
        fn _svqadd_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svqadd_u32_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_n_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqadd))]
pub fn svqadd_n_u32_m(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svqadd_u32_m(pg, op1, svdup_n_u32(op2))
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqadd))]
pub fn svqadd_u32_x(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svqadd_u32_m(pg, op1, op2)
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_n_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqadd))]
pub fn svqadd_n_u32_x(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svqadd_u32_x(pg, op1, svdup_n_u32(op2))
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_u32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqadd))]
pub fn svqadd_u32_z(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svqadd_u32_m(pg, svsel_u32(pg, op1, svdup_n_u32(0)), op2)
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_n_u32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqadd))]
pub fn svqadd_n_u32_z(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svqadd_u32_z(pg, op1, svdup_n_u32(op2))
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_u64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqadd))]
pub fn svqadd_u64_m(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uqadd.nxv2i64")]
        fn _svqadd_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svqadd_u64_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_n_u64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqadd))]
pub fn svqadd_n_u64_m(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svqadd_u64_m(pg, op1, svdup_n_u64(op2))
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_u64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqadd))]
pub fn svqadd_u64_x(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svqadd_u64_m(pg, op1, op2)
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_n_u64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqadd))]
pub fn svqadd_n_u64_x(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svqadd_u64_x(pg, op1, svdup_n_u64(op2))
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_u64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqadd))]
pub fn svqadd_u64_z(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svqadd_u64_m(pg, svsel_u64(pg, op1, svdup_n_u64(0)), op2)
}
#[doc = "Saturating add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqadd[_n_u64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqadd))]
pub fn svqadd_n_u64_z(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svqadd_u64_z(pg, op1, svdup_n_u64(op2))
}
#[doc = "Saturating complex add with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqcadd[_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqcadd, IMM_ROTATION = 90))]
pub fn svqcadd_s8<const IMM_ROTATION: i32>(op1: svint8_t, op2: svint8_t) -> svint8_t {
    static_assert!(IMM_ROTATION == 90 || IMM_ROTATION == 270);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqcadd.x.nxv16i8"
        )]
        fn _svqcadd_s8(op1: svint8_t, op2: svint8_t, imm_rotation: i32) -> svint8_t;
    }
    unsafe { _svqcadd_s8(op1, op2, IMM_ROTATION) }
}
#[doc = "Saturating complex add with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqcadd[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqcadd, IMM_ROTATION = 90))]
pub fn svqcadd_s16<const IMM_ROTATION: i32>(op1: svint16_t, op2: svint16_t) -> svint16_t {
    static_assert!(IMM_ROTATION == 90 || IMM_ROTATION == 270);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqcadd.x.nxv8i16"
        )]
        fn _svqcadd_s16(op1: svint16_t, op2: svint16_t, imm_rotation: i32) -> svint16_t;
    }
    unsafe { _svqcadd_s16(op1, op2, IMM_ROTATION) }
}
#[doc = "Saturating complex add with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqcadd[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqcadd, IMM_ROTATION = 90))]
pub fn svqcadd_s32<const IMM_ROTATION: i32>(op1: svint32_t, op2: svint32_t) -> svint32_t {
    static_assert!(IMM_ROTATION == 90 || IMM_ROTATION == 270);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqcadd.x.nxv4i32"
        )]
        fn _svqcadd_s32(op1: svint32_t, op2: svint32_t, imm_rotation: i32) -> svint32_t;
    }
    unsafe { _svqcadd_s32(op1, op2, IMM_ROTATION) }
}
#[doc = "Saturating complex add with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqcadd[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqcadd, IMM_ROTATION = 90))]
pub fn svqcadd_s64<const IMM_ROTATION: i32>(op1: svint64_t, op2: svint64_t) -> svint64_t {
    static_assert!(IMM_ROTATION == 90 || IMM_ROTATION == 270);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqcadd.x.nxv2i64"
        )]
        fn _svqcadd_s64(op1: svint64_t, op2: svint64_t, imm_rotation: i32) -> svint64_t;
    }
    unsafe { _svqcadd_s64(op1, op2, IMM_ROTATION) }
}
#[doc = "Saturating doubling multiply-add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlalb_lane[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlalb, IMM_INDEX = 0))]
pub fn svqdmlalb_lane_s32<const IMM_INDEX: i32>(
    op1: svint32_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint32_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmlalb.lane.nxv4i32"
        )]
        fn _svqdmlalb_lane_s32(
            op1: svint32_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe { _svqdmlalb_lane_s32(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Saturating doubling multiply-add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlalb_lane[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlalb, IMM_INDEX = 0))]
pub fn svqdmlalb_lane_s64<const IMM_INDEX: i32>(
    op1: svint64_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint64_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmlalb.lane.nxv2i64"
        )]
        fn _svqdmlalb_lane_s64(
            op1: svint64_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe { _svqdmlalb_lane_s64(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Saturating doubling multiply-add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlalb[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlalb))]
pub fn svqdmlalb_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmlalb.nxv8i16"
        )]
        fn _svqdmlalb_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svqdmlalb_s16(op1, op2, op3) }
}
#[doc = "Saturating doubling multiply-add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlalb[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlalb))]
pub fn svqdmlalb_n_s16(op1: svint16_t, op2: svint8_t, op3: i8) -> svint16_t {
    svqdmlalb_s16(op1, op2, svdup_n_s8(op3))
}
#[doc = "Saturating doubling multiply-add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlalb[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlalb))]
pub fn svqdmlalb_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmlalb.nxv4i32"
        )]
        fn _svqdmlalb_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svqdmlalb_s32(op1, op2, op3) }
}
#[doc = "Saturating doubling multiply-add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlalb[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlalb))]
pub fn svqdmlalb_n_s32(op1: svint32_t, op2: svint16_t, op3: i16) -> svint32_t {
    svqdmlalb_s32(op1, op2, svdup_n_s16(op3))
}
#[doc = "Saturating doubling multiply-add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlalb[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlalb))]
pub fn svqdmlalb_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmlalb.nxv2i64"
        )]
        fn _svqdmlalb_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svqdmlalb_s64(op1, op2, op3) }
}
#[doc = "Saturating doubling multiply-add long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlalb[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlalb))]
pub fn svqdmlalb_n_s64(op1: svint64_t, op2: svint32_t, op3: i32) -> svint64_t {
    svqdmlalb_s64(op1, op2, svdup_n_s32(op3))
}
#[doc = "Saturating doubling multiply-add long (bottom × top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlalbt[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlalbt))]
pub fn svqdmlalbt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmlalbt.nxv8i16"
        )]
        fn _svqdmlalbt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svqdmlalbt_s16(op1, op2, op3) }
}
#[doc = "Saturating doubling multiply-add long (bottom × top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlalbt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlalbt))]
pub fn svqdmlalbt_n_s16(op1: svint16_t, op2: svint8_t, op3: i8) -> svint16_t {
    svqdmlalbt_s16(op1, op2, svdup_n_s8(op3))
}
#[doc = "Saturating doubling multiply-add long (bottom × top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlalbt[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlalbt))]
pub fn svqdmlalbt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmlalbt.nxv4i32"
        )]
        fn _svqdmlalbt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svqdmlalbt_s32(op1, op2, op3) }
}
#[doc = "Saturating doubling multiply-add long (bottom × top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlalbt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlalbt))]
pub fn svqdmlalbt_n_s32(op1: svint32_t, op2: svint16_t, op3: i16) -> svint32_t {
    svqdmlalbt_s32(op1, op2, svdup_n_s16(op3))
}
#[doc = "Saturating doubling multiply-add long (bottom × top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlalbt[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlalbt))]
pub fn svqdmlalbt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmlalbt.nxv2i64"
        )]
        fn _svqdmlalbt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svqdmlalbt_s64(op1, op2, op3) }
}
#[doc = "Saturating doubling multiply-add long (bottom × top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlalbt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlalbt))]
pub fn svqdmlalbt_n_s64(op1: svint64_t, op2: svint32_t, op3: i32) -> svint64_t {
    svqdmlalbt_s64(op1, op2, svdup_n_s32(op3))
}
#[doc = "Saturating doubling multiply-add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlalt_lane[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlalt, IMM_INDEX = 0))]
pub fn svqdmlalt_lane_s32<const IMM_INDEX: i32>(
    op1: svint32_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint32_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmlalt.lane.nxv4i32"
        )]
        fn _svqdmlalt_lane_s32(
            op1: svint32_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe { _svqdmlalt_lane_s32(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Saturating doubling multiply-add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlalt_lane[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlalt, IMM_INDEX = 0))]
pub fn svqdmlalt_lane_s64<const IMM_INDEX: i32>(
    op1: svint64_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint64_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmlalt.lane.nxv2i64"
        )]
        fn _svqdmlalt_lane_s64(
            op1: svint64_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe { _svqdmlalt_lane_s64(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Saturating doubling multiply-add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlalt[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlalt))]
pub fn svqdmlalt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmlalt.nxv8i16"
        )]
        fn _svqdmlalt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svqdmlalt_s16(op1, op2, op3) }
}
#[doc = "Saturating doubling multiply-add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlalt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlalt))]
pub fn svqdmlalt_n_s16(op1: svint16_t, op2: svint8_t, op3: i8) -> svint16_t {
    svqdmlalt_s16(op1, op2, svdup_n_s8(op3))
}
#[doc = "Saturating doubling multiply-add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlalt[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlalt))]
pub fn svqdmlalt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmlalt.nxv4i32"
        )]
        fn _svqdmlalt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svqdmlalt_s32(op1, op2, op3) }
}
#[doc = "Saturating doubling multiply-add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlalt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlalt))]
pub fn svqdmlalt_n_s32(op1: svint32_t, op2: svint16_t, op3: i16) -> svint32_t {
    svqdmlalt_s32(op1, op2, svdup_n_s16(op3))
}
#[doc = "Saturating doubling multiply-add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlalt[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlalt))]
pub fn svqdmlalt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmlalt.nxv2i64"
        )]
        fn _svqdmlalt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svqdmlalt_s64(op1, op2, op3) }
}
#[doc = "Saturating doubling multiply-add long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlalt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlalt))]
pub fn svqdmlalt_n_s64(op1: svint64_t, op2: svint32_t, op3: i32) -> svint64_t {
    svqdmlalt_s64(op1, op2, svdup_n_s32(op3))
}
#[doc = "Saturating doubling multiply-subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlslb_lane[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlslb, IMM_INDEX = 0))]
pub fn svqdmlslb_lane_s32<const IMM_INDEX: i32>(
    op1: svint32_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint32_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmlslb.lane.nxv4i32"
        )]
        fn _svqdmlslb_lane_s32(
            op1: svint32_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe { _svqdmlslb_lane_s32(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Saturating doubling multiply-subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlslb_lane[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlslb, IMM_INDEX = 0))]
pub fn svqdmlslb_lane_s64<const IMM_INDEX: i32>(
    op1: svint64_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint64_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmlslb.lane.nxv2i64"
        )]
        fn _svqdmlslb_lane_s64(
            op1: svint64_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe { _svqdmlslb_lane_s64(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Saturating doubling multiply-subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlslb[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlslb))]
pub fn svqdmlslb_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmlslb.nxv8i16"
        )]
        fn _svqdmlslb_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svqdmlslb_s16(op1, op2, op3) }
}
#[doc = "Saturating doubling multiply-subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlslb[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlslb))]
pub fn svqdmlslb_n_s16(op1: svint16_t, op2: svint8_t, op3: i8) -> svint16_t {
    svqdmlslb_s16(op1, op2, svdup_n_s8(op3))
}
#[doc = "Saturating doubling multiply-subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlslb[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlslb))]
pub fn svqdmlslb_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmlslb.nxv4i32"
        )]
        fn _svqdmlslb_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svqdmlslb_s32(op1, op2, op3) }
}
#[doc = "Saturating doubling multiply-subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlslb[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlslb))]
pub fn svqdmlslb_n_s32(op1: svint32_t, op2: svint16_t, op3: i16) -> svint32_t {
    svqdmlslb_s32(op1, op2, svdup_n_s16(op3))
}
#[doc = "Saturating doubling multiply-subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlslb[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlslb))]
pub fn svqdmlslb_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmlslb.nxv2i64"
        )]
        fn _svqdmlslb_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svqdmlslb_s64(op1, op2, op3) }
}
#[doc = "Saturating doubling multiply-subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlslb[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlslb))]
pub fn svqdmlslb_n_s64(op1: svint64_t, op2: svint32_t, op3: i32) -> svint64_t {
    svqdmlslb_s64(op1, op2, svdup_n_s32(op3))
}
#[doc = "Saturating doubling multiply-subtract long (bottom × top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlslbt[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlslbt))]
pub fn svqdmlslbt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmlslbt.nxv8i16"
        )]
        fn _svqdmlslbt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svqdmlslbt_s16(op1, op2, op3) }
}
#[doc = "Saturating doubling multiply-subtract long (bottom × top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlslbt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlslbt))]
pub fn svqdmlslbt_n_s16(op1: svint16_t, op2: svint8_t, op3: i8) -> svint16_t {
    svqdmlslbt_s16(op1, op2, svdup_n_s8(op3))
}
#[doc = "Saturating doubling multiply-subtract long (bottom × top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlslbt[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlslbt))]
pub fn svqdmlslbt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmlslbt.nxv4i32"
        )]
        fn _svqdmlslbt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svqdmlslbt_s32(op1, op2, op3) }
}
#[doc = "Saturating doubling multiply-subtract long (bottom × top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlslbt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlslbt))]
pub fn svqdmlslbt_n_s32(op1: svint32_t, op2: svint16_t, op3: i16) -> svint32_t {
    svqdmlslbt_s32(op1, op2, svdup_n_s16(op3))
}
#[doc = "Saturating doubling multiply-subtract long (bottom × top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlslbt[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlslbt))]
pub fn svqdmlslbt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmlslbt.nxv2i64"
        )]
        fn _svqdmlslbt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svqdmlslbt_s64(op1, op2, op3) }
}
#[doc = "Saturating doubling multiply-subtract long (bottom × top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlslbt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlslbt))]
pub fn svqdmlslbt_n_s64(op1: svint64_t, op2: svint32_t, op3: i32) -> svint64_t {
    svqdmlslbt_s64(op1, op2, svdup_n_s32(op3))
}
#[doc = "Saturating doubling multiply-subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlslt_lane[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlslt, IMM_INDEX = 0))]
pub fn svqdmlslt_lane_s32<const IMM_INDEX: i32>(
    op1: svint32_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint32_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmlslt.lane.nxv4i32"
        )]
        fn _svqdmlslt_lane_s32(
            op1: svint32_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe { _svqdmlslt_lane_s32(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Saturating doubling multiply-subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlslt_lane[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlslt, IMM_INDEX = 0))]
pub fn svqdmlslt_lane_s64<const IMM_INDEX: i32>(
    op1: svint64_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint64_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmlslt.lane.nxv2i64"
        )]
        fn _svqdmlslt_lane_s64(
            op1: svint64_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe { _svqdmlslt_lane_s64(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Saturating doubling multiply-subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlslt[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlslt))]
pub fn svqdmlslt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmlslt.nxv8i16"
        )]
        fn _svqdmlslt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svqdmlslt_s16(op1, op2, op3) }
}
#[doc = "Saturating doubling multiply-subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlslt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlslt))]
pub fn svqdmlslt_n_s16(op1: svint16_t, op2: svint8_t, op3: i8) -> svint16_t {
    svqdmlslt_s16(op1, op2, svdup_n_s8(op3))
}
#[doc = "Saturating doubling multiply-subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlslt[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlslt))]
pub fn svqdmlslt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmlslt.nxv4i32"
        )]
        fn _svqdmlslt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svqdmlslt_s32(op1, op2, op3) }
}
#[doc = "Saturating doubling multiply-subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlslt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlslt))]
pub fn svqdmlslt_n_s32(op1: svint32_t, op2: svint16_t, op3: i16) -> svint32_t {
    svqdmlslt_s32(op1, op2, svdup_n_s16(op3))
}
#[doc = "Saturating doubling multiply-subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlslt[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlslt))]
pub fn svqdmlslt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmlslt.nxv2i64"
        )]
        fn _svqdmlslt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svqdmlslt_s64(op1, op2, op3) }
}
#[doc = "Saturating doubling multiply-subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmlslt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmlslt))]
pub fn svqdmlslt_n_s64(op1: svint64_t, op2: svint32_t, op3: i32) -> svint64_t {
    svqdmlslt_s64(op1, op2, svdup_n_s32(op3))
}
#[doc = "Saturating doubling multiply high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmulh_lane[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmulh, IMM_INDEX = 0))]
pub fn svqdmulh_lane_s16<const IMM_INDEX: i32>(op1: svint16_t, op2: svint16_t) -> svint16_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmulh.lane.nxv8i16"
        )]
        fn _svqdmulh_lane_s16(op1: svint16_t, op2: svint16_t, imm_index: i32) -> svint16_t;
    }
    unsafe { _svqdmulh_lane_s16(op1, op2, IMM_INDEX) }
}
#[doc = "Saturating doubling multiply high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmulh_lane[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmulh, IMM_INDEX = 0))]
pub fn svqdmulh_lane_s32<const IMM_INDEX: i32>(op1: svint32_t, op2: svint32_t) -> svint32_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmulh.lane.nxv4i32"
        )]
        fn _svqdmulh_lane_s32(op1: svint32_t, op2: svint32_t, imm_index: i32) -> svint32_t;
    }
    unsafe { _svqdmulh_lane_s32(op1, op2, IMM_INDEX) }
}
#[doc = "Saturating doubling multiply high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmulh_lane[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmulh, IMM_INDEX = 0))]
pub fn svqdmulh_lane_s64<const IMM_INDEX: i32>(op1: svint64_t, op2: svint64_t) -> svint64_t {
    static_assert_range!(IMM_INDEX, 0, 1);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmulh.lane.nxv2i64"
        )]
        fn _svqdmulh_lane_s64(op1: svint64_t, op2: svint64_t, imm_index: i32) -> svint64_t;
    }
    unsafe { _svqdmulh_lane_s64(op1, op2, IMM_INDEX) }
}
#[doc = "Saturating doubling multiply high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmulh[_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmulh))]
pub fn svqdmulh_s8(op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmulh.nxv16i8"
        )]
        fn _svqdmulh_s8(op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svqdmulh_s8(op1, op2) }
}
#[doc = "Saturating doubling multiply high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmulh[_n_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmulh))]
pub fn svqdmulh_n_s8(op1: svint8_t, op2: i8) -> svint8_t {
    svqdmulh_s8(op1, svdup_n_s8(op2))
}
#[doc = "Saturating doubling multiply high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmulh[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmulh))]
pub fn svqdmulh_s16(op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmulh.nxv8i16"
        )]
        fn _svqdmulh_s16(op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svqdmulh_s16(op1, op2) }
}
#[doc = "Saturating doubling multiply high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmulh[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmulh))]
pub fn svqdmulh_n_s16(op1: svint16_t, op2: i16) -> svint16_t {
    svqdmulh_s16(op1, svdup_n_s16(op2))
}
#[doc = "Saturating doubling multiply high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmulh[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmulh))]
pub fn svqdmulh_s32(op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmulh.nxv4i32"
        )]
        fn _svqdmulh_s32(op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svqdmulh_s32(op1, op2) }
}
#[doc = "Saturating doubling multiply high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmulh[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmulh))]
pub fn svqdmulh_n_s32(op1: svint32_t, op2: i32) -> svint32_t {
    svqdmulh_s32(op1, svdup_n_s32(op2))
}
#[doc = "Saturating doubling multiply high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmulh[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmulh))]
pub fn svqdmulh_s64(op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmulh.nxv2i64"
        )]
        fn _svqdmulh_s64(op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svqdmulh_s64(op1, op2) }
}
#[doc = "Saturating doubling multiply high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmulh[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmulh))]
pub fn svqdmulh_n_s64(op1: svint64_t, op2: i64) -> svint64_t {
    svqdmulh_s64(op1, svdup_n_s64(op2))
}
#[doc = "Saturating doubling multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmullb_lane[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmullb, IMM_INDEX = 0))]
pub fn svqdmullb_lane_s32<const IMM_INDEX: i32>(op1: svint16_t, op2: svint16_t) -> svint32_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmullb.lane.nxv4i32"
        )]
        fn _svqdmullb_lane_s32(op1: svint16_t, op2: svint16_t, imm_index: i32) -> svint32_t;
    }
    unsafe { _svqdmullb_lane_s32(op1, op2, IMM_INDEX) }
}
#[doc = "Saturating doubling multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmullb_lane[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmullb, IMM_INDEX = 0))]
pub fn svqdmullb_lane_s64<const IMM_INDEX: i32>(op1: svint32_t, op2: svint32_t) -> svint64_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmullb.lane.nxv2i64"
        )]
        fn _svqdmullb_lane_s64(op1: svint32_t, op2: svint32_t, imm_index: i32) -> svint64_t;
    }
    unsafe { _svqdmullb_lane_s64(op1, op2, IMM_INDEX) }
}
#[doc = "Saturating doubling multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmullb[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmullb))]
pub fn svqdmullb_s16(op1: svint8_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmullb.nxv8i16"
        )]
        fn _svqdmullb_s16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svqdmullb_s16(op1, op2) }
}
#[doc = "Saturating doubling multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmullb[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmullb))]
pub fn svqdmullb_n_s16(op1: svint8_t, op2: i8) -> svint16_t {
    svqdmullb_s16(op1, svdup_n_s8(op2))
}
#[doc = "Saturating doubling multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmullb[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmullb))]
pub fn svqdmullb_s32(op1: svint16_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmullb.nxv4i32"
        )]
        fn _svqdmullb_s32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svqdmullb_s32(op1, op2) }
}
#[doc = "Saturating doubling multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmullb[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmullb))]
pub fn svqdmullb_n_s32(op1: svint16_t, op2: i16) -> svint32_t {
    svqdmullb_s32(op1, svdup_n_s16(op2))
}
#[doc = "Saturating doubling multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmullb[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmullb))]
pub fn svqdmullb_s64(op1: svint32_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmullb.nxv2i64"
        )]
        fn _svqdmullb_s64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svqdmullb_s64(op1, op2) }
}
#[doc = "Saturating doubling multiply long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmullb[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmullb))]
pub fn svqdmullb_n_s64(op1: svint32_t, op2: i32) -> svint64_t {
    svqdmullb_s64(op1, svdup_n_s32(op2))
}
#[doc = "Saturating doubling multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmullt_lane[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmullt, IMM_INDEX = 0))]
pub fn svqdmullt_lane_s32<const IMM_INDEX: i32>(op1: svint16_t, op2: svint16_t) -> svint32_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmullt.lane.nxv4i32"
        )]
        fn _svqdmullt_lane_s32(op1: svint16_t, op2: svint16_t, imm_index: i32) -> svint32_t;
    }
    unsafe { _svqdmullt_lane_s32(op1, op2, IMM_INDEX) }
}
#[doc = "Saturating doubling multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmullt_lane[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmullt, IMM_INDEX = 0))]
pub fn svqdmullt_lane_s64<const IMM_INDEX: i32>(op1: svint32_t, op2: svint32_t) -> svint64_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmullt.lane.nxv2i64"
        )]
        fn _svqdmullt_lane_s64(op1: svint32_t, op2: svint32_t, imm_index: i32) -> svint64_t;
    }
    unsafe { _svqdmullt_lane_s64(op1, op2, IMM_INDEX) }
}
#[doc = "Saturating doubling multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmullt[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmullt))]
pub fn svqdmullt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmullt.nxv8i16"
        )]
        fn _svqdmullt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svqdmullt_s16(op1, op2) }
}
#[doc = "Saturating doubling multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmullt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmullt))]
pub fn svqdmullt_n_s16(op1: svint8_t, op2: i8) -> svint16_t {
    svqdmullt_s16(op1, svdup_n_s8(op2))
}
#[doc = "Saturating doubling multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmullt[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmullt))]
pub fn svqdmullt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmullt.nxv4i32"
        )]
        fn _svqdmullt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svqdmullt_s32(op1, op2) }
}
#[doc = "Saturating doubling multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmullt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmullt))]
pub fn svqdmullt_n_s32(op1: svint16_t, op2: i16) -> svint32_t {
    svqdmullt_s32(op1, svdup_n_s16(op2))
}
#[doc = "Saturating doubling multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmullt[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmullt))]
pub fn svqdmullt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqdmullt.nxv2i64"
        )]
        fn _svqdmullt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svqdmullt_s64(op1, op2) }
}
#[doc = "Saturating doubling multiply long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqdmullt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqdmullt))]
pub fn svqdmullt_n_s64(op1: svint32_t, op2: i32) -> svint64_t {
    svqdmullt_s64(op1, svdup_n_s32(op2))
}
#[doc = "Saturating negate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqneg[_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqneg))]
pub fn svqneg_s8_m(inactive: svint8_t, pg: svbool_t, op: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqneg.nxv16i8")]
        fn _svqneg_s8_m(inactive: svint8_t, pg: svbool_t, op: svint8_t) -> svint8_t;
    }
    unsafe { _svqneg_s8_m(inactive, pg, op) }
}
#[doc = "Saturating negate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqneg[_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqneg))]
pub fn svqneg_s8_x(pg: svbool_t, op: svint8_t) -> svint8_t {
    svqneg_s8_m(op, pg, op)
}
#[doc = "Saturating negate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqneg[_s8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqneg))]
pub fn svqneg_s8_z(pg: svbool_t, op: svint8_t) -> svint8_t {
    svqneg_s8_m(svdup_n_s8(0), pg, op)
}
#[doc = "Saturating negate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqneg[_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqneg))]
pub fn svqneg_s16_m(inactive: svint16_t, pg: svbool_t, op: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqneg.nxv8i16")]
        fn _svqneg_s16_m(inactive: svint16_t, pg: svbool8_t, op: svint16_t) -> svint16_t;
    }
    unsafe { _svqneg_s16_m(inactive, pg.into(), op) }
}
#[doc = "Saturating negate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqneg[_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqneg))]
pub fn svqneg_s16_x(pg: svbool_t, op: svint16_t) -> svint16_t {
    svqneg_s16_m(op, pg, op)
}
#[doc = "Saturating negate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqneg[_s16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqneg))]
pub fn svqneg_s16_z(pg: svbool_t, op: svint16_t) -> svint16_t {
    svqneg_s16_m(svdup_n_s16(0), pg, op)
}
#[doc = "Saturating negate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqneg[_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqneg))]
pub fn svqneg_s32_m(inactive: svint32_t, pg: svbool_t, op: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqneg.nxv4i32")]
        fn _svqneg_s32_m(inactive: svint32_t, pg: svbool4_t, op: svint32_t) -> svint32_t;
    }
    unsafe { _svqneg_s32_m(inactive, pg.into(), op) }
}
#[doc = "Saturating negate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqneg[_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqneg))]
pub fn svqneg_s32_x(pg: svbool_t, op: svint32_t) -> svint32_t {
    svqneg_s32_m(op, pg, op)
}
#[doc = "Saturating negate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqneg[_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqneg))]
pub fn svqneg_s32_z(pg: svbool_t, op: svint32_t) -> svint32_t {
    svqneg_s32_m(svdup_n_s32(0), pg, op)
}
#[doc = "Saturating negate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqneg[_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqneg))]
pub fn svqneg_s64_m(inactive: svint64_t, pg: svbool_t, op: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqneg.nxv2i64")]
        fn _svqneg_s64_m(inactive: svint64_t, pg: svbool2_t, op: svint64_t) -> svint64_t;
    }
    unsafe { _svqneg_s64_m(inactive, pg.into(), op) }
}
#[doc = "Saturating negate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqneg[_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqneg))]
pub fn svqneg_s64_x(pg: svbool_t, op: svint64_t) -> svint64_t {
    svqneg_s64_m(op, pg, op)
}
#[doc = "Saturating negate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqneg[_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqneg))]
pub fn svqneg_s64_z(pg: svbool_t, op: svint64_t) -> svint64_t {
    svqneg_s64_m(svdup_n_s64(0), pg, op)
}
#[doc = "Saturating rounding doubling complex multiply-add high with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdcmlah_lane[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdcmlah, IMM_INDEX = 0, IMM_ROTATION = 90))]
pub fn svqrdcmlah_lane_s16<const IMM_INDEX: i32, const IMM_ROTATION: i32>(
    op1: svint16_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint16_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrdcmlah.lane.x.nxv8i16"
        )]
        fn _svqrdcmlah_lane_s16(
            op1: svint16_t,
            op2: svint16_t,
            op3: svint16_t,
            imm_index: i32,
            imm_rotation: i32,
        ) -> svint16_t;
    }
    unsafe { _svqrdcmlah_lane_s16(op1, op2, op3, IMM_INDEX, IMM_ROTATION) }
}
#[doc = "Saturating rounding doubling complex multiply-add high with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdcmlah_lane[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdcmlah, IMM_INDEX = 0, IMM_ROTATION = 90))]
pub fn svqrdcmlah_lane_s32<const IMM_INDEX: i32, const IMM_ROTATION: i32>(
    op1: svint32_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint32_t {
    static_assert_range!(IMM_INDEX, 0, 1);
    static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrdcmlah.lane.x.nxv4i32"
        )]
        fn _svqrdcmlah_lane_s32(
            op1: svint32_t,
            op2: svint32_t,
            op3: svint32_t,
            imm_index: i32,
            imm_rotation: i32,
        ) -> svint32_t;
    }
    unsafe { _svqrdcmlah_lane_s32(op1, op2, op3, IMM_INDEX, IMM_ROTATION) }
}
#[doc = "Saturating rounding doubling complex multiply-add high with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdcmlah[_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdcmlah, IMM_ROTATION = 90))]
pub fn svqrdcmlah_s8<const IMM_ROTATION: i32>(
    op1: svint8_t,
    op2: svint8_t,
    op3: svint8_t,
) -> svint8_t {
    static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrdcmlah.x.nxv16i8"
        )]
        fn _svqrdcmlah_s8(
            op1: svint8_t,
            op2: svint8_t,
            op3: svint8_t,
            imm_rotation: i32,
        ) -> svint8_t;
    }
    unsafe { _svqrdcmlah_s8(op1, op2, op3, IMM_ROTATION) }
}
#[doc = "Saturating rounding doubling complex multiply-add high with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdcmlah[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdcmlah, IMM_ROTATION = 90))]
pub fn svqrdcmlah_s16<const IMM_ROTATION: i32>(
    op1: svint16_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint16_t {
    static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrdcmlah.x.nxv8i16"
        )]
        fn _svqrdcmlah_s16(
            op1: svint16_t,
            op2: svint16_t,
            op3: svint16_t,
            imm_rotation: i32,
        ) -> svint16_t;
    }
    unsafe { _svqrdcmlah_s16(op1, op2, op3, IMM_ROTATION) }
}
#[doc = "Saturating rounding doubling complex multiply-add high with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdcmlah[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdcmlah, IMM_ROTATION = 90))]
pub fn svqrdcmlah_s32<const IMM_ROTATION: i32>(
    op1: svint32_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint32_t {
    static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrdcmlah.x.nxv4i32"
        )]
        fn _svqrdcmlah_s32(
            op1: svint32_t,
            op2: svint32_t,
            op3: svint32_t,
            imm_rotation: i32,
        ) -> svint32_t;
    }
    unsafe { _svqrdcmlah_s32(op1, op2, op3, IMM_ROTATION) }
}
#[doc = "Saturating rounding doubling complex multiply-add high with rotate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdcmlah[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdcmlah, IMM_ROTATION = 90))]
pub fn svqrdcmlah_s64<const IMM_ROTATION: i32>(
    op1: svint64_t,
    op2: svint64_t,
    op3: svint64_t,
) -> svint64_t {
    static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrdcmlah.x.nxv2i64"
        )]
        fn _svqrdcmlah_s64(
            op1: svint64_t,
            op2: svint64_t,
            op3: svint64_t,
            imm_rotation: i32,
        ) -> svint64_t;
    }
    unsafe { _svqrdcmlah_s64(op1, op2, op3, IMM_ROTATION) }
}
#[doc = "Saturating rounding doubling multiply-add high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmlah_lane[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmlah, IMM_INDEX = 0))]
pub fn svqrdmlah_lane_s16<const IMM_INDEX: i32>(
    op1: svint16_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint16_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrdmlah.lane.nxv8i16"
        )]
        fn _svqrdmlah_lane_s16(
            op1: svint16_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint16_t;
    }
    unsafe { _svqrdmlah_lane_s16(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Saturating rounding doubling multiply-add high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmlah_lane[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmlah, IMM_INDEX = 0))]
pub fn svqrdmlah_lane_s32<const IMM_INDEX: i32>(
    op1: svint32_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint32_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrdmlah.lane.nxv4i32"
        )]
        fn _svqrdmlah_lane_s32(
            op1: svint32_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe { _svqrdmlah_lane_s32(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Saturating rounding doubling multiply-add high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmlah_lane[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmlah, IMM_INDEX = 0))]
pub fn svqrdmlah_lane_s64<const IMM_INDEX: i32>(
    op1: svint64_t,
    op2: svint64_t,
    op3: svint64_t,
) -> svint64_t {
    static_assert_range!(IMM_INDEX, 0, 1);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrdmlah.lane.nxv2i64"
        )]
        fn _svqrdmlah_lane_s64(
            op1: svint64_t,
            op2: svint64_t,
            op3: svint64_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe { _svqrdmlah_lane_s64(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Saturating rounding doubling multiply-add high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmlah[_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmlah))]
pub fn svqrdmlah_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrdmlah.nxv16i8"
        )]
        fn _svqrdmlah_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t;
    }
    unsafe { _svqrdmlah_s8(op1, op2, op3) }
}
#[doc = "Saturating rounding doubling multiply-add high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmlah[_n_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmlah))]
pub fn svqrdmlah_n_s8(op1: svint8_t, op2: svint8_t, op3: i8) -> svint8_t {
    svqrdmlah_s8(op1, op2, svdup_n_s8(op3))
}
#[doc = "Saturating rounding doubling multiply-add high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmlah[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmlah))]
pub fn svqrdmlah_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrdmlah.nxv8i16"
        )]
        fn _svqrdmlah_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t;
    }
    unsafe { _svqrdmlah_s16(op1, op2, op3) }
}
#[doc = "Saturating rounding doubling multiply-add high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmlah[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmlah))]
pub fn svqrdmlah_n_s16(op1: svint16_t, op2: svint16_t, op3: i16) -> svint16_t {
    svqrdmlah_s16(op1, op2, svdup_n_s16(op3))
}
#[doc = "Saturating rounding doubling multiply-add high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmlah[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmlah))]
pub fn svqrdmlah_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrdmlah.nxv4i32"
        )]
        fn _svqrdmlah_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _svqrdmlah_s32(op1, op2, op3) }
}
#[doc = "Saturating rounding doubling multiply-add high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmlah[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmlah))]
pub fn svqrdmlah_n_s32(op1: svint32_t, op2: svint32_t, op3: i32) -> svint32_t {
    svqrdmlah_s32(op1, op2, svdup_n_s32(op3))
}
#[doc = "Saturating rounding doubling multiply-add high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmlah[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmlah))]
pub fn svqrdmlah_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrdmlah.nxv2i64"
        )]
        fn _svqrdmlah_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _svqrdmlah_s64(op1, op2, op3) }
}
#[doc = "Saturating rounding doubling multiply-add high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmlah[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmlah))]
pub fn svqrdmlah_n_s64(op1: svint64_t, op2: svint64_t, op3: i64) -> svint64_t {
    svqrdmlah_s64(op1, op2, svdup_n_s64(op3))
}
#[doc = "Saturating rounding doubling multiply-subtract high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmlsh_lane[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmlsh, IMM_INDEX = 0))]
pub fn svqrdmlsh_lane_s16<const IMM_INDEX: i32>(
    op1: svint16_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint16_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrdmlsh.lane.nxv8i16"
        )]
        fn _svqrdmlsh_lane_s16(
            op1: svint16_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint16_t;
    }
    unsafe { _svqrdmlsh_lane_s16(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Saturating rounding doubling multiply-subtract high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmlsh_lane[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmlsh, IMM_INDEX = 0))]
pub fn svqrdmlsh_lane_s32<const IMM_INDEX: i32>(
    op1: svint32_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint32_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrdmlsh.lane.nxv4i32"
        )]
        fn _svqrdmlsh_lane_s32(
            op1: svint32_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe { _svqrdmlsh_lane_s32(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Saturating rounding doubling multiply-subtract high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmlsh_lane[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmlsh, IMM_INDEX = 0))]
pub fn svqrdmlsh_lane_s64<const IMM_INDEX: i32>(
    op1: svint64_t,
    op2: svint64_t,
    op3: svint64_t,
) -> svint64_t {
    static_assert_range!(IMM_INDEX, 0, 1);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrdmlsh.lane.nxv2i64"
        )]
        fn _svqrdmlsh_lane_s64(
            op1: svint64_t,
            op2: svint64_t,
            op3: svint64_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe { _svqrdmlsh_lane_s64(op1, op2, op3, IMM_INDEX) }
}
#[doc = "Saturating rounding doubling multiply-subtract high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmlsh[_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmlsh))]
pub fn svqrdmlsh_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrdmlsh.nxv16i8"
        )]
        fn _svqrdmlsh_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t;
    }
    unsafe { _svqrdmlsh_s8(op1, op2, op3) }
}
#[doc = "Saturating rounding doubling multiply-subtract high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmlsh[_n_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmlsh))]
pub fn svqrdmlsh_n_s8(op1: svint8_t, op2: svint8_t, op3: i8) -> svint8_t {
    svqrdmlsh_s8(op1, op2, svdup_n_s8(op3))
}
#[doc = "Saturating rounding doubling multiply-subtract high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmlsh[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmlsh))]
pub fn svqrdmlsh_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrdmlsh.nxv8i16"
        )]
        fn _svqrdmlsh_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t;
    }
    unsafe { _svqrdmlsh_s16(op1, op2, op3) }
}
#[doc = "Saturating rounding doubling multiply-subtract high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmlsh[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmlsh))]
pub fn svqrdmlsh_n_s16(op1: svint16_t, op2: svint16_t, op3: i16) -> svint16_t {
    svqrdmlsh_s16(op1, op2, svdup_n_s16(op3))
}
#[doc = "Saturating rounding doubling multiply-subtract high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmlsh[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmlsh))]
pub fn svqrdmlsh_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrdmlsh.nxv4i32"
        )]
        fn _svqrdmlsh_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _svqrdmlsh_s32(op1, op2, op3) }
}
#[doc = "Saturating rounding doubling multiply-subtract high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmlsh[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmlsh))]
pub fn svqrdmlsh_n_s32(op1: svint32_t, op2: svint32_t, op3: i32) -> svint32_t {
    svqrdmlsh_s32(op1, op2, svdup_n_s32(op3))
}
#[doc = "Saturating rounding doubling multiply-subtract high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmlsh[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmlsh))]
pub fn svqrdmlsh_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrdmlsh.nxv2i64"
        )]
        fn _svqrdmlsh_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _svqrdmlsh_s64(op1, op2, op3) }
}
#[doc = "Saturating rounding doubling multiply-subtract high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmlsh[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmlsh))]
pub fn svqrdmlsh_n_s64(op1: svint64_t, op2: svint64_t, op3: i64) -> svint64_t {
    svqrdmlsh_s64(op1, op2, svdup_n_s64(op3))
}
#[doc = "Saturating rounding doubling multiply high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmulh_lane[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmulh, IMM_INDEX = 0))]
pub fn svqrdmulh_lane_s16<const IMM_INDEX: i32>(op1: svint16_t, op2: svint16_t) -> svint16_t {
    static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrdmulh.lane.nxv8i16"
        )]
        fn _svqrdmulh_lane_s16(op1: svint16_t, op2: svint16_t, imm_index: i32) -> svint16_t;
    }
    unsafe { _svqrdmulh_lane_s16(op1, op2, IMM_INDEX) }
}
#[doc = "Saturating rounding doubling multiply high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmulh_lane[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmulh, IMM_INDEX = 0))]
pub fn svqrdmulh_lane_s32<const IMM_INDEX: i32>(op1: svint32_t, op2: svint32_t) -> svint32_t {
    static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrdmulh.lane.nxv4i32"
        )]
        fn _svqrdmulh_lane_s32(op1: svint32_t, op2: svint32_t, imm_index: i32) -> svint32_t;
    }
    unsafe { _svqrdmulh_lane_s32(op1, op2, IMM_INDEX) }
}
#[doc = "Saturating rounding doubling multiply high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmulh_lane[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmulh, IMM_INDEX = 0))]
pub fn svqrdmulh_lane_s64<const IMM_INDEX: i32>(op1: svint64_t, op2: svint64_t) -> svint64_t {
    static_assert_range!(IMM_INDEX, 0, 1);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrdmulh.lane.nxv2i64"
        )]
        fn _svqrdmulh_lane_s64(op1: svint64_t, op2: svint64_t, imm_index: i32) -> svint64_t;
    }
    unsafe { _svqrdmulh_lane_s64(op1, op2, IMM_INDEX) }
}
#[doc = "Saturating rounding doubling multiply high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmulh[_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmulh))]
pub fn svqrdmulh_s8(op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrdmulh.nxv16i8"
        )]
        fn _svqrdmulh_s8(op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svqrdmulh_s8(op1, op2) }
}
#[doc = "Saturating rounding doubling multiply high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmulh[_n_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmulh))]
pub fn svqrdmulh_n_s8(op1: svint8_t, op2: i8) -> svint8_t {
    svqrdmulh_s8(op1, svdup_n_s8(op2))
}
#[doc = "Saturating rounding doubling multiply high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmulh[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmulh))]
pub fn svqrdmulh_s16(op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrdmulh.nxv8i16"
        )]
        fn _svqrdmulh_s16(op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svqrdmulh_s16(op1, op2) }
}
#[doc = "Saturating rounding doubling multiply high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmulh[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmulh))]
pub fn svqrdmulh_n_s16(op1: svint16_t, op2: i16) -> svint16_t {
    svqrdmulh_s16(op1, svdup_n_s16(op2))
}
#[doc = "Saturating rounding doubling multiply high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmulh[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmulh))]
pub fn svqrdmulh_s32(op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrdmulh.nxv4i32"
        )]
        fn _svqrdmulh_s32(op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svqrdmulh_s32(op1, op2) }
}
#[doc = "Saturating rounding doubling multiply high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmulh[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmulh))]
pub fn svqrdmulh_n_s32(op1: svint32_t, op2: i32) -> svint32_t {
    svqrdmulh_s32(op1, svdup_n_s32(op2))
}
#[doc = "Saturating rounding doubling multiply high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmulh[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmulh))]
pub fn svqrdmulh_s64(op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrdmulh.nxv2i64"
        )]
        fn _svqrdmulh_s64(op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svqrdmulh_s64(op1, op2) }
}
#[doc = "Saturating rounding doubling multiply high"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrdmulh[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrdmulh))]
pub fn svqrdmulh_n_s64(op1: svint64_t, op2: i64) -> svint64_t {
    svqrdmulh_s64(op1, svdup_n_s64(op2))
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshl))]
pub fn svqrshl_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqrshl.nxv16i8")]
        fn _svqrshl_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svqrshl_s8_m(pg, op1, op2) }
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_n_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshl))]
pub fn svqrshl_n_s8_m(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqrshl_s8_m(pg, op1, svdup_n_s8(op2))
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshl))]
pub fn svqrshl_s8_x(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svqrshl_s8_m(pg, op1, op2)
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_n_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshl))]
pub fn svqrshl_n_s8_x(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqrshl_s8_x(pg, op1, svdup_n_s8(op2))
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_s8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshl))]
pub fn svqrshl_s8_z(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svqrshl_s8_m(pg, svsel_s8(pg, op1, svdup_n_s8(0)), op2)
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_n_s8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshl))]
pub fn svqrshl_n_s8_z(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqrshl_s8_z(pg, op1, svdup_n_s8(op2))
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshl))]
pub fn svqrshl_s16_m(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqrshl.nxv8i16")]
        fn _svqrshl_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svqrshl_s16_m(pg.into(), op1, op2) }
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_n_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshl))]
pub fn svqrshl_n_s16_m(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqrshl_s16_m(pg, op1, svdup_n_s16(op2))
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshl))]
pub fn svqrshl_s16_x(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svqrshl_s16_m(pg, op1, op2)
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_n_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshl))]
pub fn svqrshl_n_s16_x(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqrshl_s16_x(pg, op1, svdup_n_s16(op2))
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_s16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshl))]
pub fn svqrshl_s16_z(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svqrshl_s16_m(pg, svsel_s16(pg, op1, svdup_n_s16(0)), op2)
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_n_s16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshl))]
pub fn svqrshl_n_s16_z(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqrshl_s16_z(pg, op1, svdup_n_s16(op2))
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshl))]
pub fn svqrshl_s32_m(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqrshl.nxv4i32")]
        fn _svqrshl_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svqrshl_s32_m(pg.into(), op1, op2) }
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_n_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshl))]
pub fn svqrshl_n_s32_m(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqrshl_s32_m(pg, op1, svdup_n_s32(op2))
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshl))]
pub fn svqrshl_s32_x(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svqrshl_s32_m(pg, op1, op2)
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_n_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshl))]
pub fn svqrshl_n_s32_x(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqrshl_s32_x(pg, op1, svdup_n_s32(op2))
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshl))]
pub fn svqrshl_s32_z(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svqrshl_s32_m(pg, svsel_s32(pg, op1, svdup_n_s32(0)), op2)
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_n_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshl))]
pub fn svqrshl_n_s32_z(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqrshl_s32_z(pg, op1, svdup_n_s32(op2))
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshl))]
pub fn svqrshl_s64_m(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqrshl.nxv2i64")]
        fn _svqrshl_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svqrshl_s64_m(pg.into(), op1, op2) }
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_n_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshl))]
pub fn svqrshl_n_s64_m(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqrshl_s64_m(pg, op1, svdup_n_s64(op2))
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshl))]
pub fn svqrshl_s64_x(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svqrshl_s64_m(pg, op1, op2)
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_n_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshl))]
pub fn svqrshl_n_s64_x(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqrshl_s64_x(pg, op1, svdup_n_s64(op2))
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshl))]
pub fn svqrshl_s64_z(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svqrshl_s64_m(pg, svsel_s64(pg, op1, svdup_n_s64(0)), op2)
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_n_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshl))]
pub fn svqrshl_n_s64_z(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqrshl_s64_z(pg, op1, svdup_n_s64(op2))
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_u8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshl))]
pub fn svqrshl_u8_m(pg: svbool_t, op1: svuint8_t, op2: svint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uqrshl.nxv16i8")]
        fn _svqrshl_u8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svqrshl_u8_m(pg, op1.as_signed(), op2).as_unsigned() }
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_n_u8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshl))]
pub fn svqrshl_n_u8_m(pg: svbool_t, op1: svuint8_t, op2: i8) -> svuint8_t {
    svqrshl_u8_m(pg, op1, svdup_n_s8(op2))
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_u8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshl))]
pub fn svqrshl_u8_x(pg: svbool_t, op1: svuint8_t, op2: svint8_t) -> svuint8_t {
    svqrshl_u8_m(pg, op1, op2)
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_n_u8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshl))]
pub fn svqrshl_n_u8_x(pg: svbool_t, op1: svuint8_t, op2: i8) -> svuint8_t {
    svqrshl_u8_x(pg, op1, svdup_n_s8(op2))
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_u8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshl))]
pub fn svqrshl_u8_z(pg: svbool_t, op1: svuint8_t, op2: svint8_t) -> svuint8_t {
    svqrshl_u8_m(pg, svsel_u8(pg, op1, svdup_n_u8(0)), op2)
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_n_u8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshl))]
pub fn svqrshl_n_u8_z(pg: svbool_t, op1: svuint8_t, op2: i8) -> svuint8_t {
    svqrshl_u8_z(pg, op1, svdup_n_s8(op2))
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_u16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshl))]
pub fn svqrshl_u16_m(pg: svbool_t, op1: svuint16_t, op2: svint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uqrshl.nxv8i16")]
        fn _svqrshl_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svqrshl_u16_m(pg.into(), op1.as_signed(), op2).as_unsigned() }
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_n_u16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshl))]
pub fn svqrshl_n_u16_m(pg: svbool_t, op1: svuint16_t, op2: i16) -> svuint16_t {
    svqrshl_u16_m(pg, op1, svdup_n_s16(op2))
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_u16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshl))]
pub fn svqrshl_u16_x(pg: svbool_t, op1: svuint16_t, op2: svint16_t) -> svuint16_t {
    svqrshl_u16_m(pg, op1, op2)
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_n_u16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshl))]
pub fn svqrshl_n_u16_x(pg: svbool_t, op1: svuint16_t, op2: i16) -> svuint16_t {
    svqrshl_u16_x(pg, op1, svdup_n_s16(op2))
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_u16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshl))]
pub fn svqrshl_u16_z(pg: svbool_t, op1: svuint16_t, op2: svint16_t) -> svuint16_t {
    svqrshl_u16_m(pg, svsel_u16(pg, op1, svdup_n_u16(0)), op2)
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_n_u16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshl))]
pub fn svqrshl_n_u16_z(pg: svbool_t, op1: svuint16_t, op2: i16) -> svuint16_t {
    svqrshl_u16_z(pg, op1, svdup_n_s16(op2))
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshl))]
pub fn svqrshl_u32_m(pg: svbool_t, op1: svuint32_t, op2: svint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uqrshl.nxv4i32")]
        fn _svqrshl_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svqrshl_u32_m(pg.into(), op1.as_signed(), op2).as_unsigned() }
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_n_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshl))]
pub fn svqrshl_n_u32_m(pg: svbool_t, op1: svuint32_t, op2: i32) -> svuint32_t {
    svqrshl_u32_m(pg, op1, svdup_n_s32(op2))
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshl))]
pub fn svqrshl_u32_x(pg: svbool_t, op1: svuint32_t, op2: svint32_t) -> svuint32_t {
    svqrshl_u32_m(pg, op1, op2)
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_n_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshl))]
pub fn svqrshl_n_u32_x(pg: svbool_t, op1: svuint32_t, op2: i32) -> svuint32_t {
    svqrshl_u32_x(pg, op1, svdup_n_s32(op2))
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_u32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshl))]
pub fn svqrshl_u32_z(pg: svbool_t, op1: svuint32_t, op2: svint32_t) -> svuint32_t {
    svqrshl_u32_m(pg, svsel_u32(pg, op1, svdup_n_u32(0)), op2)
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_n_u32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshl))]
pub fn svqrshl_n_u32_z(pg: svbool_t, op1: svuint32_t, op2: i32) -> svuint32_t {
    svqrshl_u32_z(pg, op1, svdup_n_s32(op2))
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_u64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshl))]
pub fn svqrshl_u64_m(pg: svbool_t, op1: svuint64_t, op2: svint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uqrshl.nxv2i64")]
        fn _svqrshl_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svqrshl_u64_m(pg.into(), op1.as_signed(), op2).as_unsigned() }
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_n_u64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshl))]
pub fn svqrshl_n_u64_m(pg: svbool_t, op1: svuint64_t, op2: i64) -> svuint64_t {
    svqrshl_u64_m(pg, op1, svdup_n_s64(op2))
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_u64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshl))]
pub fn svqrshl_u64_x(pg: svbool_t, op1: svuint64_t, op2: svint64_t) -> svuint64_t {
    svqrshl_u64_m(pg, op1, op2)
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_n_u64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshl))]
pub fn svqrshl_n_u64_x(pg: svbool_t, op1: svuint64_t, op2: i64) -> svuint64_t {
    svqrshl_u64_x(pg, op1, svdup_n_s64(op2))
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_u64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshl))]
pub fn svqrshl_u64_z(pg: svbool_t, op1: svuint64_t, op2: svint64_t) -> svuint64_t {
    svqrshl_u64_m(pg, svsel_u64(pg, op1, svdup_n_u64(0)), op2)
}
#[doc = "Saturating rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshl[_n_u64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshl))]
pub fn svqrshl_n_u64_z(pg: svbool_t, op1: svuint64_t, op2: i64) -> svuint64_t {
    svqrshl_u64_z(pg, op1, svdup_n_s64(op2))
}
#[doc = "Saturating rounding shift right narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshrnb[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshrnb, IMM2 = 1))]
pub fn svqrshrnb_n_s16<const IMM2: i32>(op1: svint16_t) -> svint8_t {
    static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrshrnb.nxv8i16"
        )]
        fn _svqrshrnb_n_s16(op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svqrshrnb_n_s16(op1, IMM2) }
}
#[doc = "Saturating rounding shift right narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshrnb[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshrnb, IMM2 = 1))]
pub fn svqrshrnb_n_s32<const IMM2: i32>(op1: svint32_t) -> svint16_t {
    static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrshrnb.nxv4i32"
        )]
        fn _svqrshrnb_n_s32(op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svqrshrnb_n_s32(op1, IMM2) }
}
#[doc = "Saturating rounding shift right narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshrnb[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshrnb, IMM2 = 1))]
pub fn svqrshrnb_n_s64<const IMM2: i32>(op1: svint64_t) -> svint32_t {
    static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrshrnb.nxv2i64"
        )]
        fn _svqrshrnb_n_s64(op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svqrshrnb_n_s64(op1, IMM2) }
}
#[doc = "Saturating rounding shift right narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshrnb[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshrnb, IMM2 = 1))]
pub fn svqrshrnb_n_u16<const IMM2: i32>(op1: svuint16_t) -> svuint8_t {
    static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.uqrshrnb.nxv8i16"
        )]
        fn _svqrshrnb_n_u16(op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svqrshrnb_n_u16(op1.as_signed(), IMM2).as_unsigned() }
}
#[doc = "Saturating rounding shift right narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshrnb[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshrnb, IMM2 = 1))]
pub fn svqrshrnb_n_u32<const IMM2: i32>(op1: svuint32_t) -> svuint16_t {
    static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.uqrshrnb.nxv4i32"
        )]
        fn _svqrshrnb_n_u32(op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svqrshrnb_n_u32(op1.as_signed(), IMM2).as_unsigned() }
}
#[doc = "Saturating rounding shift right narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshrnb[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshrnb, IMM2 = 1))]
pub fn svqrshrnb_n_u64<const IMM2: i32>(op1: svuint64_t) -> svuint32_t {
    static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.uqrshrnb.nxv2i64"
        )]
        fn _svqrshrnb_n_u64(op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svqrshrnb_n_u64(op1.as_signed(), IMM2).as_unsigned() }
}
#[doc = "Saturating rounding shift right narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshrnt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshrnt, IMM2 = 1))]
pub fn svqrshrnt_n_s16<const IMM2: i32>(even: svint8_t, op1: svint16_t) -> svint8_t {
    static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrshrnt.nxv8i16"
        )]
        fn _svqrshrnt_n_s16(even: svint8_t, op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svqrshrnt_n_s16(even, op1, IMM2) }
}
#[doc = "Saturating rounding shift right narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshrnt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshrnt, IMM2 = 1))]
pub fn svqrshrnt_n_s32<const IMM2: i32>(even: svint16_t, op1: svint32_t) -> svint16_t {
    static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrshrnt.nxv4i32"
        )]
        fn _svqrshrnt_n_s32(even: svint16_t, op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svqrshrnt_n_s32(even, op1, IMM2) }
}
#[doc = "Saturating rounding shift right narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshrnt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshrnt, IMM2 = 1))]
pub fn svqrshrnt_n_s64<const IMM2: i32>(even: svint32_t, op1: svint64_t) -> svint32_t {
    static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrshrnt.nxv2i64"
        )]
        fn _svqrshrnt_n_s64(even: svint32_t, op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svqrshrnt_n_s64(even, op1, IMM2) }
}
#[doc = "Saturating rounding shift right narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshrnt[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshrnt, IMM2 = 1))]
pub fn svqrshrnt_n_u16<const IMM2: i32>(even: svuint8_t, op1: svuint16_t) -> svuint8_t {
    static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.uqrshrnt.nxv8i16"
        )]
        fn _svqrshrnt_n_u16(even: svint8_t, op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svqrshrnt_n_u16(even.as_signed(), op1.as_signed(), IMM2).as_unsigned() }
}
#[doc = "Saturating rounding shift right narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshrnt[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshrnt, IMM2 = 1))]
pub fn svqrshrnt_n_u32<const IMM2: i32>(even: svuint16_t, op1: svuint32_t) -> svuint16_t {
    static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.uqrshrnt.nxv4i32"
        )]
        fn _svqrshrnt_n_u32(even: svint16_t, op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svqrshrnt_n_u32(even.as_signed(), op1.as_signed(), IMM2).as_unsigned() }
}
#[doc = "Saturating rounding shift right narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshrnt[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqrshrnt, IMM2 = 1))]
pub fn svqrshrnt_n_u64<const IMM2: i32>(even: svuint32_t, op1: svuint64_t) -> svuint32_t {
    static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.uqrshrnt.nxv2i64"
        )]
        fn _svqrshrnt_n_u64(even: svint32_t, op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svqrshrnt_n_u64(even.as_signed(), op1.as_signed(), IMM2).as_unsigned() }
}
#[doc = "Saturating rounding shift right unsigned narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshrunb[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshrunb, IMM2 = 1))]
pub fn svqrshrunb_n_s16<const IMM2: i32>(op1: svint16_t) -> svuint8_t {
    static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrshrunb.nxv8i16"
        )]
        fn _svqrshrunb_n_s16(op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svqrshrunb_n_s16(op1, IMM2).as_unsigned() }
}
#[doc = "Saturating rounding shift right unsigned narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshrunb[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshrunb, IMM2 = 1))]
pub fn svqrshrunb_n_s32<const IMM2: i32>(op1: svint32_t) -> svuint16_t {
    static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrshrunb.nxv4i32"
        )]
        fn _svqrshrunb_n_s32(op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svqrshrunb_n_s32(op1, IMM2).as_unsigned() }
}
#[doc = "Saturating rounding shift right unsigned narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshrunb[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshrunb, IMM2 = 1))]
pub fn svqrshrunb_n_s64<const IMM2: i32>(op1: svint64_t) -> svuint32_t {
    static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrshrunb.nxv2i64"
        )]
        fn _svqrshrunb_n_s64(op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svqrshrunb_n_s64(op1, IMM2).as_unsigned() }
}
#[doc = "Saturating rounding shift right unsigned narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshrunt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshrunt, IMM2 = 1))]
pub fn svqrshrunt_n_s16<const IMM2: i32>(even: svuint8_t, op1: svint16_t) -> svuint8_t {
    static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrshrunt.nxv8i16"
        )]
        fn _svqrshrunt_n_s16(even: svint8_t, op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svqrshrunt_n_s16(even.as_signed(), op1, IMM2).as_unsigned() }
}
#[doc = "Saturating rounding shift right unsigned narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshrunt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshrunt, IMM2 = 1))]
pub fn svqrshrunt_n_s32<const IMM2: i32>(even: svuint16_t, op1: svint32_t) -> svuint16_t {
    static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrshrunt.nxv4i32"
        )]
        fn _svqrshrunt_n_s32(even: svint16_t, op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svqrshrunt_n_s32(even.as_signed(), op1, IMM2).as_unsigned() }
}
#[doc = "Saturating rounding shift right unsigned narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqrshrunt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqrshrunt, IMM2 = 1))]
pub fn svqrshrunt_n_s64<const IMM2: i32>(even: svuint32_t, op1: svint64_t) -> svuint32_t {
    static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqrshrunt.nxv2i64"
        )]
        fn _svqrshrunt_n_s64(even: svint32_t, op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svqrshrunt_n_s64(even.as_signed(), op1, IMM2).as_unsigned() }
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshl))]
pub fn svqshl_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqshl.nxv16i8")]
        fn _svqshl_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svqshl_s8_m(pg, op1, op2) }
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_n_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshl))]
pub fn svqshl_n_s8_m(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqshl_s8_m(pg, op1, svdup_n_s8(op2))
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshl))]
pub fn svqshl_s8_x(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svqshl_s8_m(pg, op1, op2)
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_n_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshl))]
pub fn svqshl_n_s8_x(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqshl_s8_x(pg, op1, svdup_n_s8(op2))
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_s8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshl))]
pub fn svqshl_s8_z(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svqshl_s8_m(pg, svsel_s8(pg, op1, svdup_n_s8(0)), op2)
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_n_s8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshl))]
pub fn svqshl_n_s8_z(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqshl_s8_z(pg, op1, svdup_n_s8(op2))
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshl))]
pub fn svqshl_s16_m(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqshl.nxv8i16")]
        fn _svqshl_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svqshl_s16_m(pg.into(), op1, op2) }
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_n_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshl))]
pub fn svqshl_n_s16_m(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqshl_s16_m(pg, op1, svdup_n_s16(op2))
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshl))]
pub fn svqshl_s16_x(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svqshl_s16_m(pg, op1, op2)
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_n_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshl))]
pub fn svqshl_n_s16_x(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqshl_s16_x(pg, op1, svdup_n_s16(op2))
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_s16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshl))]
pub fn svqshl_s16_z(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svqshl_s16_m(pg, svsel_s16(pg, op1, svdup_n_s16(0)), op2)
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_n_s16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshl))]
pub fn svqshl_n_s16_z(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqshl_s16_z(pg, op1, svdup_n_s16(op2))
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshl))]
pub fn svqshl_s32_m(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqshl.nxv4i32")]
        fn _svqshl_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svqshl_s32_m(pg.into(), op1, op2) }
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_n_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshl))]
pub fn svqshl_n_s32_m(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqshl_s32_m(pg, op1, svdup_n_s32(op2))
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshl))]
pub fn svqshl_s32_x(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svqshl_s32_m(pg, op1, op2)
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_n_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshl))]
pub fn svqshl_n_s32_x(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqshl_s32_x(pg, op1, svdup_n_s32(op2))
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshl))]
pub fn svqshl_s32_z(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svqshl_s32_m(pg, svsel_s32(pg, op1, svdup_n_s32(0)), op2)
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_n_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshl))]
pub fn svqshl_n_s32_z(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqshl_s32_z(pg, op1, svdup_n_s32(op2))
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshl))]
pub fn svqshl_s64_m(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqshl.nxv2i64")]
        fn _svqshl_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svqshl_s64_m(pg.into(), op1, op2) }
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_n_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshl))]
pub fn svqshl_n_s64_m(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqshl_s64_m(pg, op1, svdup_n_s64(op2))
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshl))]
pub fn svqshl_s64_x(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svqshl_s64_m(pg, op1, op2)
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_n_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshl))]
pub fn svqshl_n_s64_x(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqshl_s64_x(pg, op1, svdup_n_s64(op2))
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshl))]
pub fn svqshl_s64_z(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svqshl_s64_m(pg, svsel_s64(pg, op1, svdup_n_s64(0)), op2)
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_n_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshl))]
pub fn svqshl_n_s64_z(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqshl_s64_z(pg, op1, svdup_n_s64(op2))
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_u8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshl))]
pub fn svqshl_u8_m(pg: svbool_t, op1: svuint8_t, op2: svint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uqshl.nxv16i8")]
        fn _svqshl_u8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svqshl_u8_m(pg, op1.as_signed(), op2).as_unsigned() }
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_n_u8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshl))]
pub fn svqshl_n_u8_m(pg: svbool_t, op1: svuint8_t, op2: i8) -> svuint8_t {
    svqshl_u8_m(pg, op1, svdup_n_s8(op2))
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_u8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshl))]
pub fn svqshl_u8_x(pg: svbool_t, op1: svuint8_t, op2: svint8_t) -> svuint8_t {
    svqshl_u8_m(pg, op1, op2)
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_n_u8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshl))]
pub fn svqshl_n_u8_x(pg: svbool_t, op1: svuint8_t, op2: i8) -> svuint8_t {
    svqshl_u8_x(pg, op1, svdup_n_s8(op2))
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_u8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshl))]
pub fn svqshl_u8_z(pg: svbool_t, op1: svuint8_t, op2: svint8_t) -> svuint8_t {
    svqshl_u8_m(pg, svsel_u8(pg, op1, svdup_n_u8(0)), op2)
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_n_u8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshl))]
pub fn svqshl_n_u8_z(pg: svbool_t, op1: svuint8_t, op2: i8) -> svuint8_t {
    svqshl_u8_z(pg, op1, svdup_n_s8(op2))
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_u16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshl))]
pub fn svqshl_u16_m(pg: svbool_t, op1: svuint16_t, op2: svint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uqshl.nxv8i16")]
        fn _svqshl_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svqshl_u16_m(pg.into(), op1.as_signed(), op2).as_unsigned() }
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_n_u16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshl))]
pub fn svqshl_n_u16_m(pg: svbool_t, op1: svuint16_t, op2: i16) -> svuint16_t {
    svqshl_u16_m(pg, op1, svdup_n_s16(op2))
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_u16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshl))]
pub fn svqshl_u16_x(pg: svbool_t, op1: svuint16_t, op2: svint16_t) -> svuint16_t {
    svqshl_u16_m(pg, op1, op2)
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_n_u16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshl))]
pub fn svqshl_n_u16_x(pg: svbool_t, op1: svuint16_t, op2: i16) -> svuint16_t {
    svqshl_u16_x(pg, op1, svdup_n_s16(op2))
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_u16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshl))]
pub fn svqshl_u16_z(pg: svbool_t, op1: svuint16_t, op2: svint16_t) -> svuint16_t {
    svqshl_u16_m(pg, svsel_u16(pg, op1, svdup_n_u16(0)), op2)
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_n_u16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshl))]
pub fn svqshl_n_u16_z(pg: svbool_t, op1: svuint16_t, op2: i16) -> svuint16_t {
    svqshl_u16_z(pg, op1, svdup_n_s16(op2))
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshl))]
pub fn svqshl_u32_m(pg: svbool_t, op1: svuint32_t, op2: svint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uqshl.nxv4i32")]
        fn _svqshl_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svqshl_u32_m(pg.into(), op1.as_signed(), op2).as_unsigned() }
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_n_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshl))]
pub fn svqshl_n_u32_m(pg: svbool_t, op1: svuint32_t, op2: i32) -> svuint32_t {
    svqshl_u32_m(pg, op1, svdup_n_s32(op2))
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshl))]
pub fn svqshl_u32_x(pg: svbool_t, op1: svuint32_t, op2: svint32_t) -> svuint32_t {
    svqshl_u32_m(pg, op1, op2)
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_n_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshl))]
pub fn svqshl_n_u32_x(pg: svbool_t, op1: svuint32_t, op2: i32) -> svuint32_t {
    svqshl_u32_x(pg, op1, svdup_n_s32(op2))
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_u32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshl))]
pub fn svqshl_u32_z(pg: svbool_t, op1: svuint32_t, op2: svint32_t) -> svuint32_t {
    svqshl_u32_m(pg, svsel_u32(pg, op1, svdup_n_u32(0)), op2)
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_n_u32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshl))]
pub fn svqshl_n_u32_z(pg: svbool_t, op1: svuint32_t, op2: i32) -> svuint32_t {
    svqshl_u32_z(pg, op1, svdup_n_s32(op2))
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_u64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshl))]
pub fn svqshl_u64_m(pg: svbool_t, op1: svuint64_t, op2: svint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uqshl.nxv2i64")]
        fn _svqshl_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svqshl_u64_m(pg.into(), op1.as_signed(), op2).as_unsigned() }
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_n_u64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshl))]
pub fn svqshl_n_u64_m(pg: svbool_t, op1: svuint64_t, op2: i64) -> svuint64_t {
    svqshl_u64_m(pg, op1, svdup_n_s64(op2))
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_u64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshl))]
pub fn svqshl_u64_x(pg: svbool_t, op1: svuint64_t, op2: svint64_t) -> svuint64_t {
    svqshl_u64_m(pg, op1, op2)
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_n_u64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshl))]
pub fn svqshl_n_u64_x(pg: svbool_t, op1: svuint64_t, op2: i64) -> svuint64_t {
    svqshl_u64_x(pg, op1, svdup_n_s64(op2))
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_u64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshl))]
pub fn svqshl_u64_z(pg: svbool_t, op1: svuint64_t, op2: svint64_t) -> svuint64_t {
    svqshl_u64_m(pg, svsel_u64(pg, op1, svdup_n_u64(0)), op2)
}
#[doc = "Saturating shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshl[_n_u64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshl))]
pub fn svqshl_n_u64_z(pg: svbool_t, op1: svuint64_t, op2: i64) -> svuint64_t {
    svqshl_u64_z(pg, op1, svdup_n_s64(op2))
}
#[doc = "Saturating shift left unsigned"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshlu[_n_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshlu, IMM2 = 0))]
pub fn svqshlu_n_s8_m<const IMM2: i32>(pg: svbool_t, op1: svint8_t) -> svuint8_t {
    static_assert_range!(IMM2, 0, 7);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqshlu.nxv16i8")]
        fn _svqshlu_n_s8_m(pg: svbool_t, op1: svint8_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svqshlu_n_s8_m(pg, op1, IMM2).as_unsigned() }
}
#[doc = "Saturating shift left unsigned"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshlu[_n_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshlu, IMM2 = 0))]
pub fn svqshlu_n_s8_x<const IMM2: i32>(pg: svbool_t, op1: svint8_t) -> svuint8_t {
    svqshlu_n_s8_m::<IMM2>(pg, op1)
}
#[doc = "Saturating shift left unsigned"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshlu[_n_s8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshlu, IMM2 = 0))]
pub fn svqshlu_n_s8_z<const IMM2: i32>(pg: svbool_t, op1: svint8_t) -> svuint8_t {
    svqshlu_n_s8_m::<IMM2>(pg, svsel_s8(pg, op1, svdup_n_s8(0)))
}
#[doc = "Saturating shift left unsigned"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshlu[_n_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshlu, IMM2 = 0))]
pub fn svqshlu_n_s16_m<const IMM2: i32>(pg: svbool_t, op1: svint16_t) -> svuint16_t {
    static_assert_range!(IMM2, 0, 15);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqshlu.nxv8i16")]
        fn _svqshlu_n_s16_m(pg: svbool8_t, op1: svint16_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svqshlu_n_s16_m(pg.into(), op1, IMM2).as_unsigned() }
}
#[doc = "Saturating shift left unsigned"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshlu[_n_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshlu, IMM2 = 0))]
pub fn svqshlu_n_s16_x<const IMM2: i32>(pg: svbool_t, op1: svint16_t) -> svuint16_t {
    svqshlu_n_s16_m::<IMM2>(pg, op1)
}
#[doc = "Saturating shift left unsigned"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshlu[_n_s16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshlu, IMM2 = 0))]
pub fn svqshlu_n_s16_z<const IMM2: i32>(pg: svbool_t, op1: svint16_t) -> svuint16_t {
    svqshlu_n_s16_m::<IMM2>(pg, svsel_s16(pg, op1, svdup_n_s16(0)))
}
#[doc = "Saturating shift left unsigned"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshlu[_n_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshlu, IMM2 = 0))]
pub fn svqshlu_n_s32_m<const IMM2: i32>(pg: svbool_t, op1: svint32_t) -> svuint32_t {
    static_assert_range!(IMM2, 0, 31);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqshlu.nxv4i32")]
        fn _svqshlu_n_s32_m(pg: svbool4_t, op1: svint32_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svqshlu_n_s32_m(pg.into(), op1, IMM2).as_unsigned() }
}
#[doc = "Saturating shift left unsigned"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshlu[_n_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshlu, IMM2 = 0))]
pub fn svqshlu_n_s32_x<const IMM2: i32>(pg: svbool_t, op1: svint32_t) -> svuint32_t {
    svqshlu_n_s32_m::<IMM2>(pg, op1)
}
#[doc = "Saturating shift left unsigned"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshlu[_n_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshlu, IMM2 = 0))]
pub fn svqshlu_n_s32_z<const IMM2: i32>(pg: svbool_t, op1: svint32_t) -> svuint32_t {
    svqshlu_n_s32_m::<IMM2>(pg, svsel_s32(pg, op1, svdup_n_s32(0)))
}
#[doc = "Saturating shift left unsigned"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshlu[_n_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshlu, IMM2 = 0))]
pub fn svqshlu_n_s64_m<const IMM2: i32>(pg: svbool_t, op1: svint64_t) -> svuint64_t {
    static_assert_range!(IMM2, 0, 63);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqshlu.nxv2i64")]
        fn _svqshlu_n_s64_m(pg: svbool2_t, op1: svint64_t, imm2: i32) -> svint64_t;
    }
    unsafe { _svqshlu_n_s64_m(pg.into(), op1, IMM2).as_unsigned() }
}
#[doc = "Saturating shift left unsigned"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshlu[_n_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshlu, IMM2 = 0))]
pub fn svqshlu_n_s64_x<const IMM2: i32>(pg: svbool_t, op1: svint64_t) -> svuint64_t {
    svqshlu_n_s64_m::<IMM2>(pg, op1)
}
#[doc = "Saturating shift left unsigned"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshlu[_n_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshlu, IMM2 = 0))]
pub fn svqshlu_n_s64_z<const IMM2: i32>(pg: svbool_t, op1: svint64_t) -> svuint64_t {
    svqshlu_n_s64_m::<IMM2>(pg, svsel_s64(pg, op1, svdup_n_s64(0)))
}
#[doc = "Saturating shift right narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshrnb[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshrnb, IMM2 = 1))]
pub fn svqshrnb_n_s16<const IMM2: i32>(op1: svint16_t) -> svint8_t {
    static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqshrnb.nxv8i16"
        )]
        fn _svqshrnb_n_s16(op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svqshrnb_n_s16(op1, IMM2) }
}
#[doc = "Saturating shift right narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshrnb[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshrnb, IMM2 = 1))]
pub fn svqshrnb_n_s32<const IMM2: i32>(op1: svint32_t) -> svint16_t {
    static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqshrnb.nxv4i32"
        )]
        fn _svqshrnb_n_s32(op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svqshrnb_n_s32(op1, IMM2) }
}
#[doc = "Saturating shift right narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshrnb[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshrnb, IMM2 = 1))]
pub fn svqshrnb_n_s64<const IMM2: i32>(op1: svint64_t) -> svint32_t {
    static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqshrnb.nxv2i64"
        )]
        fn _svqshrnb_n_s64(op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svqshrnb_n_s64(op1, IMM2) }
}
#[doc = "Saturating shift right narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshrnb[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshrnb, IMM2 = 1))]
pub fn svqshrnb_n_u16<const IMM2: i32>(op1: svuint16_t) -> svuint8_t {
    static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.uqshrnb.nxv8i16"
        )]
        fn _svqshrnb_n_u16(op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svqshrnb_n_u16(op1.as_signed(), IMM2).as_unsigned() }
}
#[doc = "Saturating shift right narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshrnb[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshrnb, IMM2 = 1))]
pub fn svqshrnb_n_u32<const IMM2: i32>(op1: svuint32_t) -> svuint16_t {
    static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.uqshrnb.nxv4i32"
        )]
        fn _svqshrnb_n_u32(op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svqshrnb_n_u32(op1.as_signed(), IMM2).as_unsigned() }
}
#[doc = "Saturating shift right narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshrnb[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshrnb, IMM2 = 1))]
pub fn svqshrnb_n_u64<const IMM2: i32>(op1: svuint64_t) -> svuint32_t {
    static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.uqshrnb.nxv2i64"
        )]
        fn _svqshrnb_n_u64(op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svqshrnb_n_u64(op1.as_signed(), IMM2).as_unsigned() }
}
#[doc = "Saturating shift right narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshrnt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshrnt, IMM2 = 1))]
pub fn svqshrnt_n_s16<const IMM2: i32>(even: svint8_t, op1: svint16_t) -> svint8_t {
    static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqshrnt.nxv8i16"
        )]
        fn _svqshrnt_n_s16(even: svint8_t, op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svqshrnt_n_s16(even, op1, IMM2) }
}
#[doc = "Saturating shift right narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshrnt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshrnt, IMM2 = 1))]
pub fn svqshrnt_n_s32<const IMM2: i32>(even: svint16_t, op1: svint32_t) -> svint16_t {
    static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqshrnt.nxv4i32"
        )]
        fn _svqshrnt_n_s32(even: svint16_t, op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svqshrnt_n_s32(even, op1, IMM2) }
}
#[doc = "Saturating shift right narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshrnt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshrnt, IMM2 = 1))]
pub fn svqshrnt_n_s64<const IMM2: i32>(even: svint32_t, op1: svint64_t) -> svint32_t {
    static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqshrnt.nxv2i64"
        )]
        fn _svqshrnt_n_s64(even: svint32_t, op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svqshrnt_n_s64(even, op1, IMM2) }
}
#[doc = "Saturating shift right narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshrnt[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshrnt, IMM2 = 1))]
pub fn svqshrnt_n_u16<const IMM2: i32>(even: svuint8_t, op1: svuint16_t) -> svuint8_t {
    static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.uqshrnt.nxv8i16"
        )]
        fn _svqshrnt_n_u16(even: svint8_t, op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svqshrnt_n_u16(even.as_signed(), op1.as_signed(), IMM2).as_unsigned() }
}
#[doc = "Saturating shift right narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshrnt[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshrnt, IMM2 = 1))]
pub fn svqshrnt_n_u32<const IMM2: i32>(even: svuint16_t, op1: svuint32_t) -> svuint16_t {
    static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.uqshrnt.nxv4i32"
        )]
        fn _svqshrnt_n_u32(even: svint16_t, op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svqshrnt_n_u32(even.as_signed(), op1.as_signed(), IMM2).as_unsigned() }
}
#[doc = "Saturating shift right narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshrnt[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqshrnt, IMM2 = 1))]
pub fn svqshrnt_n_u64<const IMM2: i32>(even: svuint32_t, op1: svuint64_t) -> svuint32_t {
    static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.uqshrnt.nxv2i64"
        )]
        fn _svqshrnt_n_u64(even: svint32_t, op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svqshrnt_n_u64(even.as_signed(), op1.as_signed(), IMM2).as_unsigned() }
}
#[doc = "Saturating shift right unsigned narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshrunb[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshrunb, IMM2 = 1))]
pub fn svqshrunb_n_s16<const IMM2: i32>(op1: svint16_t) -> svuint8_t {
    static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqshrunb.nxv8i16"
        )]
        fn _svqshrunb_n_s16(op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svqshrunb_n_s16(op1, IMM2).as_unsigned() }
}
#[doc = "Saturating shift right unsigned narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshrunb[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshrunb, IMM2 = 1))]
pub fn svqshrunb_n_s32<const IMM2: i32>(op1: svint32_t) -> svuint16_t {
    static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqshrunb.nxv4i32"
        )]
        fn _svqshrunb_n_s32(op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svqshrunb_n_s32(op1, IMM2).as_unsigned() }
}
#[doc = "Saturating shift right unsigned narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshrunb[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshrunb, IMM2 = 1))]
pub fn svqshrunb_n_s64<const IMM2: i32>(op1: svint64_t) -> svuint32_t {
    static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqshrunb.nxv2i64"
        )]
        fn _svqshrunb_n_s64(op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svqshrunb_n_s64(op1, IMM2).as_unsigned() }
}
#[doc = "Saturating shift right unsigned narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshrunt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshrunt, IMM2 = 1))]
pub fn svqshrunt_n_s16<const IMM2: i32>(even: svuint8_t, op1: svint16_t) -> svuint8_t {
    static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqshrunt.nxv8i16"
        )]
        fn _svqshrunt_n_s16(even: svint8_t, op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svqshrunt_n_s16(even.as_signed(), op1, IMM2).as_unsigned() }
}
#[doc = "Saturating shift right unsigned narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshrunt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshrunt, IMM2 = 1))]
pub fn svqshrunt_n_s32<const IMM2: i32>(even: svuint16_t, op1: svint32_t) -> svuint16_t {
    static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqshrunt.nxv4i32"
        )]
        fn _svqshrunt_n_s32(even: svint16_t, op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svqshrunt_n_s32(even.as_signed(), op1, IMM2).as_unsigned() }
}
#[doc = "Saturating shift right unsigned narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqshrunt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqshrunt, IMM2 = 1))]
pub fn svqshrunt_n_s64<const IMM2: i32>(even: svuint32_t, op1: svint64_t) -> svuint32_t {
    static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqshrunt.nxv2i64"
        )]
        fn _svqshrunt_n_s64(even: svint32_t, op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svqshrunt_n_s64(even.as_signed(), op1, IMM2).as_unsigned() }
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsub))]
pub fn svqsub_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqsub.nxv16i8")]
        fn _svqsub_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svqsub_s8_m(pg, op1, op2) }
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_n_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsub))]
pub fn svqsub_n_s8_m(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqsub_s8_m(pg, op1, svdup_n_s8(op2))
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsub))]
pub fn svqsub_s8_x(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svqsub_s8_m(pg, op1, op2)
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_n_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsub))]
pub fn svqsub_n_s8_x(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqsub_s8_x(pg, op1, svdup_n_s8(op2))
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_s8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsub))]
pub fn svqsub_s8_z(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svqsub_s8_m(pg, svsel_s8(pg, op1, svdup_n_s8(0)), op2)
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_n_s8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsub))]
pub fn svqsub_n_s8_z(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqsub_s8_z(pg, op1, svdup_n_s8(op2))
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsub))]
pub fn svqsub_s16_m(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqsub.nxv8i16")]
        fn _svqsub_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svqsub_s16_m(pg.into(), op1, op2) }
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_n_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsub))]
pub fn svqsub_n_s16_m(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqsub_s16_m(pg, op1, svdup_n_s16(op2))
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsub))]
pub fn svqsub_s16_x(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svqsub_s16_m(pg, op1, op2)
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_n_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsub))]
pub fn svqsub_n_s16_x(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqsub_s16_x(pg, op1, svdup_n_s16(op2))
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_s16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsub))]
pub fn svqsub_s16_z(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svqsub_s16_m(pg, svsel_s16(pg, op1, svdup_n_s16(0)), op2)
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_n_s16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsub))]
pub fn svqsub_n_s16_z(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqsub_s16_z(pg, op1, svdup_n_s16(op2))
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsub))]
pub fn svqsub_s32_m(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqsub.nxv4i32")]
        fn _svqsub_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svqsub_s32_m(pg.into(), op1, op2) }
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_n_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsub))]
pub fn svqsub_n_s32_m(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqsub_s32_m(pg, op1, svdup_n_s32(op2))
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsub))]
pub fn svqsub_s32_x(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svqsub_s32_m(pg, op1, op2)
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_n_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsub))]
pub fn svqsub_n_s32_x(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqsub_s32_x(pg, op1, svdup_n_s32(op2))
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsub))]
pub fn svqsub_s32_z(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svqsub_s32_m(pg, svsel_s32(pg, op1, svdup_n_s32(0)), op2)
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_n_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsub))]
pub fn svqsub_n_s32_z(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqsub_s32_z(pg, op1, svdup_n_s32(op2))
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsub))]
pub fn svqsub_s64_m(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqsub.nxv2i64")]
        fn _svqsub_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svqsub_s64_m(pg.into(), op1, op2) }
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_n_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsub))]
pub fn svqsub_n_s64_m(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqsub_s64_m(pg, op1, svdup_n_s64(op2))
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsub))]
pub fn svqsub_s64_x(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svqsub_s64_m(pg, op1, op2)
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_n_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsub))]
pub fn svqsub_n_s64_x(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqsub_s64_x(pg, op1, svdup_n_s64(op2))
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsub))]
pub fn svqsub_s64_z(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svqsub_s64_m(pg, svsel_s64(pg, op1, svdup_n_s64(0)), op2)
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_n_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsub))]
pub fn svqsub_n_s64_z(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqsub_s64_z(pg, op1, svdup_n_s64(op2))
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_u8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsub))]
pub fn svqsub_u8_m(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uqsub.nxv16i8")]
        fn _svqsub_u8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svqsub_u8_m(pg, op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_n_u8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsub))]
pub fn svqsub_n_u8_m(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svqsub_u8_m(pg, op1, svdup_n_u8(op2))
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_u8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsub))]
pub fn svqsub_u8_x(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svqsub_u8_m(pg, op1, op2)
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_n_u8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsub))]
pub fn svqsub_n_u8_x(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svqsub_u8_x(pg, op1, svdup_n_u8(op2))
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_u8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsub))]
pub fn svqsub_u8_z(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svqsub_u8_m(pg, svsel_u8(pg, op1, svdup_n_u8(0)), op2)
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_n_u8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsub))]
pub fn svqsub_n_u8_z(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svqsub_u8_z(pg, op1, svdup_n_u8(op2))
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_u16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsub))]
pub fn svqsub_u16_m(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uqsub.nxv8i16")]
        fn _svqsub_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svqsub_u16_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_n_u16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsub))]
pub fn svqsub_n_u16_m(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svqsub_u16_m(pg, op1, svdup_n_u16(op2))
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_u16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsub))]
pub fn svqsub_u16_x(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svqsub_u16_m(pg, op1, op2)
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_n_u16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsub))]
pub fn svqsub_n_u16_x(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svqsub_u16_x(pg, op1, svdup_n_u16(op2))
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_u16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsub))]
pub fn svqsub_u16_z(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svqsub_u16_m(pg, svsel_u16(pg, op1, svdup_n_u16(0)), op2)
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_n_u16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsub))]
pub fn svqsub_n_u16_z(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svqsub_u16_z(pg, op1, svdup_n_u16(op2))
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsub))]
pub fn svqsub_u32_m(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uqsub.nxv4i32")]
        fn _svqsub_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svqsub_u32_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_n_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsub))]
pub fn svqsub_n_u32_m(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svqsub_u32_m(pg, op1, svdup_n_u32(op2))
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsub))]
pub fn svqsub_u32_x(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svqsub_u32_m(pg, op1, op2)
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_n_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsub))]
pub fn svqsub_n_u32_x(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svqsub_u32_x(pg, op1, svdup_n_u32(op2))
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_u32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsub))]
pub fn svqsub_u32_z(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svqsub_u32_m(pg, svsel_u32(pg, op1, svdup_n_u32(0)), op2)
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_n_u32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsub))]
pub fn svqsub_n_u32_z(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svqsub_u32_z(pg, op1, svdup_n_u32(op2))
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_u64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsub))]
pub fn svqsub_u64_m(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uqsub.nxv2i64")]
        fn _svqsub_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svqsub_u64_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_n_u64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsub))]
pub fn svqsub_n_u64_m(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svqsub_u64_m(pg, op1, svdup_n_u64(op2))
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_u64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsub))]
pub fn svqsub_u64_x(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svqsub_u64_m(pg, op1, op2)
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_n_u64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsub))]
pub fn svqsub_n_u64_x(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svqsub_u64_x(pg, op1, svdup_n_u64(op2))
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_u64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsub))]
pub fn svqsub_u64_z(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svqsub_u64_m(pg, svsel_u64(pg, op1, svdup_n_u64(0)), op2)
}
#[doc = "Saturating subtract"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsub[_n_u64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsub))]
pub fn svqsub_n_u64_z(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svqsub_u64_z(pg, op1, svdup_n_u64(op2))
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsubr))]
pub fn svqsubr_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqsubr.nxv16i8")]
        fn _svqsubr_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svqsubr_s8_m(pg, op1, op2) }
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_n_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsubr))]
pub fn svqsubr_n_s8_m(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqsubr_s8_m(pg, op1, svdup_n_s8(op2))
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsubr))]
pub fn svqsubr_s8_x(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svqsubr_s8_m(pg, op1, op2)
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_n_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsubr))]
pub fn svqsubr_n_s8_x(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqsubr_s8_x(pg, op1, svdup_n_s8(op2))
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_s8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsubr))]
pub fn svqsubr_s8_z(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svqsubr_s8_m(pg, svsel_s8(pg, op1, svdup_n_s8(0)), op2)
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_n_s8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsubr))]
pub fn svqsubr_n_s8_z(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqsubr_s8_z(pg, op1, svdup_n_s8(op2))
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsubr))]
pub fn svqsubr_s16_m(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqsubr.nxv8i16")]
        fn _svqsubr_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svqsubr_s16_m(pg.into(), op1, op2) }
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_n_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsubr))]
pub fn svqsubr_n_s16_m(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqsubr_s16_m(pg, op1, svdup_n_s16(op2))
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsubr))]
pub fn svqsubr_s16_x(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svqsubr_s16_m(pg, op1, op2)
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_n_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsubr))]
pub fn svqsubr_n_s16_x(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqsubr_s16_x(pg, op1, svdup_n_s16(op2))
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_s16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsubr))]
pub fn svqsubr_s16_z(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svqsubr_s16_m(pg, svsel_s16(pg, op1, svdup_n_s16(0)), op2)
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_n_s16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsubr))]
pub fn svqsubr_n_s16_z(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqsubr_s16_z(pg, op1, svdup_n_s16(op2))
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsubr))]
pub fn svqsubr_s32_m(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqsubr.nxv4i32")]
        fn _svqsubr_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svqsubr_s32_m(pg.into(), op1, op2) }
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_n_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsubr))]
pub fn svqsubr_n_s32_m(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqsubr_s32_m(pg, op1, svdup_n_s32(op2))
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsubr))]
pub fn svqsubr_s32_x(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svqsubr_s32_m(pg, op1, op2)
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_n_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsubr))]
pub fn svqsubr_n_s32_x(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqsubr_s32_x(pg, op1, svdup_n_s32(op2))
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsubr))]
pub fn svqsubr_s32_z(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svqsubr_s32_m(pg, svsel_s32(pg, op1, svdup_n_s32(0)), op2)
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_n_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsubr))]
pub fn svqsubr_n_s32_z(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqsubr_s32_z(pg, op1, svdup_n_s32(op2))
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsubr))]
pub fn svqsubr_s64_m(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqsubr.nxv2i64")]
        fn _svqsubr_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svqsubr_s64_m(pg.into(), op1, op2) }
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_n_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsubr))]
pub fn svqsubr_n_s64_m(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqsubr_s64_m(pg, op1, svdup_n_s64(op2))
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsubr))]
pub fn svqsubr_s64_x(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svqsubr_s64_m(pg, op1, op2)
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_n_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsubr))]
pub fn svqsubr_n_s64_x(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqsubr_s64_x(pg, op1, svdup_n_s64(op2))
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsubr))]
pub fn svqsubr_s64_z(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svqsubr_s64_m(pg, svsel_s64(pg, op1, svdup_n_s64(0)), op2)
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_n_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqsubr))]
pub fn svqsubr_n_s64_z(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqsubr_s64_z(pg, op1, svdup_n_s64(op2))
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_u8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsubr))]
pub fn svqsubr_u8_m(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uqsubr.nxv16i8")]
        fn _svqsubr_u8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svqsubr_u8_m(pg, op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_n_u8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsubr))]
pub fn svqsubr_n_u8_m(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svqsubr_u8_m(pg, op1, svdup_n_u8(op2))
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_u8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsubr))]
pub fn svqsubr_u8_x(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svqsubr_u8_m(pg, op1, op2)
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_n_u8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsubr))]
pub fn svqsubr_n_u8_x(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svqsubr_u8_x(pg, op1, svdup_n_u8(op2))
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_u8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsubr))]
pub fn svqsubr_u8_z(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svqsubr_u8_m(pg, svsel_u8(pg, op1, svdup_n_u8(0)), op2)
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_n_u8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsubr))]
pub fn svqsubr_n_u8_z(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svqsubr_u8_z(pg, op1, svdup_n_u8(op2))
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_u16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsubr))]
pub fn svqsubr_u16_m(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uqsubr.nxv8i16")]
        fn _svqsubr_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svqsubr_u16_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_n_u16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsubr))]
pub fn svqsubr_n_u16_m(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svqsubr_u16_m(pg, op1, svdup_n_u16(op2))
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_u16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsubr))]
pub fn svqsubr_u16_x(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svqsubr_u16_m(pg, op1, op2)
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_n_u16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsubr))]
pub fn svqsubr_n_u16_x(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svqsubr_u16_x(pg, op1, svdup_n_u16(op2))
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_u16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsubr))]
pub fn svqsubr_u16_z(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svqsubr_u16_m(pg, svsel_u16(pg, op1, svdup_n_u16(0)), op2)
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_n_u16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsubr))]
pub fn svqsubr_n_u16_z(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svqsubr_u16_z(pg, op1, svdup_n_u16(op2))
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsubr))]
pub fn svqsubr_u32_m(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uqsubr.nxv4i32")]
        fn _svqsubr_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svqsubr_u32_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_n_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsubr))]
pub fn svqsubr_n_u32_m(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svqsubr_u32_m(pg, op1, svdup_n_u32(op2))
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsubr))]
pub fn svqsubr_u32_x(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svqsubr_u32_m(pg, op1, op2)
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_n_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsubr))]
pub fn svqsubr_n_u32_x(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svqsubr_u32_x(pg, op1, svdup_n_u32(op2))
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_u32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsubr))]
pub fn svqsubr_u32_z(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svqsubr_u32_m(pg, svsel_u32(pg, op1, svdup_n_u32(0)), op2)
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_n_u32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsubr))]
pub fn svqsubr_n_u32_z(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svqsubr_u32_z(pg, op1, svdup_n_u32(op2))
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_u64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsubr))]
pub fn svqsubr_u64_m(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uqsubr.nxv2i64")]
        fn _svqsubr_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svqsubr_u64_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_n_u64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsubr))]
pub fn svqsubr_n_u64_m(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svqsubr_u64_m(pg, op1, svdup_n_u64(op2))
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_u64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsubr))]
pub fn svqsubr_u64_x(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svqsubr_u64_m(pg, op1, op2)
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_n_u64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsubr))]
pub fn svqsubr_n_u64_x(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svqsubr_u64_x(pg, op1, svdup_n_u64(op2))
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_u64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsubr))]
pub fn svqsubr_u64_z(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svqsubr_u64_m(pg, svsel_u64(pg, op1, svdup_n_u64(0)), op2)
}
#[doc = "Saturating subtract reversed"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqsubr[_n_u64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqsubr))]
pub fn svqsubr_n_u64_z(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svqsubr_u64_z(pg, op1, svdup_n_u64(op2))
}
#[doc = "Saturating extract narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqxtnb[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqxtnb))]
pub fn svqxtnb_s16(op: svint16_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqxtnb.nxv8i16")]
        fn _svqxtnb_s16(op: svint16_t) -> svint8_t;
    }
    unsafe { _svqxtnb_s16(op) }
}
#[doc = "Saturating extract narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqxtnb[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqxtnb))]
pub fn svqxtnb_s32(op: svint32_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqxtnb.nxv4i32")]
        fn _svqxtnb_s32(op: svint32_t) -> svint16_t;
    }
    unsafe { _svqxtnb_s32(op) }
}
#[doc = "Saturating extract narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqxtnb[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqxtnb))]
pub fn svqxtnb_s64(op: svint64_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqxtnb.nxv2i64")]
        fn _svqxtnb_s64(op: svint64_t) -> svint32_t;
    }
    unsafe { _svqxtnb_s64(op) }
}
#[doc = "Saturating extract narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqxtnb[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqxtnb))]
pub fn svqxtnb_u16(op: svuint16_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uqxtnb.nxv8i16")]
        fn _svqxtnb_u16(op: svint16_t) -> svint8_t;
    }
    unsafe { _svqxtnb_u16(op.as_signed()).as_unsigned() }
}
#[doc = "Saturating extract narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqxtnb[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqxtnb))]
pub fn svqxtnb_u32(op: svuint32_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uqxtnb.nxv4i32")]
        fn _svqxtnb_u32(op: svint32_t) -> svint16_t;
    }
    unsafe { _svqxtnb_u32(op.as_signed()).as_unsigned() }
}
#[doc = "Saturating extract narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqxtnb[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqxtnb))]
pub fn svqxtnb_u64(op: svuint64_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uqxtnb.nxv2i64")]
        fn _svqxtnb_u64(op: svint64_t) -> svint32_t;
    }
    unsafe { _svqxtnb_u64(op.as_signed()).as_unsigned() }
}
#[doc = "Saturating extract narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqxtnt[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqxtnt))]
pub fn svqxtnt_s16(even: svint8_t, op: svint16_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqxtnt.nxv8i16")]
        fn _svqxtnt_s16(even: svint8_t, op: svint16_t) -> svint8_t;
    }
    unsafe { _svqxtnt_s16(even, op) }
}
#[doc = "Saturating extract narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqxtnt[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqxtnt))]
pub fn svqxtnt_s32(even: svint16_t, op: svint32_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqxtnt.nxv4i32")]
        fn _svqxtnt_s32(even: svint16_t, op: svint32_t) -> svint16_t;
    }
    unsafe { _svqxtnt_s32(even, op) }
}
#[doc = "Saturating extract narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqxtnt[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqxtnt))]
pub fn svqxtnt_s64(even: svint32_t, op: svint64_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sqxtnt.nxv2i64")]
        fn _svqxtnt_s64(even: svint32_t, op: svint64_t) -> svint32_t;
    }
    unsafe { _svqxtnt_s64(even, op) }
}
#[doc = "Saturating extract narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqxtnt[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqxtnt))]
pub fn svqxtnt_u16(even: svuint8_t, op: svuint16_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uqxtnt.nxv8i16")]
        fn _svqxtnt_u16(even: svint8_t, op: svint16_t) -> svint8_t;
    }
    unsafe { _svqxtnt_u16(even.as_signed(), op.as_signed()).as_unsigned() }
}
#[doc = "Saturating extract narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqxtnt[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqxtnt))]
pub fn svqxtnt_u32(even: svuint16_t, op: svuint32_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uqxtnt.nxv4i32")]
        fn _svqxtnt_u32(even: svint16_t, op: svint32_t) -> svint16_t;
    }
    unsafe { _svqxtnt_u32(even.as_signed(), op.as_signed()).as_unsigned() }
}
#[doc = "Saturating extract narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqxtnt[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uqxtnt))]
pub fn svqxtnt_u64(even: svuint32_t, op: svuint64_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.uqxtnt.nxv2i64")]
        fn _svqxtnt_u64(even: svint32_t, op: svint64_t) -> svint32_t;
    }
    unsafe { _svqxtnt_u64(even.as_signed(), op.as_signed()).as_unsigned() }
}
#[doc = "Saturating extract unsigned narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqxtunb[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqxtunb))]
pub fn svqxtunb_s16(op: svint16_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqxtunb.nxv8i16"
        )]
        fn _svqxtunb_s16(op: svint16_t) -> svint8_t;
    }
    unsafe { _svqxtunb_s16(op).as_unsigned() }
}
#[doc = "Saturating extract unsigned narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqxtunb[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqxtunb))]
pub fn svqxtunb_s32(op: svint32_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqxtunb.nxv4i32"
        )]
        fn _svqxtunb_s32(op: svint32_t) -> svint16_t;
    }
    unsafe { _svqxtunb_s32(op).as_unsigned() }
}
#[doc = "Saturating extract unsigned narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqxtunb[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqxtunb))]
pub fn svqxtunb_s64(op: svint64_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqxtunb.nxv2i64"
        )]
        fn _svqxtunb_s64(op: svint64_t) -> svint32_t;
    }
    unsafe { _svqxtunb_s64(op).as_unsigned() }
}
#[doc = "Saturating extract unsigned narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqxtunt[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqxtunt))]
pub fn svqxtunt_s16(even: svuint8_t, op: svint16_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqxtunt.nxv8i16"
        )]
        fn _svqxtunt_s16(even: svint8_t, op: svint16_t) -> svint8_t;
    }
    unsafe { _svqxtunt_s16(even.as_signed(), op).as_unsigned() }
}
#[doc = "Saturating extract unsigned narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqxtunt[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqxtunt))]
pub fn svqxtunt_s32(even: svuint16_t, op: svint32_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqxtunt.nxv4i32"
        )]
        fn _svqxtunt_s32(even: svint16_t, op: svint32_t) -> svint16_t;
    }
    unsafe { _svqxtunt_s32(even.as_signed(), op).as_unsigned() }
}
#[doc = "Saturating extract unsigned narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svqxtunt[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sqxtunt))]
pub fn svqxtunt_s64(even: svuint32_t, op: svint64_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sqxtunt.nxv2i64"
        )]
        fn _svqxtunt_s64(even: svint32_t, op: svint64_t) -> svint32_t;
    }
    unsafe { _svqxtunt_s64(even.as_signed(), op).as_unsigned() }
}
#[doc = "Rounding add narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svraddhnb[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(raddhnb))]
pub fn svraddhnb_s16(op1: svint16_t, op2: svint16_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.raddhnb.nxv8i16"
        )]
        fn _svraddhnb_s16(op1: svint16_t, op2: svint16_t) -> svint8_t;
    }
    unsafe { _svraddhnb_s16(op1, op2) }
}
#[doc = "Rounding add narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svraddhnb[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(raddhnb))]
pub fn svraddhnb_n_s16(op1: svint16_t, op2: i16) -> svint8_t {
    svraddhnb_s16(op1, svdup_n_s16(op2))
}
#[doc = "Rounding add narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svraddhnb[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(raddhnb))]
pub fn svraddhnb_s32(op1: svint32_t, op2: svint32_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.raddhnb.nxv4i32"
        )]
        fn _svraddhnb_s32(op1: svint32_t, op2: svint32_t) -> svint16_t;
    }
    unsafe { _svraddhnb_s32(op1, op2) }
}
#[doc = "Rounding add narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svraddhnb[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(raddhnb))]
pub fn svraddhnb_n_s32(op1: svint32_t, op2: i32) -> svint16_t {
    svraddhnb_s32(op1, svdup_n_s32(op2))
}
#[doc = "Rounding add narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svraddhnb[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(raddhnb))]
pub fn svraddhnb_s64(op1: svint64_t, op2: svint64_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.raddhnb.nxv2i64"
        )]
        fn _svraddhnb_s64(op1: svint64_t, op2: svint64_t) -> svint32_t;
    }
    unsafe { _svraddhnb_s64(op1, op2) }
}
#[doc = "Rounding add narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svraddhnb[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(raddhnb))]
pub fn svraddhnb_n_s64(op1: svint64_t, op2: i64) -> svint32_t {
    svraddhnb_s64(op1, svdup_n_s64(op2))
}
#[doc = "Rounding add narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svraddhnb[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(raddhnb))]
pub fn svraddhnb_u16(op1: svuint16_t, op2: svuint16_t) -> svuint8_t {
    unsafe { svraddhnb_s16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Rounding add narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svraddhnb[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(raddhnb))]
pub fn svraddhnb_n_u16(op1: svuint16_t, op2: u16) -> svuint8_t {
    svraddhnb_u16(op1, svdup_n_u16(op2))
}
#[doc = "Rounding add narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svraddhnb[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(raddhnb))]
pub fn svraddhnb_u32(op1: svuint32_t, op2: svuint32_t) -> svuint16_t {
    unsafe { svraddhnb_s32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Rounding add narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svraddhnb[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(raddhnb))]
pub fn svraddhnb_n_u32(op1: svuint32_t, op2: u32) -> svuint16_t {
    svraddhnb_u32(op1, svdup_n_u32(op2))
}
#[doc = "Rounding add narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svraddhnb[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(raddhnb))]
pub fn svraddhnb_u64(op1: svuint64_t, op2: svuint64_t) -> svuint32_t {
    unsafe { svraddhnb_s64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Rounding add narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svraddhnb[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(raddhnb))]
pub fn svraddhnb_n_u64(op1: svuint64_t, op2: u64) -> svuint32_t {
    svraddhnb_u64(op1, svdup_n_u64(op2))
}
#[doc = "Rounding add narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svraddhnt[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(raddhnt))]
pub fn svraddhnt_s16(even: svint8_t, op1: svint16_t, op2: svint16_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.raddhnt.nxv8i16"
        )]
        fn _svraddhnt_s16(even: svint8_t, op1: svint16_t, op2: svint16_t) -> svint8_t;
    }
    unsafe { _svraddhnt_s16(even, op1, op2) }
}
#[doc = "Rounding add narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svraddhnt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(raddhnt))]
pub fn svraddhnt_n_s16(even: svint8_t, op1: svint16_t, op2: i16) -> svint8_t {
    svraddhnt_s16(even, op1, svdup_n_s16(op2))
}
#[doc = "Rounding add narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svraddhnt[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(raddhnt))]
pub fn svraddhnt_s32(even: svint16_t, op1: svint32_t, op2: svint32_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.raddhnt.nxv4i32"
        )]
        fn _svraddhnt_s32(even: svint16_t, op1: svint32_t, op2: svint32_t) -> svint16_t;
    }
    unsafe { _svraddhnt_s32(even, op1, op2) }
}
#[doc = "Rounding add narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svraddhnt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(raddhnt))]
pub fn svraddhnt_n_s32(even: svint16_t, op1: svint32_t, op2: i32) -> svint16_t {
    svraddhnt_s32(even, op1, svdup_n_s32(op2))
}
#[doc = "Rounding add narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svraddhnt[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(raddhnt))]
pub fn svraddhnt_s64(even: svint32_t, op1: svint64_t, op2: svint64_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.raddhnt.nxv2i64"
        )]
        fn _svraddhnt_s64(even: svint32_t, op1: svint64_t, op2: svint64_t) -> svint32_t;
    }
    unsafe { _svraddhnt_s64(even, op1, op2) }
}
#[doc = "Rounding add narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svraddhnt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(raddhnt))]
pub fn svraddhnt_n_s64(even: svint32_t, op1: svint64_t, op2: i64) -> svint32_t {
    svraddhnt_s64(even, op1, svdup_n_s64(op2))
}
#[doc = "Rounding add narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svraddhnt[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(raddhnt))]
pub fn svraddhnt_u16(even: svuint8_t, op1: svuint16_t, op2: svuint16_t) -> svuint8_t {
    unsafe { svraddhnt_s16(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Rounding add narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svraddhnt[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(raddhnt))]
pub fn svraddhnt_n_u16(even: svuint8_t, op1: svuint16_t, op2: u16) -> svuint8_t {
    svraddhnt_u16(even, op1, svdup_n_u16(op2))
}
#[doc = "Rounding add narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svraddhnt[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(raddhnt))]
pub fn svraddhnt_u32(even: svuint16_t, op1: svuint32_t, op2: svuint32_t) -> svuint16_t {
    unsafe { svraddhnt_s32(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Rounding add narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svraddhnt[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(raddhnt))]
pub fn svraddhnt_n_u32(even: svuint16_t, op1: svuint32_t, op2: u32) -> svuint16_t {
    svraddhnt_u32(even, op1, svdup_n_u32(op2))
}
#[doc = "Rounding add narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svraddhnt[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(raddhnt))]
pub fn svraddhnt_u64(even: svuint32_t, op1: svuint64_t, op2: svuint64_t) -> svuint32_t {
    unsafe { svraddhnt_s64(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Rounding add narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svraddhnt[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(raddhnt))]
pub fn svraddhnt_n_u64(even: svuint32_t, op1: svuint64_t, op2: u64) -> svuint32_t {
    svraddhnt_u64(even, op1, svdup_n_u64(op2))
}
#[doc = "Bitwise rotate left by 1 and exclusive OR"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrax1[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-sha3")]
#[cfg_attr(test, assert_instr(rax1))]
pub fn svrax1_s64(op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.rax1")]
        fn _svrax1_s64(op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svrax1_s64(op1, op2) }
}
#[doc = "Bitwise rotate left by 1 and exclusive OR"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrax1[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-sha3")]
#[cfg_attr(test, assert_instr(rax1))]
pub fn svrax1_u64(op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    unsafe { svrax1_s64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Reciprocal estimate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrecpe[_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urecpe))]
pub fn svrecpe_u32_m(inactive: svuint32_t, pg: svbool_t, op: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.urecpe.nxv4i32")]
        fn _svrecpe_u32_m(inactive: svint32_t, pg: svbool4_t, op: svint32_t) -> svint32_t;
    }
    unsafe { _svrecpe_u32_m(inactive.as_signed(), pg.into(), op.as_signed()).as_unsigned() }
}
#[doc = "Reciprocal estimate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrecpe[_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urecpe))]
pub fn svrecpe_u32_x(pg: svbool_t, op: svuint32_t) -> svuint32_t {
    svrecpe_u32_m(op, pg, op)
}
#[doc = "Reciprocal estimate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrecpe[_u32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urecpe))]
pub fn svrecpe_u32_z(pg: svbool_t, op: svuint32_t) -> svuint32_t {
    svrecpe_u32_m(svdup_n_u32(0), pg, op)
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srhadd))]
pub fn svrhadd_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.srhadd.nxv16i8")]
        fn _svrhadd_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svrhadd_s8_m(pg, op1, op2) }
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_n_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srhadd))]
pub fn svrhadd_n_s8_m(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svrhadd_s8_m(pg, op1, svdup_n_s8(op2))
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srhadd))]
pub fn svrhadd_s8_x(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svrhadd_s8_m(pg, op1, op2)
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_n_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srhadd))]
pub fn svrhadd_n_s8_x(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svrhadd_s8_x(pg, op1, svdup_n_s8(op2))
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_s8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srhadd))]
pub fn svrhadd_s8_z(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svrhadd_s8_m(pg, svsel_s8(pg, op1, svdup_n_s8(0)), op2)
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_n_s8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srhadd))]
pub fn svrhadd_n_s8_z(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svrhadd_s8_z(pg, op1, svdup_n_s8(op2))
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srhadd))]
pub fn svrhadd_s16_m(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.srhadd.nxv8i16")]
        fn _svrhadd_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svrhadd_s16_m(pg.into(), op1, op2) }
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_n_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srhadd))]
pub fn svrhadd_n_s16_m(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svrhadd_s16_m(pg, op1, svdup_n_s16(op2))
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srhadd))]
pub fn svrhadd_s16_x(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svrhadd_s16_m(pg, op1, op2)
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_n_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srhadd))]
pub fn svrhadd_n_s16_x(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svrhadd_s16_x(pg, op1, svdup_n_s16(op2))
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_s16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srhadd))]
pub fn svrhadd_s16_z(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svrhadd_s16_m(pg, svsel_s16(pg, op1, svdup_n_s16(0)), op2)
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_n_s16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srhadd))]
pub fn svrhadd_n_s16_z(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svrhadd_s16_z(pg, op1, svdup_n_s16(op2))
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srhadd))]
pub fn svrhadd_s32_m(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.srhadd.nxv4i32")]
        fn _svrhadd_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svrhadd_s32_m(pg.into(), op1, op2) }
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_n_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srhadd))]
pub fn svrhadd_n_s32_m(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svrhadd_s32_m(pg, op1, svdup_n_s32(op2))
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srhadd))]
pub fn svrhadd_s32_x(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svrhadd_s32_m(pg, op1, op2)
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_n_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srhadd))]
pub fn svrhadd_n_s32_x(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svrhadd_s32_x(pg, op1, svdup_n_s32(op2))
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srhadd))]
pub fn svrhadd_s32_z(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svrhadd_s32_m(pg, svsel_s32(pg, op1, svdup_n_s32(0)), op2)
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_n_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srhadd))]
pub fn svrhadd_n_s32_z(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svrhadd_s32_z(pg, op1, svdup_n_s32(op2))
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srhadd))]
pub fn svrhadd_s64_m(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.srhadd.nxv2i64")]
        fn _svrhadd_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svrhadd_s64_m(pg.into(), op1, op2) }
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_n_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srhadd))]
pub fn svrhadd_n_s64_m(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svrhadd_s64_m(pg, op1, svdup_n_s64(op2))
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srhadd))]
pub fn svrhadd_s64_x(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svrhadd_s64_m(pg, op1, op2)
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_n_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srhadd))]
pub fn svrhadd_n_s64_x(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svrhadd_s64_x(pg, op1, svdup_n_s64(op2))
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srhadd))]
pub fn svrhadd_s64_z(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svrhadd_s64_m(pg, svsel_s64(pg, op1, svdup_n_s64(0)), op2)
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_n_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srhadd))]
pub fn svrhadd_n_s64_z(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svrhadd_s64_z(pg, op1, svdup_n_s64(op2))
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_u8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urhadd))]
pub fn svrhadd_u8_m(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.urhadd.nxv16i8")]
        fn _svrhadd_u8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svrhadd_u8_m(pg, op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_n_u8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urhadd))]
pub fn svrhadd_n_u8_m(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svrhadd_u8_m(pg, op1, svdup_n_u8(op2))
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_u8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urhadd))]
pub fn svrhadd_u8_x(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svrhadd_u8_m(pg, op1, op2)
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_n_u8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urhadd))]
pub fn svrhadd_n_u8_x(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svrhadd_u8_x(pg, op1, svdup_n_u8(op2))
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_u8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urhadd))]
pub fn svrhadd_u8_z(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svrhadd_u8_m(pg, svsel_u8(pg, op1, svdup_n_u8(0)), op2)
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_n_u8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urhadd))]
pub fn svrhadd_n_u8_z(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svrhadd_u8_z(pg, op1, svdup_n_u8(op2))
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_u16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urhadd))]
pub fn svrhadd_u16_m(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.urhadd.nxv8i16")]
        fn _svrhadd_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svrhadd_u16_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_n_u16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urhadd))]
pub fn svrhadd_n_u16_m(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svrhadd_u16_m(pg, op1, svdup_n_u16(op2))
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_u16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urhadd))]
pub fn svrhadd_u16_x(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svrhadd_u16_m(pg, op1, op2)
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_n_u16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urhadd))]
pub fn svrhadd_n_u16_x(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svrhadd_u16_x(pg, op1, svdup_n_u16(op2))
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_u16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urhadd))]
pub fn svrhadd_u16_z(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svrhadd_u16_m(pg, svsel_u16(pg, op1, svdup_n_u16(0)), op2)
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_n_u16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urhadd))]
pub fn svrhadd_n_u16_z(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svrhadd_u16_z(pg, op1, svdup_n_u16(op2))
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urhadd))]
pub fn svrhadd_u32_m(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.urhadd.nxv4i32")]
        fn _svrhadd_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svrhadd_u32_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_n_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urhadd))]
pub fn svrhadd_n_u32_m(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svrhadd_u32_m(pg, op1, svdup_n_u32(op2))
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urhadd))]
pub fn svrhadd_u32_x(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svrhadd_u32_m(pg, op1, op2)
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_n_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urhadd))]
pub fn svrhadd_n_u32_x(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svrhadd_u32_x(pg, op1, svdup_n_u32(op2))
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_u32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urhadd))]
pub fn svrhadd_u32_z(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svrhadd_u32_m(pg, svsel_u32(pg, op1, svdup_n_u32(0)), op2)
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_n_u32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urhadd))]
pub fn svrhadd_n_u32_z(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svrhadd_u32_z(pg, op1, svdup_n_u32(op2))
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_u64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urhadd))]
pub fn svrhadd_u64_m(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.urhadd.nxv2i64")]
        fn _svrhadd_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svrhadd_u64_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_n_u64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urhadd))]
pub fn svrhadd_n_u64_m(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svrhadd_u64_m(pg, op1, svdup_n_u64(op2))
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_u64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urhadd))]
pub fn svrhadd_u64_x(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svrhadd_u64_m(pg, op1, op2)
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_n_u64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urhadd))]
pub fn svrhadd_n_u64_x(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svrhadd_u64_x(pg, op1, svdup_n_u64(op2))
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_u64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urhadd))]
pub fn svrhadd_u64_z(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svrhadd_u64_m(pg, svsel_u64(pg, op1, svdup_n_u64(0)), op2)
}
#[doc = "Rounding halving add"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrhadd[_n_u64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urhadd))]
pub fn svrhadd_n_u64_z(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svrhadd_u64_z(pg, op1, svdup_n_u64(op2))
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshl))]
pub fn svrshl_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.srshl.nxv16i8")]
        fn _svrshl_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svrshl_s8_m(pg, op1, op2) }
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_n_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshl))]
pub fn svrshl_n_s8_m(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svrshl_s8_m(pg, op1, svdup_n_s8(op2))
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshl))]
pub fn svrshl_s8_x(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svrshl_s8_m(pg, op1, op2)
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_n_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshl))]
pub fn svrshl_n_s8_x(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svrshl_s8_x(pg, op1, svdup_n_s8(op2))
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_s8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshl))]
pub fn svrshl_s8_z(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svrshl_s8_m(pg, svsel_s8(pg, op1, svdup_n_s8(0)), op2)
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_n_s8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshl))]
pub fn svrshl_n_s8_z(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svrshl_s8_z(pg, op1, svdup_n_s8(op2))
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshl))]
pub fn svrshl_s16_m(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.srshl.nxv8i16")]
        fn _svrshl_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svrshl_s16_m(pg.into(), op1, op2) }
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_n_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshl))]
pub fn svrshl_n_s16_m(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svrshl_s16_m(pg, op1, svdup_n_s16(op2))
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshl))]
pub fn svrshl_s16_x(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svrshl_s16_m(pg, op1, op2)
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_n_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshl))]
pub fn svrshl_n_s16_x(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svrshl_s16_x(pg, op1, svdup_n_s16(op2))
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_s16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshl))]
pub fn svrshl_s16_z(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svrshl_s16_m(pg, svsel_s16(pg, op1, svdup_n_s16(0)), op2)
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_n_s16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshl))]
pub fn svrshl_n_s16_z(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svrshl_s16_z(pg, op1, svdup_n_s16(op2))
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshl))]
pub fn svrshl_s32_m(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.srshl.nxv4i32")]
        fn _svrshl_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svrshl_s32_m(pg.into(), op1, op2) }
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_n_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshl))]
pub fn svrshl_n_s32_m(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svrshl_s32_m(pg, op1, svdup_n_s32(op2))
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshl))]
pub fn svrshl_s32_x(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svrshl_s32_m(pg, op1, op2)
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_n_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshl))]
pub fn svrshl_n_s32_x(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svrshl_s32_x(pg, op1, svdup_n_s32(op2))
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshl))]
pub fn svrshl_s32_z(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svrshl_s32_m(pg, svsel_s32(pg, op1, svdup_n_s32(0)), op2)
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_n_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshl))]
pub fn svrshl_n_s32_z(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svrshl_s32_z(pg, op1, svdup_n_s32(op2))
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshl))]
pub fn svrshl_s64_m(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.srshl.nxv2i64")]
        fn _svrshl_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svrshl_s64_m(pg.into(), op1, op2) }
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_n_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshl))]
pub fn svrshl_n_s64_m(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svrshl_s64_m(pg, op1, svdup_n_s64(op2))
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshl))]
pub fn svrshl_s64_x(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svrshl_s64_m(pg, op1, op2)
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_n_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshl))]
pub fn svrshl_n_s64_x(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svrshl_s64_x(pg, op1, svdup_n_s64(op2))
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshl))]
pub fn svrshl_s64_z(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svrshl_s64_m(pg, svsel_s64(pg, op1, svdup_n_s64(0)), op2)
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_n_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshl))]
pub fn svrshl_n_s64_z(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svrshl_s64_z(pg, op1, svdup_n_s64(op2))
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_u8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshl))]
pub fn svrshl_u8_m(pg: svbool_t, op1: svuint8_t, op2: svint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.urshl.nxv16i8")]
        fn _svrshl_u8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svrshl_u8_m(pg, op1.as_signed(), op2).as_unsigned() }
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_n_u8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshl))]
pub fn svrshl_n_u8_m(pg: svbool_t, op1: svuint8_t, op2: i8) -> svuint8_t {
    svrshl_u8_m(pg, op1, svdup_n_s8(op2))
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_u8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshl))]
pub fn svrshl_u8_x(pg: svbool_t, op1: svuint8_t, op2: svint8_t) -> svuint8_t {
    svrshl_u8_m(pg, op1, op2)
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_n_u8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshl))]
pub fn svrshl_n_u8_x(pg: svbool_t, op1: svuint8_t, op2: i8) -> svuint8_t {
    svrshl_u8_x(pg, op1, svdup_n_s8(op2))
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_u8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshl))]
pub fn svrshl_u8_z(pg: svbool_t, op1: svuint8_t, op2: svint8_t) -> svuint8_t {
    svrshl_u8_m(pg, svsel_u8(pg, op1, svdup_n_u8(0)), op2)
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_n_u8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshl))]
pub fn svrshl_n_u8_z(pg: svbool_t, op1: svuint8_t, op2: i8) -> svuint8_t {
    svrshl_u8_z(pg, op1, svdup_n_s8(op2))
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_u16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshl))]
pub fn svrshl_u16_m(pg: svbool_t, op1: svuint16_t, op2: svint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.urshl.nxv8i16")]
        fn _svrshl_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svrshl_u16_m(pg.into(), op1.as_signed(), op2).as_unsigned() }
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_n_u16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshl))]
pub fn svrshl_n_u16_m(pg: svbool_t, op1: svuint16_t, op2: i16) -> svuint16_t {
    svrshl_u16_m(pg, op1, svdup_n_s16(op2))
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_u16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshl))]
pub fn svrshl_u16_x(pg: svbool_t, op1: svuint16_t, op2: svint16_t) -> svuint16_t {
    svrshl_u16_m(pg, op1, op2)
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_n_u16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshl))]
pub fn svrshl_n_u16_x(pg: svbool_t, op1: svuint16_t, op2: i16) -> svuint16_t {
    svrshl_u16_x(pg, op1, svdup_n_s16(op2))
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_u16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshl))]
pub fn svrshl_u16_z(pg: svbool_t, op1: svuint16_t, op2: svint16_t) -> svuint16_t {
    svrshl_u16_m(pg, svsel_u16(pg, op1, svdup_n_u16(0)), op2)
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_n_u16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshl))]
pub fn svrshl_n_u16_z(pg: svbool_t, op1: svuint16_t, op2: i16) -> svuint16_t {
    svrshl_u16_z(pg, op1, svdup_n_s16(op2))
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshl))]
pub fn svrshl_u32_m(pg: svbool_t, op1: svuint32_t, op2: svint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.urshl.nxv4i32")]
        fn _svrshl_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svrshl_u32_m(pg.into(), op1.as_signed(), op2).as_unsigned() }
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_n_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshl))]
pub fn svrshl_n_u32_m(pg: svbool_t, op1: svuint32_t, op2: i32) -> svuint32_t {
    svrshl_u32_m(pg, op1, svdup_n_s32(op2))
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshl))]
pub fn svrshl_u32_x(pg: svbool_t, op1: svuint32_t, op2: svint32_t) -> svuint32_t {
    svrshl_u32_m(pg, op1, op2)
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_n_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshl))]
pub fn svrshl_n_u32_x(pg: svbool_t, op1: svuint32_t, op2: i32) -> svuint32_t {
    svrshl_u32_x(pg, op1, svdup_n_s32(op2))
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_u32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshl))]
pub fn svrshl_u32_z(pg: svbool_t, op1: svuint32_t, op2: svint32_t) -> svuint32_t {
    svrshl_u32_m(pg, svsel_u32(pg, op1, svdup_n_u32(0)), op2)
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_n_u32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshl))]
pub fn svrshl_n_u32_z(pg: svbool_t, op1: svuint32_t, op2: i32) -> svuint32_t {
    svrshl_u32_z(pg, op1, svdup_n_s32(op2))
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_u64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshl))]
pub fn svrshl_u64_m(pg: svbool_t, op1: svuint64_t, op2: svint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.urshl.nxv2i64")]
        fn _svrshl_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svrshl_u64_m(pg.into(), op1.as_signed(), op2).as_unsigned() }
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_n_u64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshl))]
pub fn svrshl_n_u64_m(pg: svbool_t, op1: svuint64_t, op2: i64) -> svuint64_t {
    svrshl_u64_m(pg, op1, svdup_n_s64(op2))
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_u64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshl))]
pub fn svrshl_u64_x(pg: svbool_t, op1: svuint64_t, op2: svint64_t) -> svuint64_t {
    svrshl_u64_m(pg, op1, op2)
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_n_u64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshl))]
pub fn svrshl_n_u64_x(pg: svbool_t, op1: svuint64_t, op2: i64) -> svuint64_t {
    svrshl_u64_x(pg, op1, svdup_n_s64(op2))
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_u64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshl))]
pub fn svrshl_u64_z(pg: svbool_t, op1: svuint64_t, op2: svint64_t) -> svuint64_t {
    svrshl_u64_m(pg, svsel_u64(pg, op1, svdup_n_u64(0)), op2)
}
#[doc = "Rounding shift left"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshl[_n_u64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshl))]
pub fn svrshl_n_u64_z(pg: svbool_t, op1: svuint64_t, op2: i64) -> svuint64_t {
    svrshl_u64_z(pg, op1, svdup_n_s64(op2))
}
#[doc = "Rounding shift right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshr[_n_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshr, IMM2 = 1))]
pub fn svrshr_n_s8_m<const IMM2: i32>(pg: svbool_t, op1: svint8_t) -> svint8_t {
    static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.srshr.nxv16i8")]
        fn _svrshr_n_s8_m(pg: svbool_t, op1: svint8_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svrshr_n_s8_m(pg, op1, IMM2) }
}
#[doc = "Rounding shift right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshr[_n_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshr, IMM2 = 1))]
pub fn svrshr_n_s8_x<const IMM2: i32>(pg: svbool_t, op1: svint8_t) -> svint8_t {
    svrshr_n_s8_m::<IMM2>(pg, op1)
}
#[doc = "Rounding shift right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshr[_n_s8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshr, IMM2 = 1))]
pub fn svrshr_n_s8_z<const IMM2: i32>(pg: svbool_t, op1: svint8_t) -> svint8_t {
    svrshr_n_s8_m::<IMM2>(pg, svsel_s8(pg, op1, svdup_n_s8(0)))
}
#[doc = "Rounding shift right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshr[_n_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshr, IMM2 = 1))]
pub fn svrshr_n_s16_m<const IMM2: i32>(pg: svbool_t, op1: svint16_t) -> svint16_t {
    static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.srshr.nxv8i16")]
        fn _svrshr_n_s16_m(pg: svbool8_t, op1: svint16_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svrshr_n_s16_m(pg.into(), op1, IMM2) }
}
#[doc = "Rounding shift right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshr[_n_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshr, IMM2 = 1))]
pub fn svrshr_n_s16_x<const IMM2: i32>(pg: svbool_t, op1: svint16_t) -> svint16_t {
    svrshr_n_s16_m::<IMM2>(pg, op1)
}
#[doc = "Rounding shift right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshr[_n_s16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshr, IMM2 = 1))]
pub fn svrshr_n_s16_z<const IMM2: i32>(pg: svbool_t, op1: svint16_t) -> svint16_t {
    svrshr_n_s16_m::<IMM2>(pg, svsel_s16(pg, op1, svdup_n_s16(0)))
}
#[doc = "Rounding shift right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshr[_n_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshr, IMM2 = 1))]
pub fn svrshr_n_s32_m<const IMM2: i32>(pg: svbool_t, op1: svint32_t) -> svint32_t {
    static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.srshr.nxv4i32")]
        fn _svrshr_n_s32_m(pg: svbool4_t, op1: svint32_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svrshr_n_s32_m(pg.into(), op1, IMM2) }
}
#[doc = "Rounding shift right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshr[_n_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshr, IMM2 = 1))]
pub fn svrshr_n_s32_x<const IMM2: i32>(pg: svbool_t, op1: svint32_t) -> svint32_t {
    svrshr_n_s32_m::<IMM2>(pg, op1)
}
#[doc = "Rounding shift right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshr[_n_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshr, IMM2 = 1))]
pub fn svrshr_n_s32_z<const IMM2: i32>(pg: svbool_t, op1: svint32_t) -> svint32_t {
    svrshr_n_s32_m::<IMM2>(pg, svsel_s32(pg, op1, svdup_n_s32(0)))
}
#[doc = "Rounding shift right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshr[_n_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshr, IMM2 = 1))]
pub fn svrshr_n_s64_m<const IMM2: i32>(pg: svbool_t, op1: svint64_t) -> svint64_t {
    static_assert_range!(IMM2, 1, 64);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.srshr.nxv2i64")]
        fn _svrshr_n_s64_m(pg: svbool2_t, op1: svint64_t, imm2: i32) -> svint64_t;
    }
    unsafe { _svrshr_n_s64_m(pg.into(), op1, IMM2) }
}
#[doc = "Rounding shift right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshr[_n_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshr, IMM2 = 1))]
pub fn svrshr_n_s64_x<const IMM2: i32>(pg: svbool_t, op1: svint64_t) -> svint64_t {
    svrshr_n_s64_m::<IMM2>(pg, op1)
}
#[doc = "Rounding shift right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshr[_n_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srshr, IMM2 = 1))]
pub fn svrshr_n_s64_z<const IMM2: i32>(pg: svbool_t, op1: svint64_t) -> svint64_t {
    svrshr_n_s64_m::<IMM2>(pg, svsel_s64(pg, op1, svdup_n_s64(0)))
}
#[doc = "Rounding shift right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshr[_n_u8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshr, IMM2 = 1))]
pub fn svrshr_n_u8_m<const IMM2: i32>(pg: svbool_t, op1: svuint8_t) -> svuint8_t {
    static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.urshr.nxv16i8")]
        fn _svrshr_n_u8_m(pg: svbool_t, op1: svint8_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svrshr_n_u8_m(pg, op1.as_signed(), IMM2).as_unsigned() }
}
#[doc = "Rounding shift right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshr[_n_u8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshr, IMM2 = 1))]
pub fn svrshr_n_u8_x<const IMM2: i32>(pg: svbool_t, op1: svuint8_t) -> svuint8_t {
    svrshr_n_u8_m::<IMM2>(pg, op1)
}
#[doc = "Rounding shift right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshr[_n_u8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshr, IMM2 = 1))]
pub fn svrshr_n_u8_z<const IMM2: i32>(pg: svbool_t, op1: svuint8_t) -> svuint8_t {
    svrshr_n_u8_m::<IMM2>(pg, svsel_u8(pg, op1, svdup_n_u8(0)))
}
#[doc = "Rounding shift right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshr[_n_u16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshr, IMM2 = 1))]
pub fn svrshr_n_u16_m<const IMM2: i32>(pg: svbool_t, op1: svuint16_t) -> svuint16_t {
    static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.urshr.nxv8i16")]
        fn _svrshr_n_u16_m(pg: svbool8_t, op1: svint16_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svrshr_n_u16_m(pg.into(), op1.as_signed(), IMM2).as_unsigned() }
}
#[doc = "Rounding shift right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshr[_n_u16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshr, IMM2 = 1))]
pub fn svrshr_n_u16_x<const IMM2: i32>(pg: svbool_t, op1: svuint16_t) -> svuint16_t {
    svrshr_n_u16_m::<IMM2>(pg, op1)
}
#[doc = "Rounding shift right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshr[_n_u16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshr, IMM2 = 1))]
pub fn svrshr_n_u16_z<const IMM2: i32>(pg: svbool_t, op1: svuint16_t) -> svuint16_t {
    svrshr_n_u16_m::<IMM2>(pg, svsel_u16(pg, op1, svdup_n_u16(0)))
}
#[doc = "Rounding shift right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshr[_n_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshr, IMM2 = 1))]
pub fn svrshr_n_u32_m<const IMM2: i32>(pg: svbool_t, op1: svuint32_t) -> svuint32_t {
    static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.urshr.nxv4i32")]
        fn _svrshr_n_u32_m(pg: svbool4_t, op1: svint32_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svrshr_n_u32_m(pg.into(), op1.as_signed(), IMM2).as_unsigned() }
}
#[doc = "Rounding shift right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshr[_n_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshr, IMM2 = 1))]
pub fn svrshr_n_u32_x<const IMM2: i32>(pg: svbool_t, op1: svuint32_t) -> svuint32_t {
    svrshr_n_u32_m::<IMM2>(pg, op1)
}
#[doc = "Rounding shift right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshr[_n_u32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshr, IMM2 = 1))]
pub fn svrshr_n_u32_z<const IMM2: i32>(pg: svbool_t, op1: svuint32_t) -> svuint32_t {
    svrshr_n_u32_m::<IMM2>(pg, svsel_u32(pg, op1, svdup_n_u32(0)))
}
#[doc = "Rounding shift right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshr[_n_u64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshr, IMM2 = 1))]
pub fn svrshr_n_u64_m<const IMM2: i32>(pg: svbool_t, op1: svuint64_t) -> svuint64_t {
    static_assert_range!(IMM2, 1, 64);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.urshr.nxv2i64")]
        fn _svrshr_n_u64_m(pg: svbool2_t, op1: svint64_t, imm2: i32) -> svint64_t;
    }
    unsafe { _svrshr_n_u64_m(pg.into(), op1.as_signed(), IMM2).as_unsigned() }
}
#[doc = "Rounding shift right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshr[_n_u64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshr, IMM2 = 1))]
pub fn svrshr_n_u64_x<const IMM2: i32>(pg: svbool_t, op1: svuint64_t) -> svuint64_t {
    svrshr_n_u64_m::<IMM2>(pg, op1)
}
#[doc = "Rounding shift right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshr[_n_u64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(urshr, IMM2 = 1))]
pub fn svrshr_n_u64_z<const IMM2: i32>(pg: svbool_t, op1: svuint64_t) -> svuint64_t {
    svrshr_n_u64_m::<IMM2>(pg, svsel_u64(pg, op1, svdup_n_u64(0)))
}
#[doc = "Rounding shift right narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshrnb[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rshrnb, IMM2 = 1))]
pub fn svrshrnb_n_s16<const IMM2: i32>(op1: svint16_t) -> svint8_t {
    static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.rshrnb.nxv8i16")]
        fn _svrshrnb_n_s16(op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svrshrnb_n_s16(op1, IMM2) }
}
#[doc = "Rounding shift right narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshrnb[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rshrnb, IMM2 = 1))]
pub fn svrshrnb_n_s32<const IMM2: i32>(op1: svint32_t) -> svint16_t {
    static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.rshrnb.nxv4i32")]
        fn _svrshrnb_n_s32(op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svrshrnb_n_s32(op1, IMM2) }
}
#[doc = "Rounding shift right narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshrnb[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rshrnb, IMM2 = 1))]
pub fn svrshrnb_n_s64<const IMM2: i32>(op1: svint64_t) -> svint32_t {
    static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.rshrnb.nxv2i64")]
        fn _svrshrnb_n_s64(op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svrshrnb_n_s64(op1, IMM2) }
}
#[doc = "Rounding shift right narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshrnb[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rshrnb, IMM2 = 1))]
pub fn svrshrnb_n_u16<const IMM2: i32>(op1: svuint16_t) -> svuint8_t {
    static_assert_range!(IMM2, 1, 8);
    unsafe { svrshrnb_n_s16::<IMM2>(op1.as_signed()).as_unsigned() }
}
#[doc = "Rounding shift right narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshrnb[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rshrnb, IMM2 = 1))]
pub fn svrshrnb_n_u32<const IMM2: i32>(op1: svuint32_t) -> svuint16_t {
    static_assert_range!(IMM2, 1, 16);
    unsafe { svrshrnb_n_s32::<IMM2>(op1.as_signed()).as_unsigned() }
}
#[doc = "Rounding shift right narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshrnb[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rshrnb, IMM2 = 1))]
pub fn svrshrnb_n_u64<const IMM2: i32>(op1: svuint64_t) -> svuint32_t {
    static_assert_range!(IMM2, 1, 32);
    unsafe { svrshrnb_n_s64::<IMM2>(op1.as_signed()).as_unsigned() }
}
#[doc = "Rounding shift right narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshrnt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rshrnt, IMM2 = 1))]
pub fn svrshrnt_n_s16<const IMM2: i32>(even: svint8_t, op1: svint16_t) -> svint8_t {
    static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.rshrnt.nxv8i16")]
        fn _svrshrnt_n_s16(even: svint8_t, op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svrshrnt_n_s16(even, op1, IMM2) }
}
#[doc = "Rounding shift right narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshrnt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rshrnt, IMM2 = 1))]
pub fn svrshrnt_n_s32<const IMM2: i32>(even: svint16_t, op1: svint32_t) -> svint16_t {
    static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.rshrnt.nxv4i32")]
        fn _svrshrnt_n_s32(even: svint16_t, op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svrshrnt_n_s32(even, op1, IMM2) }
}
#[doc = "Rounding shift right narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshrnt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rshrnt, IMM2 = 1))]
pub fn svrshrnt_n_s64<const IMM2: i32>(even: svint32_t, op1: svint64_t) -> svint32_t {
    static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.rshrnt.nxv2i64")]
        fn _svrshrnt_n_s64(even: svint32_t, op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svrshrnt_n_s64(even, op1, IMM2) }
}
#[doc = "Rounding shift right narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshrnt[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rshrnt, IMM2 = 1))]
pub fn svrshrnt_n_u16<const IMM2: i32>(even: svuint8_t, op1: svuint16_t) -> svuint8_t {
    static_assert_range!(IMM2, 1, 8);
    unsafe { svrshrnt_n_s16::<IMM2>(even.as_signed(), op1.as_signed()).as_unsigned() }
}
#[doc = "Rounding shift right narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshrnt[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rshrnt, IMM2 = 1))]
pub fn svrshrnt_n_u32<const IMM2: i32>(even: svuint16_t, op1: svuint32_t) -> svuint16_t {
    static_assert_range!(IMM2, 1, 16);
    unsafe { svrshrnt_n_s32::<IMM2>(even.as_signed(), op1.as_signed()).as_unsigned() }
}
#[doc = "Rounding shift right narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrshrnt[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rshrnt, IMM2 = 1))]
pub fn svrshrnt_n_u64<const IMM2: i32>(even: svuint32_t, op1: svuint64_t) -> svuint32_t {
    static_assert_range!(IMM2, 1, 32);
    unsafe { svrshrnt_n_s64::<IMM2>(even.as_signed(), op1.as_signed()).as_unsigned() }
}
#[doc = "Reciprocal square root estimate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsqrte[_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ursqrte))]
pub fn svrsqrte_u32_m(inactive: svuint32_t, pg: svbool_t, op: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ursqrte.nxv4i32"
        )]
        fn _svrsqrte_u32_m(inactive: svint32_t, pg: svbool4_t, op: svint32_t) -> svint32_t;
    }
    unsafe { _svrsqrte_u32_m(inactive.as_signed(), pg.into(), op.as_signed()).as_unsigned() }
}
#[doc = "Reciprocal square root estimate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsqrte[_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ursqrte))]
pub fn svrsqrte_u32_x(pg: svbool_t, op: svuint32_t) -> svuint32_t {
    svrsqrte_u32_m(op, pg, op)
}
#[doc = "Reciprocal square root estimate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsqrte[_u32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ursqrte))]
pub fn svrsqrte_u32_z(pg: svbool_t, op: svuint32_t) -> svuint32_t {
    svrsqrte_u32_m(svdup_n_u32(0), pg, op)
}
#[doc = "Rounding shift right and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsra[_n_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srsra, IMM3 = 1))]
pub fn svrsra_n_s8<const IMM3: i32>(op1: svint8_t, op2: svint8_t) -> svint8_t {
    static_assert_range!(IMM3, 1, 8);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.srsra.nxv16i8")]
        fn _svrsra_n_s8(op1: svint8_t, op2: svint8_t, imm3: i32) -> svint8_t;
    }
    unsafe { _svrsra_n_s8(op1, op2, IMM3) }
}
#[doc = "Rounding shift right and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsra[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srsra, IMM3 = 1))]
pub fn svrsra_n_s16<const IMM3: i32>(op1: svint16_t, op2: svint16_t) -> svint16_t {
    static_assert_range!(IMM3, 1, 16);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.srsra.nxv8i16")]
        fn _svrsra_n_s16(op1: svint16_t, op2: svint16_t, imm3: i32) -> svint16_t;
    }
    unsafe { _svrsra_n_s16(op1, op2, IMM3) }
}
#[doc = "Rounding shift right and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsra[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srsra, IMM3 = 1))]
pub fn svrsra_n_s32<const IMM3: i32>(op1: svint32_t, op2: svint32_t) -> svint32_t {
    static_assert_range!(IMM3, 1, 32);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.srsra.nxv4i32")]
        fn _svrsra_n_s32(op1: svint32_t, op2: svint32_t, imm3: i32) -> svint32_t;
    }
    unsafe { _svrsra_n_s32(op1, op2, IMM3) }
}
#[doc = "Rounding shift right and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsra[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(srsra, IMM3 = 1))]
pub fn svrsra_n_s64<const IMM3: i32>(op1: svint64_t, op2: svint64_t) -> svint64_t {
    static_assert_range!(IMM3, 1, 64);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.srsra.nxv2i64")]
        fn _svrsra_n_s64(op1: svint64_t, op2: svint64_t, imm3: i32) -> svint64_t;
    }
    unsafe { _svrsra_n_s64(op1, op2, IMM3) }
}
#[doc = "Rounding shift right and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsra[_n_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ursra, IMM3 = 1))]
pub fn svrsra_n_u8<const IMM3: i32>(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    static_assert_range!(IMM3, 1, 8);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.ursra.nxv16i8")]
        fn _svrsra_n_u8(op1: svint8_t, op2: svint8_t, imm3: i32) -> svint8_t;
    }
    unsafe { _svrsra_n_u8(op1.as_signed(), op2.as_signed(), IMM3).as_unsigned() }
}
#[doc = "Rounding shift right and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsra[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ursra, IMM3 = 1))]
pub fn svrsra_n_u16<const IMM3: i32>(op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    static_assert_range!(IMM3, 1, 16);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.ursra.nxv8i16")]
        fn _svrsra_n_u16(op1: svint16_t, op2: svint16_t, imm3: i32) -> svint16_t;
    }
    unsafe { _svrsra_n_u16(op1.as_signed(), op2.as_signed(), IMM3).as_unsigned() }
}
#[doc = "Rounding shift right and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsra[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ursra, IMM3 = 1))]
pub fn svrsra_n_u32<const IMM3: i32>(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    static_assert_range!(IMM3, 1, 32);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.ursra.nxv4i32")]
        fn _svrsra_n_u32(op1: svint32_t, op2: svint32_t, imm3: i32) -> svint32_t;
    }
    unsafe { _svrsra_n_u32(op1.as_signed(), op2.as_signed(), IMM3).as_unsigned() }
}
#[doc = "Rounding shift right and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsra[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ursra, IMM3 = 1))]
pub fn svrsra_n_u64<const IMM3: i32>(op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    static_assert_range!(IMM3, 1, 64);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.ursra.nxv2i64")]
        fn _svrsra_n_u64(op1: svint64_t, op2: svint64_t, imm3: i32) -> svint64_t;
    }
    unsafe { _svrsra_n_u64(op1.as_signed(), op2.as_signed(), IMM3).as_unsigned() }
}
#[doc = "Rounding subtract narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsubhnb[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rsubhnb))]
pub fn svrsubhnb_s16(op1: svint16_t, op2: svint16_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.rsubhnb.nxv8i16"
        )]
        fn _svrsubhnb_s16(op1: svint16_t, op2: svint16_t) -> svint8_t;
    }
    unsafe { _svrsubhnb_s16(op1, op2) }
}
#[doc = "Rounding subtract narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsubhnb[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rsubhnb))]
pub fn svrsubhnb_n_s16(op1: svint16_t, op2: i16) -> svint8_t {
    svrsubhnb_s16(op1, svdup_n_s16(op2))
}
#[doc = "Rounding subtract narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsubhnb[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rsubhnb))]
pub fn svrsubhnb_s32(op1: svint32_t, op2: svint32_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.rsubhnb.nxv4i32"
        )]
        fn _svrsubhnb_s32(op1: svint32_t, op2: svint32_t) -> svint16_t;
    }
    unsafe { _svrsubhnb_s32(op1, op2) }
}
#[doc = "Rounding subtract narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsubhnb[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rsubhnb))]
pub fn svrsubhnb_n_s32(op1: svint32_t, op2: i32) -> svint16_t {
    svrsubhnb_s32(op1, svdup_n_s32(op2))
}
#[doc = "Rounding subtract narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsubhnb[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rsubhnb))]
pub fn svrsubhnb_s64(op1: svint64_t, op2: svint64_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.rsubhnb.nxv2i64"
        )]
        fn _svrsubhnb_s64(op1: svint64_t, op2: svint64_t) -> svint32_t;
    }
    unsafe { _svrsubhnb_s64(op1, op2) }
}
#[doc = "Rounding subtract narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsubhnb[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rsubhnb))]
pub fn svrsubhnb_n_s64(op1: svint64_t, op2: i64) -> svint32_t {
    svrsubhnb_s64(op1, svdup_n_s64(op2))
}
#[doc = "Rounding subtract narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsubhnb[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rsubhnb))]
pub fn svrsubhnb_u16(op1: svuint16_t, op2: svuint16_t) -> svuint8_t {
    unsafe { svrsubhnb_s16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Rounding subtract narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsubhnb[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rsubhnb))]
pub fn svrsubhnb_n_u16(op1: svuint16_t, op2: u16) -> svuint8_t {
    svrsubhnb_u16(op1, svdup_n_u16(op2))
}
#[doc = "Rounding subtract narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsubhnb[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rsubhnb))]
pub fn svrsubhnb_u32(op1: svuint32_t, op2: svuint32_t) -> svuint16_t {
    unsafe { svrsubhnb_s32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Rounding subtract narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsubhnb[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rsubhnb))]
pub fn svrsubhnb_n_u32(op1: svuint32_t, op2: u32) -> svuint16_t {
    svrsubhnb_u32(op1, svdup_n_u32(op2))
}
#[doc = "Rounding subtract narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsubhnb[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rsubhnb))]
pub fn svrsubhnb_u64(op1: svuint64_t, op2: svuint64_t) -> svuint32_t {
    unsafe { svrsubhnb_s64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Rounding subtract narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsubhnb[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rsubhnb))]
pub fn svrsubhnb_n_u64(op1: svuint64_t, op2: u64) -> svuint32_t {
    svrsubhnb_u64(op1, svdup_n_u64(op2))
}
#[doc = "Rounding subtract narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsubhnt[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rsubhnt))]
pub fn svrsubhnt_s16(even: svint8_t, op1: svint16_t, op2: svint16_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.rsubhnt.nxv8i16"
        )]
        fn _svrsubhnt_s16(even: svint8_t, op1: svint16_t, op2: svint16_t) -> svint8_t;
    }
    unsafe { _svrsubhnt_s16(even, op1, op2) }
}
#[doc = "Rounding subtract narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsubhnt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rsubhnt))]
pub fn svrsubhnt_n_s16(even: svint8_t, op1: svint16_t, op2: i16) -> svint8_t {
    svrsubhnt_s16(even, op1, svdup_n_s16(op2))
}
#[doc = "Rounding subtract narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsubhnt[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rsubhnt))]
pub fn svrsubhnt_s32(even: svint16_t, op1: svint32_t, op2: svint32_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.rsubhnt.nxv4i32"
        )]
        fn _svrsubhnt_s32(even: svint16_t, op1: svint32_t, op2: svint32_t) -> svint16_t;
    }
    unsafe { _svrsubhnt_s32(even, op1, op2) }
}
#[doc = "Rounding subtract narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsubhnt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rsubhnt))]
pub fn svrsubhnt_n_s32(even: svint16_t, op1: svint32_t, op2: i32) -> svint16_t {
    svrsubhnt_s32(even, op1, svdup_n_s32(op2))
}
#[doc = "Rounding subtract narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsubhnt[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rsubhnt))]
pub fn svrsubhnt_s64(even: svint32_t, op1: svint64_t, op2: svint64_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.rsubhnt.nxv2i64"
        )]
        fn _svrsubhnt_s64(even: svint32_t, op1: svint64_t, op2: svint64_t) -> svint32_t;
    }
    unsafe { _svrsubhnt_s64(even, op1, op2) }
}
#[doc = "Rounding subtract narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsubhnt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rsubhnt))]
pub fn svrsubhnt_n_s64(even: svint32_t, op1: svint64_t, op2: i64) -> svint32_t {
    svrsubhnt_s64(even, op1, svdup_n_s64(op2))
}
#[doc = "Rounding subtract narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsubhnt[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rsubhnt))]
pub fn svrsubhnt_u16(even: svuint8_t, op1: svuint16_t, op2: svuint16_t) -> svuint8_t {
    unsafe { svrsubhnt_s16(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Rounding subtract narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsubhnt[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rsubhnt))]
pub fn svrsubhnt_n_u16(even: svuint8_t, op1: svuint16_t, op2: u16) -> svuint8_t {
    svrsubhnt_u16(even, op1, svdup_n_u16(op2))
}
#[doc = "Rounding subtract narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsubhnt[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rsubhnt))]
pub fn svrsubhnt_u32(even: svuint16_t, op1: svuint32_t, op2: svuint32_t) -> svuint16_t {
    unsafe { svrsubhnt_s32(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Rounding subtract narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsubhnt[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rsubhnt))]
pub fn svrsubhnt_n_u32(even: svuint16_t, op1: svuint32_t, op2: u32) -> svuint16_t {
    svrsubhnt_u32(even, op1, svdup_n_u32(op2))
}
#[doc = "Rounding subtract narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsubhnt[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rsubhnt))]
pub fn svrsubhnt_u64(even: svuint32_t, op1: svuint64_t, op2: svuint64_t) -> svuint32_t {
    unsafe { svrsubhnt_s64(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Rounding subtract narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svrsubhnt[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(rsubhnt))]
pub fn svrsubhnt_n_u64(even: svuint32_t, op1: svuint64_t, op2: u64) -> svuint32_t {
    svrsubhnt_u64(even, op1, svdup_n_u64(op2))
}
#[doc = "Subtract with borrow long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsbclb[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sbclb))]
pub fn svsbclb_u32(op1: svuint32_t, op2: svuint32_t, op3: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sbclb.nxv4i32")]
        fn _svsbclb_u32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _svsbclb_u32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Subtract with borrow long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsbclb[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sbclb))]
pub fn svsbclb_n_u32(op1: svuint32_t, op2: svuint32_t, op3: u32) -> svuint32_t {
    svsbclb_u32(op1, op2, svdup_n_u32(op3))
}
#[doc = "Subtract with borrow long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsbclb[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sbclb))]
pub fn svsbclb_u64(op1: svuint64_t, op2: svuint64_t, op3: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sbclb.nxv2i64")]
        fn _svsbclb_u64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _svsbclb_u64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Subtract with borrow long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsbclb[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sbclb))]
pub fn svsbclb_n_u64(op1: svuint64_t, op2: svuint64_t, op3: u64) -> svuint64_t {
    svsbclb_u64(op1, op2, svdup_n_u64(op3))
}
#[doc = "Subtract with borrow long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsbclt[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sbclt))]
pub fn svsbclt_u32(op1: svuint32_t, op2: svuint32_t, op3: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sbclt.nxv4i32")]
        fn _svsbclt_u32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _svsbclt_u32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Subtract with borrow long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsbclt[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sbclt))]
pub fn svsbclt_n_u32(op1: svuint32_t, op2: svuint32_t, op3: u32) -> svuint32_t {
    svsbclt_u32(op1, op2, svdup_n_u32(op3))
}
#[doc = "Subtract with borrow long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsbclt[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sbclt))]
pub fn svsbclt_u64(op1: svuint64_t, op2: svuint64_t, op3: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sbclt.nxv2i64")]
        fn _svsbclt_u64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _svsbclt_u64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[doc = "Subtract with borrow long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsbclt[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sbclt))]
pub fn svsbclt_n_u64(op1: svuint64_t, op2: svuint64_t, op3: u64) -> svuint64_t {
    svsbclt_u64(op1, op2, svdup_n_u64(op3))
}
#[doc = "Shift left long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svshllb[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sshllb, IMM2 = 0))]
pub fn svshllb_n_s16<const IMM2: i32>(op1: svint8_t) -> svint16_t {
    static_assert_range!(IMM2, 0, 7);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sshllb.nxv8i16")]
        fn _svshllb_n_s16(op1: svint8_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svshllb_n_s16(op1, IMM2) }
}
#[doc = "Shift left long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svshllb[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sshllb, IMM2 = 0))]
pub fn svshllb_n_s32<const IMM2: i32>(op1: svint16_t) -> svint32_t {
    static_assert_range!(IMM2, 0, 15);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sshllb.nxv4i32")]
        fn _svshllb_n_s32(op1: svint16_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svshllb_n_s32(op1, IMM2) }
}
#[doc = "Shift left long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svshllb[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sshllb, IMM2 = 0))]
pub fn svshllb_n_s64<const IMM2: i32>(op1: svint32_t) -> svint64_t {
    static_assert_range!(IMM2, 0, 31);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sshllb.nxv2i64")]
        fn _svshllb_n_s64(op1: svint32_t, imm2: i32) -> svint64_t;
    }
    unsafe { _svshllb_n_s64(op1, IMM2) }
}
#[doc = "Shift left long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svshllb[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ushllb, IMM2 = 0))]
pub fn svshllb_n_u16<const IMM2: i32>(op1: svuint8_t) -> svuint16_t {
    static_assert_range!(IMM2, 0, 7);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.ushllb.nxv8i16")]
        fn _svshllb_n_u16(op1: svint8_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svshllb_n_u16(op1.as_signed(), IMM2).as_unsigned() }
}
#[doc = "Shift left long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svshllb[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ushllb, IMM2 = 0))]
pub fn svshllb_n_u32<const IMM2: i32>(op1: svuint16_t) -> svuint32_t {
    static_assert_range!(IMM2, 0, 15);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.ushllb.nxv4i32")]
        fn _svshllb_n_u32(op1: svint16_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svshllb_n_u32(op1.as_signed(), IMM2).as_unsigned() }
}
#[doc = "Shift left long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svshllb[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ushllb, IMM2 = 0))]
pub fn svshllb_n_u64<const IMM2: i32>(op1: svuint32_t) -> svuint64_t {
    static_assert_range!(IMM2, 0, 31);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.ushllb.nxv2i64")]
        fn _svshllb_n_u64(op1: svint32_t, imm2: i32) -> svint64_t;
    }
    unsafe { _svshllb_n_u64(op1.as_signed(), IMM2).as_unsigned() }
}
#[doc = "Shift left long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svshllt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sshllt, IMM2 = 0))]
pub fn svshllt_n_s16<const IMM2: i32>(op1: svint8_t) -> svint16_t {
    static_assert_range!(IMM2, 0, 7);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sshllt.nxv8i16")]
        fn _svshllt_n_s16(op1: svint8_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svshllt_n_s16(op1, IMM2) }
}
#[doc = "Shift left long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svshllt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sshllt, IMM2 = 0))]
pub fn svshllt_n_s32<const IMM2: i32>(op1: svint16_t) -> svint32_t {
    static_assert_range!(IMM2, 0, 15);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sshllt.nxv4i32")]
        fn _svshllt_n_s32(op1: svint16_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svshllt_n_s32(op1, IMM2) }
}
#[doc = "Shift left long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svshllt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sshllt, IMM2 = 0))]
pub fn svshllt_n_s64<const IMM2: i32>(op1: svint32_t) -> svint64_t {
    static_assert_range!(IMM2, 0, 31);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sshllt.nxv2i64")]
        fn _svshllt_n_s64(op1: svint32_t, imm2: i32) -> svint64_t;
    }
    unsafe { _svshllt_n_s64(op1, IMM2) }
}
#[doc = "Shift left long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svshllt[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ushllt, IMM2 = 0))]
pub fn svshllt_n_u16<const IMM2: i32>(op1: svuint8_t) -> svuint16_t {
    static_assert_range!(IMM2, 0, 7);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.ushllt.nxv8i16")]
        fn _svshllt_n_u16(op1: svint8_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svshllt_n_u16(op1.as_signed(), IMM2).as_unsigned() }
}
#[doc = "Shift left long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svshllt[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ushllt, IMM2 = 0))]
pub fn svshllt_n_u32<const IMM2: i32>(op1: svuint16_t) -> svuint32_t {
    static_assert_range!(IMM2, 0, 15);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.ushllt.nxv4i32")]
        fn _svshllt_n_u32(op1: svint16_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svshllt_n_u32(op1.as_signed(), IMM2).as_unsigned() }
}
#[doc = "Shift left long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svshllt[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ushllt, IMM2 = 0))]
pub fn svshllt_n_u64<const IMM2: i32>(op1: svuint32_t) -> svuint64_t {
    static_assert_range!(IMM2, 0, 31);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.ushllt.nxv2i64")]
        fn _svshllt_n_u64(op1: svint32_t, imm2: i32) -> svint64_t;
    }
    unsafe { _svshllt_n_u64(op1.as_signed(), IMM2).as_unsigned() }
}
#[doc = "Shift right narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svshrnb[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shrnb, IMM2 = 1))]
pub fn svshrnb_n_s16<const IMM2: i32>(op1: svint16_t) -> svint8_t {
    static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.shrnb.nxv8i16")]
        fn _svshrnb_n_s16(op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svshrnb_n_s16(op1, IMM2) }
}
#[doc = "Shift right narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svshrnb[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shrnb, IMM2 = 1))]
pub fn svshrnb_n_s32<const IMM2: i32>(op1: svint32_t) -> svint16_t {
    static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.shrnb.nxv4i32")]
        fn _svshrnb_n_s32(op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svshrnb_n_s32(op1, IMM2) }
}
#[doc = "Shift right narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svshrnb[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shrnb, IMM2 = 1))]
pub fn svshrnb_n_s64<const IMM2: i32>(op1: svint64_t) -> svint32_t {
    static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.shrnb.nxv2i64")]
        fn _svshrnb_n_s64(op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svshrnb_n_s64(op1, IMM2) }
}
#[doc = "Shift right narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svshrnb[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shrnb, IMM2 = 1))]
pub fn svshrnb_n_u16<const IMM2: i32>(op1: svuint16_t) -> svuint8_t {
    static_assert_range!(IMM2, 1, 8);
    unsafe { svshrnb_n_s16::<IMM2>(op1.as_signed()).as_unsigned() }
}
#[doc = "Shift right narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svshrnb[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shrnb, IMM2 = 1))]
pub fn svshrnb_n_u32<const IMM2: i32>(op1: svuint32_t) -> svuint16_t {
    static_assert_range!(IMM2, 1, 16);
    unsafe { svshrnb_n_s32::<IMM2>(op1.as_signed()).as_unsigned() }
}
#[doc = "Shift right narrow (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svshrnb[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shrnb, IMM2 = 1))]
pub fn svshrnb_n_u64<const IMM2: i32>(op1: svuint64_t) -> svuint32_t {
    static_assert_range!(IMM2, 1, 32);
    unsafe { svshrnb_n_s64::<IMM2>(op1.as_signed()).as_unsigned() }
}
#[doc = "Shift right narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svshrnt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shrnt, IMM2 = 1))]
pub fn svshrnt_n_s16<const IMM2: i32>(even: svint8_t, op1: svint16_t) -> svint8_t {
    static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.shrnt.nxv8i16")]
        fn _svshrnt_n_s16(even: svint8_t, op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svshrnt_n_s16(even, op1, IMM2) }
}
#[doc = "Shift right narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svshrnt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shrnt, IMM2 = 1))]
pub fn svshrnt_n_s32<const IMM2: i32>(even: svint16_t, op1: svint32_t) -> svint16_t {
    static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.shrnt.nxv4i32")]
        fn _svshrnt_n_s32(even: svint16_t, op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svshrnt_n_s32(even, op1, IMM2) }
}
#[doc = "Shift right narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svshrnt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shrnt, IMM2 = 1))]
pub fn svshrnt_n_s64<const IMM2: i32>(even: svint32_t, op1: svint64_t) -> svint32_t {
    static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.shrnt.nxv2i64")]
        fn _svshrnt_n_s64(even: svint32_t, op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svshrnt_n_s64(even, op1, IMM2) }
}
#[doc = "Shift right narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svshrnt[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shrnt, IMM2 = 1))]
pub fn svshrnt_n_u16<const IMM2: i32>(even: svuint8_t, op1: svuint16_t) -> svuint8_t {
    static_assert_range!(IMM2, 1, 8);
    unsafe { svshrnt_n_s16::<IMM2>(even.as_signed(), op1.as_signed()).as_unsigned() }
}
#[doc = "Shift right narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svshrnt[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shrnt, IMM2 = 1))]
pub fn svshrnt_n_u32<const IMM2: i32>(even: svuint16_t, op1: svuint32_t) -> svuint16_t {
    static_assert_range!(IMM2, 1, 16);
    unsafe { svshrnt_n_s32::<IMM2>(even.as_signed(), op1.as_signed()).as_unsigned() }
}
#[doc = "Shift right narrow (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svshrnt[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(shrnt, IMM2 = 1))]
pub fn svshrnt_n_u64<const IMM2: i32>(even: svuint32_t, op1: svuint64_t) -> svuint32_t {
    static_assert_range!(IMM2, 1, 32);
    unsafe { svshrnt_n_s64::<IMM2>(even.as_signed(), op1.as_signed()).as_unsigned() }
}
#[doc = "Shift left and insert"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsli[_n_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sli, IMM3 = 0))]
pub fn svsli_n_s8<const IMM3: i32>(op1: svint8_t, op2: svint8_t) -> svint8_t {
    static_assert_range!(IMM3, 0, 7);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sli.nxv16i8")]
        fn _svsli_n_s8(op1: svint8_t, op2: svint8_t, imm3: i32) -> svint8_t;
    }
    unsafe { _svsli_n_s8(op1, op2, IMM3) }
}
#[doc = "Shift left and insert"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsli[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sli, IMM3 = 0))]
pub fn svsli_n_s16<const IMM3: i32>(op1: svint16_t, op2: svint16_t) -> svint16_t {
    static_assert_range!(IMM3, 0, 15);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sli.nxv8i16")]
        fn _svsli_n_s16(op1: svint16_t, op2: svint16_t, imm3: i32) -> svint16_t;
    }
    unsafe { _svsli_n_s16(op1, op2, IMM3) }
}
#[doc = "Shift left and insert"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsli[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sli, IMM3 = 0))]
pub fn svsli_n_s32<const IMM3: i32>(op1: svint32_t, op2: svint32_t) -> svint32_t {
    static_assert_range!(IMM3, 0, 31);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sli.nxv4i32")]
        fn _svsli_n_s32(op1: svint32_t, op2: svint32_t, imm3: i32) -> svint32_t;
    }
    unsafe { _svsli_n_s32(op1, op2, IMM3) }
}
#[doc = "Shift left and insert"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsli[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sli, IMM3 = 0))]
pub fn svsli_n_s64<const IMM3: i32>(op1: svint64_t, op2: svint64_t) -> svint64_t {
    static_assert_range!(IMM3, 0, 63);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sli.nxv2i64")]
        fn _svsli_n_s64(op1: svint64_t, op2: svint64_t, imm3: i32) -> svint64_t;
    }
    unsafe { _svsli_n_s64(op1, op2, IMM3) }
}
#[doc = "Shift left and insert"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsli[_n_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sli, IMM3 = 0))]
pub fn svsli_n_u8<const IMM3: i32>(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    static_assert_range!(IMM3, 0, 7);
    unsafe { svsli_n_s8::<IMM3>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Shift left and insert"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsli[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sli, IMM3 = 0))]
pub fn svsli_n_u16<const IMM3: i32>(op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    static_assert_range!(IMM3, 0, 15);
    unsafe { svsli_n_s16::<IMM3>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Shift left and insert"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsli[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sli, IMM3 = 0))]
pub fn svsli_n_u32<const IMM3: i32>(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    static_assert_range!(IMM3, 0, 31);
    unsafe { svsli_n_s32::<IMM3>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Shift left and insert"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsli[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sli, IMM3 = 0))]
pub fn svsli_n_u64<const IMM3: i32>(op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    static_assert_range!(IMM3, 0, 63);
    unsafe { svsli_n_s64::<IMM3>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "SM4 encryption and decryption"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsm4e[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-sm4")]
#[cfg_attr(test, assert_instr(sm4e))]
pub fn svsm4e_u32(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sm4e")]
        fn _svsm4e_u32(op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svsm4e_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "SM4 key updates"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsm4ekey[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2,sve2-sm4")]
#[cfg_attr(test, assert_instr(sm4ekey))]
pub fn svsm4ekey_u32(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sm4ekey")]
        fn _svsm4ekey_u32(op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svsm4ekey_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Saturating add with signed addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsqadd[_u8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usqadd))]
pub fn svsqadd_u8_m(pg: svbool_t, op1: svuint8_t, op2: svint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.usqadd.nxv16i8")]
        fn _svsqadd_u8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svsqadd_u8_m(pg, op1.as_signed(), op2).as_unsigned() }
}
#[doc = "Saturating add with signed addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsqadd[_n_u8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usqadd))]
pub fn svsqadd_n_u8_m(pg: svbool_t, op1: svuint8_t, op2: i8) -> svuint8_t {
    svsqadd_u8_m(pg, op1, svdup_n_s8(op2))
}
#[doc = "Saturating add with signed addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsqadd[_u8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usqadd))]
pub fn svsqadd_u8_x(pg: svbool_t, op1: svuint8_t, op2: svint8_t) -> svuint8_t {
    svsqadd_u8_m(pg, op1, op2)
}
#[doc = "Saturating add with signed addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsqadd[_n_u8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usqadd))]
pub fn svsqadd_n_u8_x(pg: svbool_t, op1: svuint8_t, op2: i8) -> svuint8_t {
    svsqadd_u8_x(pg, op1, svdup_n_s8(op2))
}
#[doc = "Saturating add with signed addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsqadd[_u8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usqadd))]
pub fn svsqadd_u8_z(pg: svbool_t, op1: svuint8_t, op2: svint8_t) -> svuint8_t {
    svsqadd_u8_m(pg, svsel_u8(pg, op1, svdup_n_u8(0)), op2)
}
#[doc = "Saturating add with signed addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsqadd[_n_u8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usqadd))]
pub fn svsqadd_n_u8_z(pg: svbool_t, op1: svuint8_t, op2: i8) -> svuint8_t {
    svsqadd_u8_z(pg, op1, svdup_n_s8(op2))
}
#[doc = "Saturating add with signed addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsqadd[_u16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usqadd))]
pub fn svsqadd_u16_m(pg: svbool_t, op1: svuint16_t, op2: svint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.usqadd.nxv8i16")]
        fn _svsqadd_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svsqadd_u16_m(pg.into(), op1.as_signed(), op2).as_unsigned() }
}
#[doc = "Saturating add with signed addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsqadd[_n_u16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usqadd))]
pub fn svsqadd_n_u16_m(pg: svbool_t, op1: svuint16_t, op2: i16) -> svuint16_t {
    svsqadd_u16_m(pg, op1, svdup_n_s16(op2))
}
#[doc = "Saturating add with signed addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsqadd[_u16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usqadd))]
pub fn svsqadd_u16_x(pg: svbool_t, op1: svuint16_t, op2: svint16_t) -> svuint16_t {
    svsqadd_u16_m(pg, op1, op2)
}
#[doc = "Saturating add with signed addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsqadd[_n_u16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usqadd))]
pub fn svsqadd_n_u16_x(pg: svbool_t, op1: svuint16_t, op2: i16) -> svuint16_t {
    svsqadd_u16_x(pg, op1, svdup_n_s16(op2))
}
#[doc = "Saturating add with signed addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsqadd[_u16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usqadd))]
pub fn svsqadd_u16_z(pg: svbool_t, op1: svuint16_t, op2: svint16_t) -> svuint16_t {
    svsqadd_u16_m(pg, svsel_u16(pg, op1, svdup_n_u16(0)), op2)
}
#[doc = "Saturating add with signed addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsqadd[_n_u16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usqadd))]
pub fn svsqadd_n_u16_z(pg: svbool_t, op1: svuint16_t, op2: i16) -> svuint16_t {
    svsqadd_u16_z(pg, op1, svdup_n_s16(op2))
}
#[doc = "Saturating add with signed addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsqadd[_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usqadd))]
pub fn svsqadd_u32_m(pg: svbool_t, op1: svuint32_t, op2: svint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.usqadd.nxv4i32")]
        fn _svsqadd_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svsqadd_u32_m(pg.into(), op1.as_signed(), op2).as_unsigned() }
}
#[doc = "Saturating add with signed addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsqadd[_n_u32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usqadd))]
pub fn svsqadd_n_u32_m(pg: svbool_t, op1: svuint32_t, op2: i32) -> svuint32_t {
    svsqadd_u32_m(pg, op1, svdup_n_s32(op2))
}
#[doc = "Saturating add with signed addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsqadd[_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usqadd))]
pub fn svsqadd_u32_x(pg: svbool_t, op1: svuint32_t, op2: svint32_t) -> svuint32_t {
    svsqadd_u32_m(pg, op1, op2)
}
#[doc = "Saturating add with signed addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsqadd[_n_u32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usqadd))]
pub fn svsqadd_n_u32_x(pg: svbool_t, op1: svuint32_t, op2: i32) -> svuint32_t {
    svsqadd_u32_x(pg, op1, svdup_n_s32(op2))
}
#[doc = "Saturating add with signed addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsqadd[_u32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usqadd))]
pub fn svsqadd_u32_z(pg: svbool_t, op1: svuint32_t, op2: svint32_t) -> svuint32_t {
    svsqadd_u32_m(pg, svsel_u32(pg, op1, svdup_n_u32(0)), op2)
}
#[doc = "Saturating add with signed addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsqadd[_n_u32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usqadd))]
pub fn svsqadd_n_u32_z(pg: svbool_t, op1: svuint32_t, op2: i32) -> svuint32_t {
    svsqadd_u32_z(pg, op1, svdup_n_s32(op2))
}
#[doc = "Saturating add with signed addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsqadd[_u64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usqadd))]
pub fn svsqadd_u64_m(pg: svbool_t, op1: svuint64_t, op2: svint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.usqadd.nxv2i64")]
        fn _svsqadd_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svsqadd_u64_m(pg.into(), op1.as_signed(), op2).as_unsigned() }
}
#[doc = "Saturating add with signed addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsqadd[_n_u64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usqadd))]
pub fn svsqadd_n_u64_m(pg: svbool_t, op1: svuint64_t, op2: i64) -> svuint64_t {
    svsqadd_u64_m(pg, op1, svdup_n_s64(op2))
}
#[doc = "Saturating add with signed addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsqadd[_u64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usqadd))]
pub fn svsqadd_u64_x(pg: svbool_t, op1: svuint64_t, op2: svint64_t) -> svuint64_t {
    svsqadd_u64_m(pg, op1, op2)
}
#[doc = "Saturating add with signed addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsqadd[_n_u64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usqadd))]
pub fn svsqadd_n_u64_x(pg: svbool_t, op1: svuint64_t, op2: i64) -> svuint64_t {
    svsqadd_u64_x(pg, op1, svdup_n_s64(op2))
}
#[doc = "Saturating add with signed addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsqadd[_u64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usqadd))]
pub fn svsqadd_u64_z(pg: svbool_t, op1: svuint64_t, op2: svint64_t) -> svuint64_t {
    svsqadd_u64_m(pg, svsel_u64(pg, op1, svdup_n_u64(0)), op2)
}
#[doc = "Saturating add with signed addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsqadd[_n_u64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usqadd))]
pub fn svsqadd_n_u64_z(pg: svbool_t, op1: svuint64_t, op2: i64) -> svuint64_t {
    svsqadd_u64_z(pg, op1, svdup_n_s64(op2))
}
#[doc = "Shift right and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsra[_n_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssra, IMM3 = 1))]
pub fn svsra_n_s8<const IMM3: i32>(op1: svint8_t, op2: svint8_t) -> svint8_t {
    static_assert_range!(IMM3, 1, 8);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.ssra.nxv16i8")]
        fn _svsra_n_s8(op1: svint8_t, op2: svint8_t, imm3: i32) -> svint8_t;
    }
    unsafe { _svsra_n_s8(op1, op2, IMM3) }
}
#[doc = "Shift right and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsra[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssra, IMM3 = 1))]
pub fn svsra_n_s16<const IMM3: i32>(op1: svint16_t, op2: svint16_t) -> svint16_t {
    static_assert_range!(IMM3, 1, 16);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.ssra.nxv8i16")]
        fn _svsra_n_s16(op1: svint16_t, op2: svint16_t, imm3: i32) -> svint16_t;
    }
    unsafe { _svsra_n_s16(op1, op2, IMM3) }
}
#[doc = "Shift right and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsra[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssra, IMM3 = 1))]
pub fn svsra_n_s32<const IMM3: i32>(op1: svint32_t, op2: svint32_t) -> svint32_t {
    static_assert_range!(IMM3, 1, 32);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.ssra.nxv4i32")]
        fn _svsra_n_s32(op1: svint32_t, op2: svint32_t, imm3: i32) -> svint32_t;
    }
    unsafe { _svsra_n_s32(op1, op2, IMM3) }
}
#[doc = "Shift right and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsra[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssra, IMM3 = 1))]
pub fn svsra_n_s64<const IMM3: i32>(op1: svint64_t, op2: svint64_t) -> svint64_t {
    static_assert_range!(IMM3, 1, 64);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.ssra.nxv2i64")]
        fn _svsra_n_s64(op1: svint64_t, op2: svint64_t, imm3: i32) -> svint64_t;
    }
    unsafe { _svsra_n_s64(op1, op2, IMM3) }
}
#[doc = "Shift right and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsra[_n_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usra, IMM3 = 1))]
pub fn svsra_n_u8<const IMM3: i32>(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    static_assert_range!(IMM3, 1, 8);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.usra.nxv16i8")]
        fn _svsra_n_u8(op1: svint8_t, op2: svint8_t, imm3: i32) -> svint8_t;
    }
    unsafe { _svsra_n_u8(op1.as_signed(), op2.as_signed(), IMM3).as_unsigned() }
}
#[doc = "Shift right and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsra[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usra, IMM3 = 1))]
pub fn svsra_n_u16<const IMM3: i32>(op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    static_assert_range!(IMM3, 1, 16);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.usra.nxv8i16")]
        fn _svsra_n_u16(op1: svint16_t, op2: svint16_t, imm3: i32) -> svint16_t;
    }
    unsafe { _svsra_n_u16(op1.as_signed(), op2.as_signed(), IMM3).as_unsigned() }
}
#[doc = "Shift right and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsra[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usra, IMM3 = 1))]
pub fn svsra_n_u32<const IMM3: i32>(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    static_assert_range!(IMM3, 1, 32);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.usra.nxv4i32")]
        fn _svsra_n_u32(op1: svint32_t, op2: svint32_t, imm3: i32) -> svint32_t;
    }
    unsafe { _svsra_n_u32(op1.as_signed(), op2.as_signed(), IMM3).as_unsigned() }
}
#[doc = "Shift right and accumulate"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsra[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usra, IMM3 = 1))]
pub fn svsra_n_u64<const IMM3: i32>(op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    static_assert_range!(IMM3, 1, 64);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.usra.nxv2i64")]
        fn _svsra_n_u64(op1: svint64_t, op2: svint64_t, imm3: i32) -> svint64_t;
    }
    unsafe { _svsra_n_u64(op1.as_signed(), op2.as_signed(), IMM3).as_unsigned() }
}
#[doc = "Shift right and insert"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsri[_n_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sri, IMM3 = 1))]
pub fn svsri_n_s8<const IMM3: i32>(op1: svint8_t, op2: svint8_t) -> svint8_t {
    static_assert_range!(IMM3, 1, 8);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sri.nxv16i8")]
        fn _svsri_n_s8(op1: svint8_t, op2: svint8_t, imm3: i32) -> svint8_t;
    }
    unsafe { _svsri_n_s8(op1, op2, IMM3) }
}
#[doc = "Shift right and insert"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsri[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sri, IMM3 = 1))]
pub fn svsri_n_s16<const IMM3: i32>(op1: svint16_t, op2: svint16_t) -> svint16_t {
    static_assert_range!(IMM3, 1, 16);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sri.nxv8i16")]
        fn _svsri_n_s16(op1: svint16_t, op2: svint16_t, imm3: i32) -> svint16_t;
    }
    unsafe { _svsri_n_s16(op1, op2, IMM3) }
}
#[doc = "Shift right and insert"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsri[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sri, IMM3 = 1))]
pub fn svsri_n_s32<const IMM3: i32>(op1: svint32_t, op2: svint32_t) -> svint32_t {
    static_assert_range!(IMM3, 1, 32);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sri.nxv4i32")]
        fn _svsri_n_s32(op1: svint32_t, op2: svint32_t, imm3: i32) -> svint32_t;
    }
    unsafe { _svsri_n_s32(op1, op2, IMM3) }
}
#[doc = "Shift right and insert"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsri[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sri, IMM3 = 1))]
pub fn svsri_n_s64<const IMM3: i32>(op1: svint64_t, op2: svint64_t) -> svint64_t {
    static_assert_range!(IMM3, 1, 64);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.sri.nxv2i64")]
        fn _svsri_n_s64(op1: svint64_t, op2: svint64_t, imm3: i32) -> svint64_t;
    }
    unsafe { _svsri_n_s64(op1, op2, IMM3) }
}
#[doc = "Shift right and insert"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsri[_n_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sri, IMM3 = 1))]
pub fn svsri_n_u8<const IMM3: i32>(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    static_assert_range!(IMM3, 1, 8);
    unsafe { svsri_n_s8::<IMM3>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Shift right and insert"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsri[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sri, IMM3 = 1))]
pub fn svsri_n_u16<const IMM3: i32>(op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    static_assert_range!(IMM3, 1, 16);
    unsafe { svsri_n_s16::<IMM3>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Shift right and insert"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsri[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sri, IMM3 = 1))]
pub fn svsri_n_u32<const IMM3: i32>(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    static_assert_range!(IMM3, 1, 32);
    unsafe { svsri_n_s32::<IMM3>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Shift right and insert"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsri[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sri, IMM3 = 1))]
pub fn svsri_n_u64<const IMM3: i32>(op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    static_assert_range!(IMM3, 1, 64);
    unsafe { svsri_n_s64::<IMM3>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter_[s64]index[_f64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_s64index_f64(
    pg: svbool_t,
    base: *mut f64,
    indices: svint64_t,
    data: svfloat64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.stnt1.scatter.index.nxv2f64"
        )]
        fn _svstnt1_scatter_s64index_f64(
            data: svfloat64_t,
            pg: svbool2_t,
            base: *mut f64,
            indices: svint64_t,
        );
    }
    _svstnt1_scatter_s64index_f64(data, pg.into(), base, indices)
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter_[s64]index[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_s64index_s64(
    pg: svbool_t,
    base: *mut i64,
    indices: svint64_t,
    data: svint64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.stnt1.scatter.index.nxv2i64"
        )]
        fn _svstnt1_scatter_s64index_s64(
            data: svint64_t,
            pg: svbool2_t,
            base: *mut i64,
            indices: svint64_t,
        );
    }
    _svstnt1_scatter_s64index_s64(data, pg.into(), base, indices)
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter_[s64]index[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_s64index_u64(
    pg: svbool_t,
    base: *mut u64,
    indices: svint64_t,
    data: svuint64_t,
) {
    svstnt1_scatter_s64index_s64(pg, base.as_signed(), indices, data.as_signed())
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter_[u64]index[_f64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64index_f64(
    pg: svbool_t,
    base: *mut f64,
    indices: svuint64_t,
    data: svfloat64_t,
) {
    svstnt1_scatter_s64index_f64(pg, base, indices.as_signed(), data)
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter_[u64]index[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64index_s64(
    pg: svbool_t,
    base: *mut i64,
    indices: svuint64_t,
    data: svint64_t,
) {
    svstnt1_scatter_s64index_s64(pg, base, indices.as_signed(), data)
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter_[u64]index[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64index_u64(
    pg: svbool_t,
    base: *mut u64,
    indices: svuint64_t,
    data: svuint64_t,
) {
    svstnt1_scatter_s64index_s64(pg, base.as_signed(), indices.as_signed(), data.as_signed())
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter_[s64]offset[_f64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_s64offset_f64(
    pg: svbool_t,
    base: *mut f64,
    offsets: svint64_t,
    data: svfloat64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.stnt1.scatter.nxv2f64"
        )]
        fn _svstnt1_scatter_s64offset_f64(
            data: svfloat64_t,
            pg: svbool2_t,
            base: *mut f64,
            offsets: svint64_t,
        );
    }
    _svstnt1_scatter_s64offset_f64(data, pg.into(), base, offsets)
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter_[s64]offset[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_s64offset_s64(
    pg: svbool_t,
    base: *mut i64,
    offsets: svint64_t,
    data: svint64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.stnt1.scatter.nxv2i64"
        )]
        fn _svstnt1_scatter_s64offset_s64(
            data: svint64_t,
            pg: svbool2_t,
            base: *mut i64,
            offsets: svint64_t,
        );
    }
    _svstnt1_scatter_s64offset_s64(data, pg.into(), base, offsets)
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter_[s64]offset[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_s64offset_u64(
    pg: svbool_t,
    base: *mut u64,
    offsets: svint64_t,
    data: svuint64_t,
) {
    svstnt1_scatter_s64offset_s64(pg, base.as_signed(), offsets, data.as_signed())
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter_[u32]offset[_f32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1_scatter_u32offset_f32(
    pg: svbool_t,
    base: *mut f32,
    offsets: svuint32_t,
    data: svfloat32_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.stnt1.scatter.uxtw.nxv4f32"
        )]
        fn _svstnt1_scatter_u32offset_f32(
            data: svfloat32_t,
            pg: svbool4_t,
            base: *mut f32,
            offsets: svint32_t,
        );
    }
    _svstnt1_scatter_u32offset_f32(data, pg.into(), base, offsets.as_signed())
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter_[u32]offset[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1_scatter_u32offset_s32(
    pg: svbool_t,
    base: *mut i32,
    offsets: svuint32_t,
    data: svint32_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.stnt1.scatter.uxtw.nxv4i32"
        )]
        fn _svstnt1_scatter_u32offset_s32(
            data: svint32_t,
            pg: svbool4_t,
            base: *mut i32,
            offsets: svint32_t,
        );
    }
    _svstnt1_scatter_u32offset_s32(data, pg.into(), base, offsets.as_signed())
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter_[u32]offset[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1_scatter_u32offset_u32(
    pg: svbool_t,
    base: *mut u32,
    offsets: svuint32_t,
    data: svuint32_t,
) {
    svstnt1_scatter_u32offset_s32(pg, base.as_signed(), offsets, data.as_signed())
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter_[u64]offset[_f64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64offset_f64(
    pg: svbool_t,
    base: *mut f64,
    offsets: svuint64_t,
    data: svfloat64_t,
) {
    svstnt1_scatter_s64offset_f64(pg, base, offsets.as_signed(), data)
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter_[u64]offset[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64offset_s64(
    pg: svbool_t,
    base: *mut i64,
    offsets: svuint64_t,
    data: svint64_t,
) {
    svstnt1_scatter_s64offset_s64(pg, base, offsets.as_signed(), data)
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter_[u64]offset[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64offset_u64(
    pg: svbool_t,
    base: *mut u64,
    offsets: svuint64_t,
    data: svuint64_t,
) {
    svstnt1_scatter_s64offset_s64(pg, base.as_signed(), offsets.as_signed(), data.as_signed())
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter[_u32base_f32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1_scatter_u32base_f32(pg: svbool_t, bases: svuint32_t, data: svfloat32_t) {
    svstnt1_scatter_u32base_offset_f32(pg, bases, 0, data)
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter[_u32base_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1_scatter_u32base_s32(pg: svbool_t, bases: svuint32_t, data: svint32_t) {
    svstnt1_scatter_u32base_offset_s32(pg, bases, 0, data)
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter[_u32base_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1_scatter_u32base_u32(pg: svbool_t, bases: svuint32_t, data: svuint32_t) {
    svstnt1_scatter_u32base_offset_u32(pg, bases, 0, data)
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter[_u64base_f64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64base_f64(pg: svbool_t, bases: svuint64_t, data: svfloat64_t) {
    svstnt1_scatter_u64base_offset_f64(pg, bases, 0, data)
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter[_u64base_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64base_s64(pg: svbool_t, bases: svuint64_t, data: svint64_t) {
    svstnt1_scatter_u64base_offset_s64(pg, bases, 0, data)
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter[_u64base_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64base_u64(pg: svbool_t, bases: svuint64_t, data: svuint64_t) {
    svstnt1_scatter_u64base_offset_u64(pg, bases, 0, data)
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter[_u32base]_index[_f32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1_scatter_u32base_index_f32(
    pg: svbool_t,
    bases: svuint32_t,
    index: i64,
    data: svfloat32_t,
) {
    svstnt1_scatter_u32base_offset_f32(pg, bases, index.unchecked_shl(2), data)
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter[_u32base]_index[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1_scatter_u32base_index_s32(
    pg: svbool_t,
    bases: svuint32_t,
    index: i64,
    data: svint32_t,
) {
    svstnt1_scatter_u32base_offset_s32(pg, bases, index.unchecked_shl(2), data)
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter[_u32base]_index[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1_scatter_u32base_index_u32(
    pg: svbool_t,
    bases: svuint32_t,
    index: i64,
    data: svuint32_t,
) {
    svstnt1_scatter_u32base_offset_u32(pg, bases, index.unchecked_shl(2), data)
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter[_u64base]_index[_f64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64base_index_f64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
    data: svfloat64_t,
) {
    svstnt1_scatter_u64base_offset_f64(pg, bases, index.unchecked_shl(3), data)
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter[_u64base]_index[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64base_index_s64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
    data: svint64_t,
) {
    svstnt1_scatter_u64base_offset_s64(pg, bases, index.unchecked_shl(3), data)
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter[_u64base]_index[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64base_index_u64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
    data: svuint64_t,
) {
    svstnt1_scatter_u64base_offset_u64(pg, bases, index.unchecked_shl(3), data)
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter[_u32base]_offset[_f32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1_scatter_u32base_offset_f32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
    data: svfloat32_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.stnt1.scatter.scalar.offset.nxv4f32.nxv4i32"
        )]
        fn _svstnt1_scatter_u32base_offset_f32(
            data: svfloat32_t,
            pg: svbool4_t,
            bases: svint32_t,
            offset: i64,
        );
    }
    _svstnt1_scatter_u32base_offset_f32(data, pg.into(), bases.as_signed(), offset)
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter[_u32base]_offset[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1_scatter_u32base_offset_s32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
    data: svint32_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.stnt1.scatter.scalar.offset.nxv4i32.nxv4i32"
        )]
        fn _svstnt1_scatter_u32base_offset_s32(
            data: svint32_t,
            pg: svbool4_t,
            bases: svint32_t,
            offset: i64,
        );
    }
    _svstnt1_scatter_u32base_offset_s32(data, pg.into(), bases.as_signed(), offset)
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter[_u32base]_offset[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1_scatter_u32base_offset_u32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
    data: svuint32_t,
) {
    svstnt1_scatter_u32base_offset_s32(pg, bases, offset, data.as_signed())
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter[_u64base]_offset[_f64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64base_offset_f64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
    data: svfloat64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.stnt1.scatter.scalar.offset.nxv2f64.nxv2i64"
        )]
        fn _svstnt1_scatter_u64base_offset_f64(
            data: svfloat64_t,
            pg: svbool2_t,
            bases: svint64_t,
            offset: i64,
        );
    }
    _svstnt1_scatter_u64base_offset_f64(data, pg.into(), bases.as_signed(), offset)
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter[_u64base]_offset[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64base_offset_s64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
    data: svint64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.stnt1.scatter.scalar.offset.nxv2i64.nxv2i64"
        )]
        fn _svstnt1_scatter_u64base_offset_s64(
            data: svint64_t,
            pg: svbool2_t,
            bases: svint64_t,
            offset: i64,
        );
    }
    _svstnt1_scatter_u64base_offset_s64(data, pg.into(), bases.as_signed(), offset)
}
#[doc = "Non-truncating store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1_scatter[_u64base]_offset[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64base_offset_u64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
    data: svuint64_t,
) {
    svstnt1_scatter_u64base_offset_s64(pg, bases, offset, data.as_signed())
}
#[doc = "Truncate to 8 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1b_scatter_[s64]offset[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_s64offset_s64(
    pg: svbool_t,
    base: *mut i8,
    offsets: svint64_t,
    data: svint64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.stnt1.scatter.nxv2i8"
        )]
        fn _svstnt1b_scatter_s64offset_s64(
            data: nxv2i8,
            pg: svbool2_t,
            base: *mut i8,
            offsets: svint64_t,
        );
    }
    _svstnt1b_scatter_s64offset_s64(simd_cast(data), pg.into(), base, offsets)
}
#[doc = "Truncate to 16 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1h_scatter_[s64]offset[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_s64offset_s64(
    pg: svbool_t,
    base: *mut i16,
    offsets: svint64_t,
    data: svint64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.stnt1.scatter.nxv2i16"
        )]
        fn _svstnt1h_scatter_s64offset_s64(
            data: nxv2i16,
            pg: svbool2_t,
            base: *mut i16,
            offsets: svint64_t,
        );
    }
    _svstnt1h_scatter_s64offset_s64(simd_cast(data), pg.into(), base, offsets)
}
#[doc = "Truncate to 32 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1w_scatter_[s64]offset[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_s64offset_s64(
    pg: svbool_t,
    base: *mut i32,
    offsets: svint64_t,
    data: svint64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.stnt1.scatter.nxv2i32"
        )]
        fn _svstnt1w_scatter_s64offset_s64(
            data: nxv2i32,
            pg: svbool2_t,
            base: *mut i32,
            offsets: svint64_t,
        );
    }
    _svstnt1w_scatter_s64offset_s64(simd_cast(data), pg.into(), base, offsets)
}
#[doc = "Truncate to 8 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1b_scatter_[s64]offset[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_s64offset_u64(
    pg: svbool_t,
    base: *mut u8,
    offsets: svint64_t,
    data: svuint64_t,
) {
    svstnt1b_scatter_s64offset_s64(pg, base.as_signed(), offsets, data.as_signed())
}
#[doc = "Truncate to 16 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1h_scatter_[s64]offset[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_s64offset_u64(
    pg: svbool_t,
    base: *mut u16,
    offsets: svint64_t,
    data: svuint64_t,
) {
    svstnt1h_scatter_s64offset_s64(pg, base.as_signed(), offsets, data.as_signed())
}
#[doc = "Truncate to 32 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1w_scatter_[s64]offset[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_s64offset_u64(
    pg: svbool_t,
    base: *mut u32,
    offsets: svint64_t,
    data: svuint64_t,
) {
    svstnt1w_scatter_s64offset_s64(pg, base.as_signed(), offsets, data.as_signed())
}
#[doc = "Truncate to 8 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1b_scatter_[u32]offset[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_u32offset_s32(
    pg: svbool_t,
    base: *mut i8,
    offsets: svuint32_t,
    data: svint32_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.stnt1.scatter.uxtw.nxv4i8"
        )]
        fn _svstnt1b_scatter_u32offset_s32(
            data: nxv4i8,
            pg: svbool4_t,
            base: *mut i8,
            offsets: svint32_t,
        );
    }
    _svstnt1b_scatter_u32offset_s32(simd_cast(data), pg.into(), base, offsets.as_signed())
}
#[doc = "Truncate to 16 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1h_scatter_[u32]offset[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u32offset_s32(
    pg: svbool_t,
    base: *mut i16,
    offsets: svuint32_t,
    data: svint32_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.stnt1.scatter.uxtw.nxv4i16"
        )]
        fn _svstnt1h_scatter_u32offset_s32(
            data: nxv4i16,
            pg: svbool4_t,
            base: *mut i16,
            offsets: svint32_t,
        );
    }
    _svstnt1h_scatter_u32offset_s32(simd_cast(data), pg.into(), base, offsets.as_signed())
}
#[doc = "Truncate to 8 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1b_scatter_[u32]offset[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_u32offset_u32(
    pg: svbool_t,
    base: *mut u8,
    offsets: svuint32_t,
    data: svuint32_t,
) {
    svstnt1b_scatter_u32offset_s32(pg, base.as_signed(), offsets, data.as_signed())
}
#[doc = "Truncate to 16 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1h_scatter_[u32]offset[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u32offset_u32(
    pg: svbool_t,
    base: *mut u16,
    offsets: svuint32_t,
    data: svuint32_t,
) {
    svstnt1h_scatter_u32offset_s32(pg, base.as_signed(), offsets, data.as_signed())
}
#[doc = "Truncate to 8 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1b_scatter_[u64]offset[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_u64offset_s64(
    pg: svbool_t,
    base: *mut i8,
    offsets: svuint64_t,
    data: svint64_t,
) {
    svstnt1b_scatter_s64offset_s64(pg, base, offsets.as_signed(), data)
}
#[doc = "Truncate to 16 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1h_scatter_[u64]offset[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u64offset_s64(
    pg: svbool_t,
    base: *mut i16,
    offsets: svuint64_t,
    data: svint64_t,
) {
    svstnt1h_scatter_s64offset_s64(pg, base, offsets.as_signed(), data)
}
#[doc = "Truncate to 32 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1w_scatter_[u64]offset[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_u64offset_s64(
    pg: svbool_t,
    base: *mut i32,
    offsets: svuint64_t,
    data: svint64_t,
) {
    svstnt1w_scatter_s64offset_s64(pg, base, offsets.as_signed(), data)
}
#[doc = "Truncate to 8 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1b_scatter_[u64]offset[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_u64offset_u64(
    pg: svbool_t,
    base: *mut u8,
    offsets: svuint64_t,
    data: svuint64_t,
) {
    svstnt1b_scatter_s64offset_s64(pg, base.as_signed(), offsets.as_signed(), data.as_signed())
}
#[doc = "Truncate to 16 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1h_scatter_[u64]offset[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u64offset_u64(
    pg: svbool_t,
    base: *mut u16,
    offsets: svuint64_t,
    data: svuint64_t,
) {
    svstnt1h_scatter_s64offset_s64(pg, base.as_signed(), offsets.as_signed(), data.as_signed())
}
#[doc = "Truncate to 32 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1w_scatter_[u64]offset[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_u64offset_u64(
    pg: svbool_t,
    base: *mut u32,
    offsets: svuint64_t,
    data: svuint64_t,
) {
    svstnt1w_scatter_s64offset_s64(pg, base.as_signed(), offsets.as_signed(), data.as_signed())
}
#[doc = "Truncate to 8 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1b_scatter[_u32base]_offset[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_u32base_offset_s32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
    data: svint32_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.stnt1.scatter.scalar.offset.nxv4i8.nxv4i32"
        )]
        fn _svstnt1b_scatter_u32base_offset_s32(
            data: nxv4i8,
            pg: svbool4_t,
            bases: svint32_t,
            offset: i64,
        );
    }
    _svstnt1b_scatter_u32base_offset_s32(simd_cast(data), pg.into(), bases.as_signed(), offset)
}
#[doc = "Truncate to 16 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1h_scatter[_u32base]_offset[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u32base_offset_s32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
    data: svint32_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.stnt1.scatter.scalar.offset.nxv4i16.nxv4i32"
        )]
        fn _svstnt1h_scatter_u32base_offset_s32(
            data: nxv4i16,
            pg: svbool4_t,
            bases: svint32_t,
            offset: i64,
        );
    }
    _svstnt1h_scatter_u32base_offset_s32(simd_cast(data), pg.into(), bases.as_signed(), offset)
}
#[doc = "Truncate to 8 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1b_scatter[_u32base]_offset[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_u32base_offset_u32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
    data: svuint32_t,
) {
    svstnt1b_scatter_u32base_offset_s32(pg, bases, offset, data.as_signed())
}
#[doc = "Truncate to 16 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1h_scatter[_u32base]_offset[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u32base_offset_u32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
    data: svuint32_t,
) {
    svstnt1h_scatter_u32base_offset_s32(pg, bases, offset, data.as_signed())
}
#[doc = "Truncate to 8 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1b_scatter[_u64base]_offset[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_u64base_offset_s64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
    data: svint64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.stnt1.scatter.scalar.offset.nxv2i8.nxv2i64"
        )]
        fn _svstnt1b_scatter_u64base_offset_s64(
            data: nxv2i8,
            pg: svbool2_t,
            bases: svint64_t,
            offset: i64,
        );
    }
    _svstnt1b_scatter_u64base_offset_s64(simd_cast(data), pg.into(), bases.as_signed(), offset)
}
#[doc = "Truncate to 16 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1h_scatter[_u64base]_offset[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u64base_offset_s64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
    data: svint64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.stnt1.scatter.scalar.offset.nxv2i16.nxv2i64"
        )]
        fn _svstnt1h_scatter_u64base_offset_s64(
            data: nxv2i16,
            pg: svbool2_t,
            bases: svint64_t,
            offset: i64,
        );
    }
    _svstnt1h_scatter_u64base_offset_s64(simd_cast(data), pg.into(), bases.as_signed(), offset)
}
#[doc = "Truncate to 32 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1w_scatter[_u64base]_offset[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_u64base_offset_s64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
    data: svint64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.stnt1.scatter.scalar.offset.nxv2i32.nxv2i64"
        )]
        fn _svstnt1w_scatter_u64base_offset_s64(
            data: nxv2i32,
            pg: svbool2_t,
            bases: svint64_t,
            offset: i64,
        );
    }
    _svstnt1w_scatter_u64base_offset_s64(simd_cast(data), pg.into(), bases.as_signed(), offset)
}
#[doc = "Truncate to 8 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1b_scatter[_u64base]_offset[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_u64base_offset_u64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
    data: svuint64_t,
) {
    svstnt1b_scatter_u64base_offset_s64(pg, bases, offset, data.as_signed())
}
#[doc = "Truncate to 16 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1h_scatter[_u64base]_offset[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u64base_offset_u64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
    data: svuint64_t,
) {
    svstnt1h_scatter_u64base_offset_s64(pg, bases, offset, data.as_signed())
}
#[doc = "Truncate to 32 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1w_scatter[_u64base]_offset[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_u64base_offset_u64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
    data: svuint64_t,
) {
    svstnt1w_scatter_u64base_offset_s64(pg, bases, offset, data.as_signed())
}
#[doc = "Truncate to 8 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1b_scatter[_u32base_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_u32base_s32(pg: svbool_t, bases: svuint32_t, data: svint32_t) {
    svstnt1b_scatter_u32base_offset_s32(pg, bases, 0, data)
}
#[doc = "Truncate to 16 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1h_scatter[_u32base_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u32base_s32(pg: svbool_t, bases: svuint32_t, data: svint32_t) {
    svstnt1h_scatter_u32base_offset_s32(pg, bases, 0, data)
}
#[doc = "Truncate to 8 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1b_scatter[_u32base_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_u32base_u32(pg: svbool_t, bases: svuint32_t, data: svuint32_t) {
    svstnt1b_scatter_u32base_offset_u32(pg, bases, 0, data)
}
#[doc = "Truncate to 16 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1h_scatter[_u32base_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u32base_u32(pg: svbool_t, bases: svuint32_t, data: svuint32_t) {
    svstnt1h_scatter_u32base_offset_u32(pg, bases, 0, data)
}
#[doc = "Truncate to 8 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1b_scatter[_u64base_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_u64base_s64(pg: svbool_t, bases: svuint64_t, data: svint64_t) {
    svstnt1b_scatter_u64base_offset_s64(pg, bases, 0, data)
}
#[doc = "Truncate to 16 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1h_scatter[_u64base_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u64base_s64(pg: svbool_t, bases: svuint64_t, data: svint64_t) {
    svstnt1h_scatter_u64base_offset_s64(pg, bases, 0, data)
}
#[doc = "Truncate to 32 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1w_scatter[_u64base_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_u64base_s64(pg: svbool_t, bases: svuint64_t, data: svint64_t) {
    svstnt1w_scatter_u64base_offset_s64(pg, bases, 0, data)
}
#[doc = "Truncate to 8 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1b_scatter[_u64base_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_u64base_u64(pg: svbool_t, bases: svuint64_t, data: svuint64_t) {
    svstnt1b_scatter_u64base_offset_u64(pg, bases, 0, data)
}
#[doc = "Truncate to 16 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1h_scatter[_u64base_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u64base_u64(pg: svbool_t, bases: svuint64_t, data: svuint64_t) {
    svstnt1h_scatter_u64base_offset_u64(pg, bases, 0, data)
}
#[doc = "Truncate to 32 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1w_scatter[_u64base_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_u64base_u64(pg: svbool_t, bases: svuint64_t, data: svuint64_t) {
    svstnt1w_scatter_u64base_offset_u64(pg, bases, 0, data)
}
#[doc = "Truncate to 16 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1h_scatter_[s64]index[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_s64index_s64(
    pg: svbool_t,
    base: *mut i16,
    indices: svint64_t,
    data: svint64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.stnt1.scatter.index.nxv2i16"
        )]
        fn _svstnt1h_scatter_s64index_s64(
            data: nxv2i16,
            pg: svbool2_t,
            base: *mut i16,
            indices: svint64_t,
        );
    }
    _svstnt1h_scatter_s64index_s64(simd_cast(data), pg.into(), base, indices)
}
#[doc = "Truncate to 32 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1w_scatter_[s64]index[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_s64index_s64(
    pg: svbool_t,
    base: *mut i32,
    indices: svint64_t,
    data: svint64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.stnt1.scatter.index.nxv2i32"
        )]
        fn _svstnt1w_scatter_s64index_s64(
            data: nxv2i32,
            pg: svbool2_t,
            base: *mut i32,
            indices: svint64_t,
        );
    }
    _svstnt1w_scatter_s64index_s64(simd_cast(data), pg.into(), base, indices)
}
#[doc = "Truncate to 16 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1h_scatter_[s64]index[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_s64index_u64(
    pg: svbool_t,
    base: *mut u16,
    indices: svint64_t,
    data: svuint64_t,
) {
    svstnt1h_scatter_s64index_s64(pg, base.as_signed(), indices, data.as_signed())
}
#[doc = "Truncate to 32 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1w_scatter_[s64]index[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_s64index_u64(
    pg: svbool_t,
    base: *mut u32,
    indices: svint64_t,
    data: svuint64_t,
) {
    svstnt1w_scatter_s64index_s64(pg, base.as_signed(), indices, data.as_signed())
}
#[doc = "Truncate to 16 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1h_scatter_[u64]index[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u64index_s64(
    pg: svbool_t,
    base: *mut i16,
    indices: svuint64_t,
    data: svint64_t,
) {
    svstnt1h_scatter_s64index_s64(pg, base, indices.as_signed(), data)
}
#[doc = "Truncate to 32 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1w_scatter_[u64]index[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_u64index_s64(
    pg: svbool_t,
    base: *mut i32,
    indices: svuint64_t,
    data: svint64_t,
) {
    svstnt1w_scatter_s64index_s64(pg, base, indices.as_signed(), data)
}
#[doc = "Truncate to 16 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1h_scatter_[u64]index[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u64index_u64(
    pg: svbool_t,
    base: *mut u16,
    indices: svuint64_t,
    data: svuint64_t,
) {
    svstnt1h_scatter_s64index_s64(pg, base.as_signed(), indices.as_signed(), data.as_signed())
}
#[doc = "Truncate to 32 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1w_scatter_[u64]index[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_u64index_u64(
    pg: svbool_t,
    base: *mut u32,
    indices: svuint64_t,
    data: svuint64_t,
) {
    svstnt1w_scatter_s64index_s64(pg, base.as_signed(), indices.as_signed(), data.as_signed())
}
#[doc = "Truncate to 16 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1h_scatter[_u32base]_index[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u32base_index_s32(
    pg: svbool_t,
    bases: svuint32_t,
    index: i64,
    data: svint32_t,
) {
    svstnt1h_scatter_u32base_offset_s32(pg, bases, index.unchecked_shl(1), data)
}
#[doc = "Truncate to 16 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1h_scatter[_u32base]_index[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u32base_index_u32(
    pg: svbool_t,
    bases: svuint32_t,
    index: i64,
    data: svuint32_t,
) {
    svstnt1h_scatter_u32base_offset_u32(pg, bases, index.unchecked_shl(1), data)
}
#[doc = "Truncate to 16 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1h_scatter[_u64base]_index[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u64base_index_s64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
    data: svint64_t,
) {
    svstnt1h_scatter_u64base_offset_s64(pg, bases, index.unchecked_shl(1), data)
}
#[doc = "Truncate to 32 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1w_scatter[_u64base]_index[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_u64base_index_s64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
    data: svint64_t,
) {
    svstnt1w_scatter_u64base_offset_s64(pg, bases, index.unchecked_shl(2), data)
}
#[doc = "Truncate to 16 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1h_scatter[_u64base]_index[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u64base_index_u64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
    data: svuint64_t,
) {
    svstnt1h_scatter_u64base_offset_u64(pg, bases, index.unchecked_shl(1), data)
}
#[doc = "Truncate to 32 bits and store, non-temporal"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svstnt1w_scatter[_u64base]_index[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_u64base_index_u64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
    data: svuint64_t,
) {
    svstnt1w_scatter_u64base_offset_u64(pg, bases, index.unchecked_shl(2), data)
}
#[doc = "Subtract narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubhnb[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(subhnb))]
pub fn svsubhnb_s16(op1: svint16_t, op2: svint16_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.subhnb.nxv8i16")]
        fn _svsubhnb_s16(op1: svint16_t, op2: svint16_t) -> svint8_t;
    }
    unsafe { _svsubhnb_s16(op1, op2) }
}
#[doc = "Subtract narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubhnb[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(subhnb))]
pub fn svsubhnb_n_s16(op1: svint16_t, op2: i16) -> svint8_t {
    svsubhnb_s16(op1, svdup_n_s16(op2))
}
#[doc = "Subtract narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubhnb[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(subhnb))]
pub fn svsubhnb_s32(op1: svint32_t, op2: svint32_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.subhnb.nxv4i32")]
        fn _svsubhnb_s32(op1: svint32_t, op2: svint32_t) -> svint16_t;
    }
    unsafe { _svsubhnb_s32(op1, op2) }
}
#[doc = "Subtract narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubhnb[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(subhnb))]
pub fn svsubhnb_n_s32(op1: svint32_t, op2: i32) -> svint16_t {
    svsubhnb_s32(op1, svdup_n_s32(op2))
}
#[doc = "Subtract narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubhnb[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(subhnb))]
pub fn svsubhnb_s64(op1: svint64_t, op2: svint64_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.subhnb.nxv2i64")]
        fn _svsubhnb_s64(op1: svint64_t, op2: svint64_t) -> svint32_t;
    }
    unsafe { _svsubhnb_s64(op1, op2) }
}
#[doc = "Subtract narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubhnb[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(subhnb))]
pub fn svsubhnb_n_s64(op1: svint64_t, op2: i64) -> svint32_t {
    svsubhnb_s64(op1, svdup_n_s64(op2))
}
#[doc = "Subtract narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubhnb[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(subhnb))]
pub fn svsubhnb_u16(op1: svuint16_t, op2: svuint16_t) -> svuint8_t {
    unsafe { svsubhnb_s16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Subtract narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubhnb[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(subhnb))]
pub fn svsubhnb_n_u16(op1: svuint16_t, op2: u16) -> svuint8_t {
    svsubhnb_u16(op1, svdup_n_u16(op2))
}
#[doc = "Subtract narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubhnb[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(subhnb))]
pub fn svsubhnb_u32(op1: svuint32_t, op2: svuint32_t) -> svuint16_t {
    unsafe { svsubhnb_s32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Subtract narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubhnb[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(subhnb))]
pub fn svsubhnb_n_u32(op1: svuint32_t, op2: u32) -> svuint16_t {
    svsubhnb_u32(op1, svdup_n_u32(op2))
}
#[doc = "Subtract narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubhnb[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(subhnb))]
pub fn svsubhnb_u64(op1: svuint64_t, op2: svuint64_t) -> svuint32_t {
    unsafe { svsubhnb_s64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Subtract narrow high part (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubhnb[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(subhnb))]
pub fn svsubhnb_n_u64(op1: svuint64_t, op2: u64) -> svuint32_t {
    svsubhnb_u64(op1, svdup_n_u64(op2))
}
#[doc = "Subtract narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubhnt[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(subhnt))]
pub fn svsubhnt_s16(even: svint8_t, op1: svint16_t, op2: svint16_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.subhnt.nxv8i16")]
        fn _svsubhnt_s16(even: svint8_t, op1: svint16_t, op2: svint16_t) -> svint8_t;
    }
    unsafe { _svsubhnt_s16(even, op1, op2) }
}
#[doc = "Subtract narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubhnt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(subhnt))]
pub fn svsubhnt_n_s16(even: svint8_t, op1: svint16_t, op2: i16) -> svint8_t {
    svsubhnt_s16(even, op1, svdup_n_s16(op2))
}
#[doc = "Subtract narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubhnt[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(subhnt))]
pub fn svsubhnt_s32(even: svint16_t, op1: svint32_t, op2: svint32_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.subhnt.nxv4i32")]
        fn _svsubhnt_s32(even: svint16_t, op1: svint32_t, op2: svint32_t) -> svint16_t;
    }
    unsafe { _svsubhnt_s32(even, op1, op2) }
}
#[doc = "Subtract narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubhnt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(subhnt))]
pub fn svsubhnt_n_s32(even: svint16_t, op1: svint32_t, op2: i32) -> svint16_t {
    svsubhnt_s32(even, op1, svdup_n_s32(op2))
}
#[doc = "Subtract narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubhnt[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(subhnt))]
pub fn svsubhnt_s64(even: svint32_t, op1: svint64_t, op2: svint64_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.subhnt.nxv2i64")]
        fn _svsubhnt_s64(even: svint32_t, op1: svint64_t, op2: svint64_t) -> svint32_t;
    }
    unsafe { _svsubhnt_s64(even, op1, op2) }
}
#[doc = "Subtract narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubhnt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(subhnt))]
pub fn svsubhnt_n_s64(even: svint32_t, op1: svint64_t, op2: i64) -> svint32_t {
    svsubhnt_s64(even, op1, svdup_n_s64(op2))
}
#[doc = "Subtract narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubhnt[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(subhnt))]
pub fn svsubhnt_u16(even: svuint8_t, op1: svuint16_t, op2: svuint16_t) -> svuint8_t {
    unsafe { svsubhnt_s16(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Subtract narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubhnt[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(subhnt))]
pub fn svsubhnt_n_u16(even: svuint8_t, op1: svuint16_t, op2: u16) -> svuint8_t {
    svsubhnt_u16(even, op1, svdup_n_u16(op2))
}
#[doc = "Subtract narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubhnt[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(subhnt))]
pub fn svsubhnt_u32(even: svuint16_t, op1: svuint32_t, op2: svuint32_t) -> svuint16_t {
    unsafe { svsubhnt_s32(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Subtract narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubhnt[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(subhnt))]
pub fn svsubhnt_n_u32(even: svuint16_t, op1: svuint32_t, op2: u32) -> svuint16_t {
    svsubhnt_u32(even, op1, svdup_n_u32(op2))
}
#[doc = "Subtract narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubhnt[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(subhnt))]
pub fn svsubhnt_u64(even: svuint32_t, op1: svuint64_t, op2: svuint64_t) -> svuint32_t {
    unsafe { svsubhnt_s64(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Subtract narrow high part (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubhnt[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(subhnt))]
pub fn svsubhnt_n_u64(even: svuint32_t, op1: svuint64_t, op2: u64) -> svuint32_t {
    svsubhnt_u64(even, op1, svdup_n_u64(op2))
}
#[doc = "Subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublb[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssublb))]
pub fn svsublb_s16(op1: svint8_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.ssublb.nxv8i16")]
        fn _svsublb_s16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svsublb_s16(op1, op2) }
}
#[doc = "Subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublb[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssublb))]
pub fn svsublb_n_s16(op1: svint8_t, op2: i8) -> svint16_t {
    svsublb_s16(op1, svdup_n_s8(op2))
}
#[doc = "Subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublb[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssublb))]
pub fn svsublb_s32(op1: svint16_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.ssublb.nxv4i32")]
        fn _svsublb_s32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svsublb_s32(op1, op2) }
}
#[doc = "Subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublb[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssublb))]
pub fn svsublb_n_s32(op1: svint16_t, op2: i16) -> svint32_t {
    svsublb_s32(op1, svdup_n_s16(op2))
}
#[doc = "Subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublb[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssublb))]
pub fn svsublb_s64(op1: svint32_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.ssublb.nxv2i64")]
        fn _svsublb_s64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svsublb_s64(op1, op2) }
}
#[doc = "Subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublb[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssublb))]
pub fn svsublb_n_s64(op1: svint32_t, op2: i32) -> svint64_t {
    svsublb_s64(op1, svdup_n_s32(op2))
}
#[doc = "Subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublb[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usublb))]
pub fn svsublb_u16(op1: svuint8_t, op2: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.usublb.nxv8i16")]
        fn _svsublb_u16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svsublb_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublb[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usublb))]
pub fn svsublb_n_u16(op1: svuint8_t, op2: u8) -> svuint16_t {
    svsublb_u16(op1, svdup_n_u8(op2))
}
#[doc = "Subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublb[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usublb))]
pub fn svsublb_u32(op1: svuint16_t, op2: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.usublb.nxv4i32")]
        fn _svsublb_u32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svsublb_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublb[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usublb))]
pub fn svsublb_n_u32(op1: svuint16_t, op2: u16) -> svuint32_t {
    svsublb_u32(op1, svdup_n_u16(op2))
}
#[doc = "Subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublb[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usublb))]
pub fn svsublb_u64(op1: svuint32_t, op2: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.usublb.nxv2i64")]
        fn _svsublb_u64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svsublb_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Subtract long (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublb[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usublb))]
pub fn svsublb_n_u64(op1: svuint32_t, op2: u32) -> svuint64_t {
    svsublb_u64(op1, svdup_n_u32(op2))
}
#[doc = "Subtract long (bottom - top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublbt[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssublbt))]
pub fn svsublbt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ssublbt.nxv8i16"
        )]
        fn _svsublbt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svsublbt_s16(op1, op2) }
}
#[doc = "Subtract long (bottom - top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublbt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssublbt))]
pub fn svsublbt_n_s16(op1: svint8_t, op2: i8) -> svint16_t {
    svsublbt_s16(op1, svdup_n_s8(op2))
}
#[doc = "Subtract long (bottom - top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublbt[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssublbt))]
pub fn svsublbt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ssublbt.nxv4i32"
        )]
        fn _svsublbt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svsublbt_s32(op1, op2) }
}
#[doc = "Subtract long (bottom - top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublbt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssublbt))]
pub fn svsublbt_n_s32(op1: svint16_t, op2: i16) -> svint32_t {
    svsublbt_s32(op1, svdup_n_s16(op2))
}
#[doc = "Subtract long (bottom - top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublbt[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssublbt))]
pub fn svsublbt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ssublbt.nxv2i64"
        )]
        fn _svsublbt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svsublbt_s64(op1, op2) }
}
#[doc = "Subtract long (bottom - top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublbt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssublbt))]
pub fn svsublbt_n_s64(op1: svint32_t, op2: i32) -> svint64_t {
    svsublbt_s64(op1, svdup_n_s32(op2))
}
#[doc = "Subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublt[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssublt))]
pub fn svsublt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.ssublt.nxv8i16")]
        fn _svsublt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svsublt_s16(op1, op2) }
}
#[doc = "Subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssublt))]
pub fn svsublt_n_s16(op1: svint8_t, op2: i8) -> svint16_t {
    svsublt_s16(op1, svdup_n_s8(op2))
}
#[doc = "Subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublt[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssublt))]
pub fn svsublt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.ssublt.nxv4i32")]
        fn _svsublt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svsublt_s32(op1, op2) }
}
#[doc = "Subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssublt))]
pub fn svsublt_n_s32(op1: svint16_t, op2: i16) -> svint32_t {
    svsublt_s32(op1, svdup_n_s16(op2))
}
#[doc = "Subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublt[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssublt))]
pub fn svsublt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.ssublt.nxv2i64")]
        fn _svsublt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svsublt_s64(op1, op2) }
}
#[doc = "Subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssublt))]
pub fn svsublt_n_s64(op1: svint32_t, op2: i32) -> svint64_t {
    svsublt_s64(op1, svdup_n_s32(op2))
}
#[doc = "Subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublt[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usublt))]
pub fn svsublt_u16(op1: svuint8_t, op2: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.usublt.nxv8i16")]
        fn _svsublt_u16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svsublt_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublt[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usublt))]
pub fn svsublt_n_u16(op1: svuint8_t, op2: u8) -> svuint16_t {
    svsublt_u16(op1, svdup_n_u8(op2))
}
#[doc = "Subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublt[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usublt))]
pub fn svsublt_u32(op1: svuint16_t, op2: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.usublt.nxv4i32")]
        fn _svsublt_u32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svsublt_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublt[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usublt))]
pub fn svsublt_n_u32(op1: svuint16_t, op2: u16) -> svuint32_t {
    svsublt_u32(op1, svdup_n_u16(op2))
}
#[doc = "Subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublt[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usublt))]
pub fn svsublt_u64(op1: svuint32_t, op2: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.usublt.nxv2i64")]
        fn _svsublt_u64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svsublt_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Subtract long (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsublt[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usublt))]
pub fn svsublt_n_u64(op1: svuint32_t, op2: u32) -> svuint64_t {
    svsublt_u64(op1, svdup_n_u32(op2))
}
#[doc = "Subtract long (top - bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubltb[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssubltb))]
pub fn svsubltb_s16(op1: svint8_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ssubltb.nxv8i16"
        )]
        fn _svsubltb_s16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svsubltb_s16(op1, op2) }
}
#[doc = "Subtract long (top - bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubltb[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssubltb))]
pub fn svsubltb_n_s16(op1: svint8_t, op2: i8) -> svint16_t {
    svsubltb_s16(op1, svdup_n_s8(op2))
}
#[doc = "Subtract long (top - bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubltb[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssubltb))]
pub fn svsubltb_s32(op1: svint16_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ssubltb.nxv4i32"
        )]
        fn _svsubltb_s32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svsubltb_s32(op1, op2) }
}
#[doc = "Subtract long (top - bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubltb[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssubltb))]
pub fn svsubltb_n_s32(op1: svint16_t, op2: i16) -> svint32_t {
    svsubltb_s32(op1, svdup_n_s16(op2))
}
#[doc = "Subtract long (top - bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubltb[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssubltb))]
pub fn svsubltb_s64(op1: svint32_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.ssubltb.nxv2i64"
        )]
        fn _svsubltb_s64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svsubltb_s64(op1, op2) }
}
#[doc = "Subtract long (top - bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubltb[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssubltb))]
pub fn svsubltb_n_s64(op1: svint32_t, op2: i32) -> svint64_t {
    svsubltb_s64(op1, svdup_n_s32(op2))
}
#[doc = "Subtract wide (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubwb[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssubwb))]
pub fn svsubwb_s16(op1: svint16_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.ssubwb.nxv8i16")]
        fn _svsubwb_s16(op1: svint16_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svsubwb_s16(op1, op2) }
}
#[doc = "Subtract wide (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubwb[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssubwb))]
pub fn svsubwb_n_s16(op1: svint16_t, op2: i8) -> svint16_t {
    svsubwb_s16(op1, svdup_n_s8(op2))
}
#[doc = "Subtract wide (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubwb[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssubwb))]
pub fn svsubwb_s32(op1: svint32_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.ssubwb.nxv4i32")]
        fn _svsubwb_s32(op1: svint32_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svsubwb_s32(op1, op2) }
}
#[doc = "Subtract wide (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubwb[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssubwb))]
pub fn svsubwb_n_s32(op1: svint32_t, op2: i16) -> svint32_t {
    svsubwb_s32(op1, svdup_n_s16(op2))
}
#[doc = "Subtract wide (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubwb[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssubwb))]
pub fn svsubwb_s64(op1: svint64_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.ssubwb.nxv2i64")]
        fn _svsubwb_s64(op1: svint64_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svsubwb_s64(op1, op2) }
}
#[doc = "Subtract wide (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubwb[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssubwb))]
pub fn svsubwb_n_s64(op1: svint64_t, op2: i32) -> svint64_t {
    svsubwb_s64(op1, svdup_n_s32(op2))
}
#[doc = "Subtract wide (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubwb[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usubwb))]
pub fn svsubwb_u16(op1: svuint16_t, op2: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.usubwb.nxv8i16")]
        fn _svsubwb_u16(op1: svint16_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svsubwb_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Subtract wide (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubwb[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usubwb))]
pub fn svsubwb_n_u16(op1: svuint16_t, op2: u8) -> svuint16_t {
    svsubwb_u16(op1, svdup_n_u8(op2))
}
#[doc = "Subtract wide (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubwb[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usubwb))]
pub fn svsubwb_u32(op1: svuint32_t, op2: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.usubwb.nxv4i32")]
        fn _svsubwb_u32(op1: svint32_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svsubwb_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Subtract wide (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubwb[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usubwb))]
pub fn svsubwb_n_u32(op1: svuint32_t, op2: u16) -> svuint32_t {
    svsubwb_u32(op1, svdup_n_u16(op2))
}
#[doc = "Subtract wide (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubwb[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usubwb))]
pub fn svsubwb_u64(op1: svuint64_t, op2: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.usubwb.nxv2i64")]
        fn _svsubwb_u64(op1: svint64_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svsubwb_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Subtract wide (bottom)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubwb[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usubwb))]
pub fn svsubwb_n_u64(op1: svuint64_t, op2: u32) -> svuint64_t {
    svsubwb_u64(op1, svdup_n_u32(op2))
}
#[doc = "Subtract wide (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubwt[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssubwt))]
pub fn svsubwt_s16(op1: svint16_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.ssubwt.nxv8i16")]
        fn _svsubwt_s16(op1: svint16_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svsubwt_s16(op1, op2) }
}
#[doc = "Subtract wide (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubwt[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssubwt))]
pub fn svsubwt_n_s16(op1: svint16_t, op2: i8) -> svint16_t {
    svsubwt_s16(op1, svdup_n_s8(op2))
}
#[doc = "Subtract wide (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubwt[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssubwt))]
pub fn svsubwt_s32(op1: svint32_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.ssubwt.nxv4i32")]
        fn _svsubwt_s32(op1: svint32_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svsubwt_s32(op1, op2) }
}
#[doc = "Subtract wide (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubwt[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssubwt))]
pub fn svsubwt_n_s32(op1: svint32_t, op2: i16) -> svint32_t {
    svsubwt_s32(op1, svdup_n_s16(op2))
}
#[doc = "Subtract wide (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubwt[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssubwt))]
pub fn svsubwt_s64(op1: svint64_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.ssubwt.nxv2i64")]
        fn _svsubwt_s64(op1: svint64_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svsubwt_s64(op1, op2) }
}
#[doc = "Subtract wide (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubwt[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(ssubwt))]
pub fn svsubwt_n_s64(op1: svint64_t, op2: i32) -> svint64_t {
    svsubwt_s64(op1, svdup_n_s32(op2))
}
#[doc = "Subtract wide (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubwt[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usubwt))]
pub fn svsubwt_u16(op1: svuint16_t, op2: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.usubwt.nxv8i16")]
        fn _svsubwt_u16(op1: svint16_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svsubwt_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Subtract wide (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubwt[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usubwt))]
pub fn svsubwt_n_u16(op1: svuint16_t, op2: u8) -> svuint16_t {
    svsubwt_u16(op1, svdup_n_u8(op2))
}
#[doc = "Subtract wide (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubwt[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usubwt))]
pub fn svsubwt_u32(op1: svuint32_t, op2: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.usubwt.nxv4i32")]
        fn _svsubwt_u32(op1: svint32_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svsubwt_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Subtract wide (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubwt[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usubwt))]
pub fn svsubwt_n_u32(op1: svuint32_t, op2: u16) -> svuint32_t {
    svsubwt_u32(op1, svdup_n_u16(op2))
}
#[doc = "Subtract wide (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubwt[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usubwt))]
pub fn svsubwt_u64(op1: svuint64_t, op2: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.usubwt.nxv2i64")]
        fn _svsubwt_u64(op1: svint64_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svsubwt_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Subtract wide (top)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svsubwt[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(usubwt))]
pub fn svsubwt_n_u64(op1: svuint64_t, op2: u32) -> svuint64_t {
    svsubwt_u64(op1, svdup_n_u32(op2))
}
#[doc = "Table lookup in two-vector table"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svtbl2[_f32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(tbl))]
pub fn svtbl2_f32(data: svfloat32x2_t, indices: svuint32_t) -> svfloat32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.tbl2.nxv4f32")]
        fn _svtbl2_f32(data0: svfloat32_t, data1: svfloat32_t, indices: svint32_t) -> svfloat32_t;
    }
    unsafe {
        _svtbl2_f32(
            svget2_f32::<0>(data),
            svget2_f32::<1>(data),
            indices.as_signed(),
        )
    }
}
#[doc = "Table lookup in two-vector table"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svtbl2[_f64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(tbl))]
pub fn svtbl2_f64(data: svfloat64x2_t, indices: svuint64_t) -> svfloat64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.tbl2.nxv2f64")]
        fn _svtbl2_f64(data0: svfloat64_t, data1: svfloat64_t, indices: svint64_t) -> svfloat64_t;
    }
    unsafe {
        _svtbl2_f64(
            svget2_f64::<0>(data),
            svget2_f64::<1>(data),
            indices.as_signed(),
        )
    }
}
#[doc = "Table lookup in two-vector table"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svtbl2[_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(tbl))]
pub fn svtbl2_s8(data: svint8x2_t, indices: svuint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.tbl2.nxv16i8")]
        fn _svtbl2_s8(data0: svint8_t, data1: svint8_t, indices: svint8_t) -> svint8_t;
    }
    unsafe {
        _svtbl2_s8(
            svget2_s8::<0>(data),
            svget2_s8::<1>(data),
            indices.as_signed(),
        )
    }
}
#[doc = "Table lookup in two-vector table"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svtbl2[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(tbl))]
pub fn svtbl2_s16(data: svint16x2_t, indices: svuint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.tbl2.nxv8i16")]
        fn _svtbl2_s16(data0: svint16_t, data1: svint16_t, indices: svint16_t) -> svint16_t;
    }
    unsafe {
        _svtbl2_s16(
            svget2_s16::<0>(data),
            svget2_s16::<1>(data),
            indices.as_signed(),
        )
    }
}
#[doc = "Table lookup in two-vector table"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svtbl2[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(tbl))]
pub fn svtbl2_s32(data: svint32x2_t, indices: svuint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.tbl2.nxv4i32")]
        fn _svtbl2_s32(data0: svint32_t, data1: svint32_t, indices: svint32_t) -> svint32_t;
    }
    unsafe {
        _svtbl2_s32(
            svget2_s32::<0>(data),
            svget2_s32::<1>(data),
            indices.as_signed(),
        )
    }
}
#[doc = "Table lookup in two-vector table"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svtbl2[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(tbl))]
pub fn svtbl2_s64(data: svint64x2_t, indices: svuint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.tbl2.nxv2i64")]
        fn _svtbl2_s64(data0: svint64_t, data1: svint64_t, indices: svint64_t) -> svint64_t;
    }
    unsafe {
        _svtbl2_s64(
            svget2_s64::<0>(data),
            svget2_s64::<1>(data),
            indices.as_signed(),
        )
    }
}
#[doc = "Table lookup in two-vector table"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svtbl2[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(tbl))]
pub fn svtbl2_u8(data: svuint8x2_t, indices: svuint8_t) -> svuint8_t {
    unsafe { svtbl2_s8(data.as_signed(), indices).as_unsigned() }
}
#[doc = "Table lookup in two-vector table"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svtbl2[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(tbl))]
pub fn svtbl2_u16(data: svuint16x2_t, indices: svuint16_t) -> svuint16_t {
    unsafe { svtbl2_s16(data.as_signed(), indices).as_unsigned() }
}
#[doc = "Table lookup in two-vector table"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svtbl2[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(tbl))]
pub fn svtbl2_u32(data: svuint32x2_t, indices: svuint32_t) -> svuint32_t {
    unsafe { svtbl2_s32(data.as_signed(), indices).as_unsigned() }
}
#[doc = "Table lookup in two-vector table"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svtbl2[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(tbl))]
pub fn svtbl2_u64(data: svuint64x2_t, indices: svuint64_t) -> svuint64_t {
    unsafe { svtbl2_s64(data.as_signed(), indices).as_unsigned() }
}
#[doc = "Table lookup in single-vector table (merging)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svtbx[_f32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(tbx))]
pub fn svtbx_f32(fallback: svfloat32_t, data: svfloat32_t, indices: svuint32_t) -> svfloat32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.tbx.nxv4f32")]
        fn _svtbx_f32(fallback: svfloat32_t, data: svfloat32_t, indices: svint32_t) -> svfloat32_t;
    }
    unsafe { _svtbx_f32(fallback, data, indices.as_signed()) }
}
#[doc = "Table lookup in single-vector table (merging)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svtbx[_f64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(tbx))]
pub fn svtbx_f64(fallback: svfloat64_t, data: svfloat64_t, indices: svuint64_t) -> svfloat64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.tbx.nxv2f64")]
        fn _svtbx_f64(fallback: svfloat64_t, data: svfloat64_t, indices: svint64_t) -> svfloat64_t;
    }
    unsafe { _svtbx_f64(fallback, data, indices.as_signed()) }
}
#[doc = "Table lookup in single-vector table (merging)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svtbx[_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(tbx))]
pub fn svtbx_s8(fallback: svint8_t, data: svint8_t, indices: svuint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.tbx.nxv16i8")]
        fn _svtbx_s8(fallback: svint8_t, data: svint8_t, indices: svint8_t) -> svint8_t;
    }
    unsafe { _svtbx_s8(fallback, data, indices.as_signed()) }
}
#[doc = "Table lookup in single-vector table (merging)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svtbx[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(tbx))]
pub fn svtbx_s16(fallback: svint16_t, data: svint16_t, indices: svuint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.tbx.nxv8i16")]
        fn _svtbx_s16(fallback: svint16_t, data: svint16_t, indices: svint16_t) -> svint16_t;
    }
    unsafe { _svtbx_s16(fallback, data, indices.as_signed()) }
}
#[doc = "Table lookup in single-vector table (merging)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svtbx[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(tbx))]
pub fn svtbx_s32(fallback: svint32_t, data: svint32_t, indices: svuint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.tbx.nxv4i32")]
        fn _svtbx_s32(fallback: svint32_t, data: svint32_t, indices: svint32_t) -> svint32_t;
    }
    unsafe { _svtbx_s32(fallback, data, indices.as_signed()) }
}
#[doc = "Table lookup in single-vector table (merging)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svtbx[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(tbx))]
pub fn svtbx_s64(fallback: svint64_t, data: svint64_t, indices: svuint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.tbx.nxv2i64")]
        fn _svtbx_s64(fallback: svint64_t, data: svint64_t, indices: svint64_t) -> svint64_t;
    }
    unsafe { _svtbx_s64(fallback, data, indices.as_signed()) }
}
#[doc = "Table lookup in single-vector table (merging)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svtbx[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(tbx))]
pub fn svtbx_u8(fallback: svuint8_t, data: svuint8_t, indices: svuint8_t) -> svuint8_t {
    unsafe { svtbx_s8(fallback.as_signed(), data.as_signed(), indices).as_unsigned() }
}
#[doc = "Table lookup in single-vector table (merging)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svtbx[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(tbx))]
pub fn svtbx_u16(fallback: svuint16_t, data: svuint16_t, indices: svuint16_t) -> svuint16_t {
    unsafe { svtbx_s16(fallback.as_signed(), data.as_signed(), indices).as_unsigned() }
}
#[doc = "Table lookup in single-vector table (merging)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svtbx[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(tbx))]
pub fn svtbx_u32(fallback: svuint32_t, data: svuint32_t, indices: svuint32_t) -> svuint32_t {
    unsafe { svtbx_s32(fallback.as_signed(), data.as_signed(), indices).as_unsigned() }
}
#[doc = "Table lookup in single-vector table (merging)"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svtbx[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(tbx))]
pub fn svtbx_u64(fallback: svuint64_t, data: svuint64_t, indices: svuint64_t) -> svuint64_t {
    unsafe { svtbx_s64(fallback.as_signed(), data.as_signed(), indices).as_unsigned() }
}
#[doc = "Unpack and extend high half"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svunpkhi[_b])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(punpkhi))]
pub fn svunpkhi_b(op: svbool_t) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.punpkhi.nxv16i1"
        )]
        fn _svunpkhi_b(op: svbool_t) -> svbool8_t;
    }
    unsafe { _svunpkhi_b(op).into() }
}
#[doc = "Unpack and extend high half"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svunpkhi[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sunpkhi))]
pub fn svunpkhi_s16(op: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sunpkhi.nxv8i16"
        )]
        fn _svunpkhi_s16(op: svint8_t) -> svint16_t;
    }
    unsafe { _svunpkhi_s16(op) }
}
#[doc = "Unpack and extend high half"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svunpkhi[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sunpkhi))]
pub fn svunpkhi_s32(op: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sunpkhi.nxv4i32"
        )]
        fn _svunpkhi_s32(op: svint16_t) -> svint32_t;
    }
    unsafe { _svunpkhi_s32(op) }
}
#[doc = "Unpack and extend high half"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svunpkhi[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sunpkhi))]
pub fn svunpkhi_s64(op: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sunpkhi.nxv2i64"
        )]
        fn _svunpkhi_s64(op: svint32_t) -> svint64_t;
    }
    unsafe { _svunpkhi_s64(op) }
}
#[doc = "Unpack and extend high half"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svunpkhi[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uunpkhi))]
pub fn svunpkhi_u16(op: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.uunpkhi.nxv8i16"
        )]
        fn _svunpkhi_u16(op: svint8_t) -> svint16_t;
    }
    unsafe { _svunpkhi_u16(op.as_signed()).as_unsigned() }
}
#[doc = "Unpack and extend high half"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svunpkhi[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uunpkhi))]
pub fn svunpkhi_u32(op: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.uunpkhi.nxv4i32"
        )]
        fn _svunpkhi_u32(op: svint16_t) -> svint32_t;
    }
    unsafe { _svunpkhi_u32(op.as_signed()).as_unsigned() }
}
#[doc = "Unpack and extend high half"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svunpkhi[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uunpkhi))]
pub fn svunpkhi_u64(op: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.uunpkhi.nxv2i64"
        )]
        fn _svunpkhi_u64(op: svint32_t) -> svint64_t;
    }
    unsafe { _svunpkhi_u64(op.as_signed()).as_unsigned() }
}
#[doc = "Unpack and extend low half"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svunpklo[_b])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(punpklo))]
pub fn svunpklo_b(op: svbool_t) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.punpklo.nxv16i1"
        )]
        fn _svunpklo_b(op: svbool_t) -> svbool8_t;
    }
    unsafe { _svunpklo_b(op).into() }
}
#[doc = "Unpack and extend low half"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svunpklo[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sunpklo))]
pub fn svunpklo_s16(op: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sunpklo.nxv8i16"
        )]
        fn _svunpklo_s16(op: svint8_t) -> svint16_t;
    }
    unsafe { _svunpklo_s16(op) }
}
#[doc = "Unpack and extend low half"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svunpklo[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sunpklo))]
pub fn svunpklo_s32(op: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sunpklo.nxv4i32"
        )]
        fn _svunpklo_s32(op: svint16_t) -> svint32_t;
    }
    unsafe { _svunpklo_s32(op) }
}
#[doc = "Unpack and extend low half"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svunpklo[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(sunpklo))]
pub fn svunpklo_s64(op: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.sunpklo.nxv2i64"
        )]
        fn _svunpklo_s64(op: svint32_t) -> svint64_t;
    }
    unsafe { _svunpklo_s64(op) }
}
#[doc = "Unpack and extend low half"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svunpklo[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uunpklo))]
pub fn svunpklo_u16(op: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.uunpklo.nxv8i16"
        )]
        fn _svunpklo_u16(op: svint8_t) -> svint16_t;
    }
    unsafe { _svunpklo_u16(op.as_signed()).as_unsigned() }
}
#[doc = "Unpack and extend low half"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svunpklo[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uunpklo))]
pub fn svunpklo_u32(op: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.uunpklo.nxv4i32"
        )]
        fn _svunpklo_u32(op: svint16_t) -> svint32_t;
    }
    unsafe { _svunpklo_u32(op.as_signed()).as_unsigned() }
}
#[doc = "Unpack and extend low half"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svunpklo[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(uunpklo))]
pub fn svunpklo_u64(op: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.uunpklo.nxv2i64"
        )]
        fn _svunpklo_u64(op: svint32_t) -> svint64_t;
    }
    unsafe { _svunpklo_u64(op.as_signed()).as_unsigned() }
}
#[doc = "Saturating add with unsigned addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svuqadd[_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(suqadd))]
pub fn svuqadd_s8_m(pg: svbool_t, op1: svint8_t, op2: svuint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.suqadd.nxv16i8")]
        fn _svuqadd_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svuqadd_s8_m(pg, op1, op2.as_signed()) }
}
#[doc = "Saturating add with unsigned addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svuqadd[_n_s8]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(suqadd))]
pub fn svuqadd_n_s8_m(pg: svbool_t, op1: svint8_t, op2: u8) -> svint8_t {
    svuqadd_s8_m(pg, op1, svdup_n_u8(op2))
}
#[doc = "Saturating add with unsigned addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svuqadd[_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(suqadd))]
pub fn svuqadd_s8_x(pg: svbool_t, op1: svint8_t, op2: svuint8_t) -> svint8_t {
    svuqadd_s8_m(pg, op1, op2)
}
#[doc = "Saturating add with unsigned addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svuqadd[_n_s8]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(suqadd))]
pub fn svuqadd_n_s8_x(pg: svbool_t, op1: svint8_t, op2: u8) -> svint8_t {
    svuqadd_s8_x(pg, op1, svdup_n_u8(op2))
}
#[doc = "Saturating add with unsigned addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svuqadd[_s8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(suqadd))]
pub fn svuqadd_s8_z(pg: svbool_t, op1: svint8_t, op2: svuint8_t) -> svint8_t {
    svuqadd_s8_m(pg, svsel_s8(pg, op1, svdup_n_s8(0)), op2)
}
#[doc = "Saturating add with unsigned addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svuqadd[_n_s8]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(suqadd))]
pub fn svuqadd_n_s8_z(pg: svbool_t, op1: svint8_t, op2: u8) -> svint8_t {
    svuqadd_s8_z(pg, op1, svdup_n_u8(op2))
}
#[doc = "Saturating add with unsigned addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svuqadd[_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(suqadd))]
pub fn svuqadd_s16_m(pg: svbool_t, op1: svint16_t, op2: svuint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.suqadd.nxv8i16")]
        fn _svuqadd_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svuqadd_s16_m(pg.into(), op1, op2.as_signed()) }
}
#[doc = "Saturating add with unsigned addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svuqadd[_n_s16]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(suqadd))]
pub fn svuqadd_n_s16_m(pg: svbool_t, op1: svint16_t, op2: u16) -> svint16_t {
    svuqadd_s16_m(pg, op1, svdup_n_u16(op2))
}
#[doc = "Saturating add with unsigned addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svuqadd[_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(suqadd))]
pub fn svuqadd_s16_x(pg: svbool_t, op1: svint16_t, op2: svuint16_t) -> svint16_t {
    svuqadd_s16_m(pg, op1, op2)
}
#[doc = "Saturating add with unsigned addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svuqadd[_n_s16]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(suqadd))]
pub fn svuqadd_n_s16_x(pg: svbool_t, op1: svint16_t, op2: u16) -> svint16_t {
    svuqadd_s16_x(pg, op1, svdup_n_u16(op2))
}
#[doc = "Saturating add with unsigned addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svuqadd[_s16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(suqadd))]
pub fn svuqadd_s16_z(pg: svbool_t, op1: svint16_t, op2: svuint16_t) -> svint16_t {
    svuqadd_s16_m(pg, svsel_s16(pg, op1, svdup_n_s16(0)), op2)
}
#[doc = "Saturating add with unsigned addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svuqadd[_n_s16]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(suqadd))]
pub fn svuqadd_n_s16_z(pg: svbool_t, op1: svint16_t, op2: u16) -> svint16_t {
    svuqadd_s16_z(pg, op1, svdup_n_u16(op2))
}
#[doc = "Saturating add with unsigned addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svuqadd[_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(suqadd))]
pub fn svuqadd_s32_m(pg: svbool_t, op1: svint32_t, op2: svuint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.suqadd.nxv4i32")]
        fn _svuqadd_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svuqadd_s32_m(pg.into(), op1, op2.as_signed()) }
}
#[doc = "Saturating add with unsigned addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svuqadd[_n_s32]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(suqadd))]
pub fn svuqadd_n_s32_m(pg: svbool_t, op1: svint32_t, op2: u32) -> svint32_t {
    svuqadd_s32_m(pg, op1, svdup_n_u32(op2))
}
#[doc = "Saturating add with unsigned addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svuqadd[_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(suqadd))]
pub fn svuqadd_s32_x(pg: svbool_t, op1: svint32_t, op2: svuint32_t) -> svint32_t {
    svuqadd_s32_m(pg, op1, op2)
}
#[doc = "Saturating add with unsigned addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svuqadd[_n_s32]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(suqadd))]
pub fn svuqadd_n_s32_x(pg: svbool_t, op1: svint32_t, op2: u32) -> svint32_t {
    svuqadd_s32_x(pg, op1, svdup_n_u32(op2))
}
#[doc = "Saturating add with unsigned addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svuqadd[_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(suqadd))]
pub fn svuqadd_s32_z(pg: svbool_t, op1: svint32_t, op2: svuint32_t) -> svint32_t {
    svuqadd_s32_m(pg, svsel_s32(pg, op1, svdup_n_s32(0)), op2)
}
#[doc = "Saturating add with unsigned addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svuqadd[_n_s32]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(suqadd))]
pub fn svuqadd_n_s32_z(pg: svbool_t, op1: svint32_t, op2: u32) -> svint32_t {
    svuqadd_s32_z(pg, op1, svdup_n_u32(op2))
}
#[doc = "Saturating add with unsigned addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svuqadd[_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(suqadd))]
pub fn svuqadd_s64_m(pg: svbool_t, op1: svint64_t, op2: svuint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.suqadd.nxv2i64")]
        fn _svuqadd_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svuqadd_s64_m(pg.into(), op1, op2.as_signed()) }
}
#[doc = "Saturating add with unsigned addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svuqadd[_n_s64]_m)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(suqadd))]
pub fn svuqadd_n_s64_m(pg: svbool_t, op1: svint64_t, op2: u64) -> svint64_t {
    svuqadd_s64_m(pg, op1, svdup_n_u64(op2))
}
#[doc = "Saturating add with unsigned addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svuqadd[_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(suqadd))]
pub fn svuqadd_s64_x(pg: svbool_t, op1: svint64_t, op2: svuint64_t) -> svint64_t {
    svuqadd_s64_m(pg, op1, op2)
}
#[doc = "Saturating add with unsigned addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svuqadd[_n_s64]_x)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(suqadd))]
pub fn svuqadd_n_s64_x(pg: svbool_t, op1: svint64_t, op2: u64) -> svint64_t {
    svuqadd_s64_x(pg, op1, svdup_n_u64(op2))
}
#[doc = "Saturating add with unsigned addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svuqadd[_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(suqadd))]
pub fn svuqadd_s64_z(pg: svbool_t, op1: svint64_t, op2: svuint64_t) -> svint64_t {
    svuqadd_s64_m(pg, svsel_s64(pg, op1, svdup_n_s64(0)), op2)
}
#[doc = "Saturating add with unsigned addend"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svuqadd[_n_s64]_z)"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(suqadd))]
pub fn svuqadd_n_s64_z(pg: svbool_t, op1: svint64_t, op2: u64) -> svint64_t {
    svuqadd_s64_z(pg, op1, svdup_n_u64(op2))
}
#[doc = "While decrementing scalar is greater than or equal to"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilege_b8[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilege))]
pub fn svwhilege_b8_s32(op1: i32, op2: i32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilege.nxv16i1.i32"
        )]
        fn _svwhilege_b8_s32(op1: i32, op2: i32) -> svbool_t;
    }
    unsafe { _svwhilege_b8_s32(op1, op2) }
}
#[doc = "While decrementing scalar is greater than or equal to"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilege_b16[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilege))]
pub fn svwhilege_b16_s32(op1: i32, op2: i32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilege.nxv8i1.i32"
        )]
        fn _svwhilege_b16_s32(op1: i32, op2: i32) -> svbool8_t;
    }
    unsafe { _svwhilege_b16_s32(op1, op2).into() }
}
#[doc = "While decrementing scalar is greater than or equal to"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilege_b32[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilege))]
pub fn svwhilege_b32_s32(op1: i32, op2: i32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilege.nxv4i1.i32"
        )]
        fn _svwhilege_b32_s32(op1: i32, op2: i32) -> svbool4_t;
    }
    unsafe { _svwhilege_b32_s32(op1, op2).into() }
}
#[doc = "While decrementing scalar is greater than or equal to"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilege_b64[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilege))]
pub fn svwhilege_b64_s32(op1: i32, op2: i32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilege.nxv2i1.i32"
        )]
        fn _svwhilege_b64_s32(op1: i32, op2: i32) -> svbool2_t;
    }
    unsafe { _svwhilege_b64_s32(op1, op2).into() }
}
#[doc = "While decrementing scalar is greater than or equal to"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilege_b8[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilege))]
pub fn svwhilege_b8_s64(op1: i64, op2: i64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilege.nxv16i1.i64"
        )]
        fn _svwhilege_b8_s64(op1: i64, op2: i64) -> svbool_t;
    }
    unsafe { _svwhilege_b8_s64(op1, op2) }
}
#[doc = "While decrementing scalar is greater than or equal to"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilege_b16[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilege))]
pub fn svwhilege_b16_s64(op1: i64, op2: i64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilege.nxv8i1.i64"
        )]
        fn _svwhilege_b16_s64(op1: i64, op2: i64) -> svbool8_t;
    }
    unsafe { _svwhilege_b16_s64(op1, op2).into() }
}
#[doc = "While decrementing scalar is greater than or equal to"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilege_b32[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilege))]
pub fn svwhilege_b32_s64(op1: i64, op2: i64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilege.nxv4i1.i64"
        )]
        fn _svwhilege_b32_s64(op1: i64, op2: i64) -> svbool4_t;
    }
    unsafe { _svwhilege_b32_s64(op1, op2).into() }
}
#[doc = "While decrementing scalar is greater than or equal to"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilege_b64[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilege))]
pub fn svwhilege_b64_s64(op1: i64, op2: i64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilege.nxv2i1.i64"
        )]
        fn _svwhilege_b64_s64(op1: i64, op2: i64) -> svbool2_t;
    }
    unsafe { _svwhilege_b64_s64(op1, op2).into() }
}
#[doc = "While decrementing scalar is greater than or equal to"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilege_b8[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilehs))]
pub fn svwhilege_b8_u32(op1: u32, op2: u32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilehs.nxv16i1.i32"
        )]
        fn _svwhilege_b8_u32(op1: i32, op2: i32) -> svbool_t;
    }
    unsafe { _svwhilege_b8_u32(op1.as_signed(), op2.as_signed()) }
}
#[doc = "While decrementing scalar is greater than or equal to"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilege_b16[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilehs))]
pub fn svwhilege_b16_u32(op1: u32, op2: u32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilehs.nxv8i1.i32"
        )]
        fn _svwhilege_b16_u32(op1: i32, op2: i32) -> svbool8_t;
    }
    unsafe { _svwhilege_b16_u32(op1.as_signed(), op2.as_signed()).into() }
}
#[doc = "While decrementing scalar is greater than or equal to"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilege_b32[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilehs))]
pub fn svwhilege_b32_u32(op1: u32, op2: u32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilehs.nxv4i1.i32"
        )]
        fn _svwhilege_b32_u32(op1: i32, op2: i32) -> svbool4_t;
    }
    unsafe { _svwhilege_b32_u32(op1.as_signed(), op2.as_signed()).into() }
}
#[doc = "While decrementing scalar is greater than or equal to"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilege_b64[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilehs))]
pub fn svwhilege_b64_u32(op1: u32, op2: u32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilehs.nxv2i1.i32"
        )]
        fn _svwhilege_b64_u32(op1: i32, op2: i32) -> svbool2_t;
    }
    unsafe { _svwhilege_b64_u32(op1.as_signed(), op2.as_signed()).into() }
}
#[doc = "While decrementing scalar is greater than or equal to"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilege_b8[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilehs))]
pub fn svwhilege_b8_u64(op1: u64, op2: u64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilehs.nxv16i1.i64"
        )]
        fn _svwhilege_b8_u64(op1: i64, op2: i64) -> svbool_t;
    }
    unsafe { _svwhilege_b8_u64(op1.as_signed(), op2.as_signed()) }
}
#[doc = "While decrementing scalar is greater than or equal to"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilege_b16[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilehs))]
pub fn svwhilege_b16_u64(op1: u64, op2: u64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilehs.nxv8i1.i64"
        )]
        fn _svwhilege_b16_u64(op1: i64, op2: i64) -> svbool8_t;
    }
    unsafe { _svwhilege_b16_u64(op1.as_signed(), op2.as_signed()).into() }
}
#[doc = "While decrementing scalar is greater than or equal to"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilege_b32[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilehs))]
pub fn svwhilege_b32_u64(op1: u64, op2: u64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilehs.nxv4i1.i64"
        )]
        fn _svwhilege_b32_u64(op1: i64, op2: i64) -> svbool4_t;
    }
    unsafe { _svwhilege_b32_u64(op1.as_signed(), op2.as_signed()).into() }
}
#[doc = "While decrementing scalar is greater than or equal to"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilege_b64[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilehs))]
pub fn svwhilege_b64_u64(op1: u64, op2: u64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilehs.nxv2i1.i64"
        )]
        fn _svwhilege_b64_u64(op1: i64, op2: i64) -> svbool2_t;
    }
    unsafe { _svwhilege_b64_u64(op1.as_signed(), op2.as_signed()).into() }
}
#[doc = "While decrementing scalar is greater than"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilegt_b8[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilegt))]
pub fn svwhilegt_b8_s32(op1: i32, op2: i32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilegt.nxv16i1.i32"
        )]
        fn _svwhilegt_b8_s32(op1: i32, op2: i32) -> svbool_t;
    }
    unsafe { _svwhilegt_b8_s32(op1, op2) }
}
#[doc = "While decrementing scalar is greater than"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilegt_b16[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilegt))]
pub fn svwhilegt_b16_s32(op1: i32, op2: i32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilegt.nxv8i1.i32"
        )]
        fn _svwhilegt_b16_s32(op1: i32, op2: i32) -> svbool8_t;
    }
    unsafe { _svwhilegt_b16_s32(op1, op2).into() }
}
#[doc = "While decrementing scalar is greater than"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilegt_b32[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilegt))]
pub fn svwhilegt_b32_s32(op1: i32, op2: i32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilegt.nxv4i1.i32"
        )]
        fn _svwhilegt_b32_s32(op1: i32, op2: i32) -> svbool4_t;
    }
    unsafe { _svwhilegt_b32_s32(op1, op2).into() }
}
#[doc = "While decrementing scalar is greater than"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilegt_b64[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilegt))]
pub fn svwhilegt_b64_s32(op1: i32, op2: i32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilegt.nxv2i1.i32"
        )]
        fn _svwhilegt_b64_s32(op1: i32, op2: i32) -> svbool2_t;
    }
    unsafe { _svwhilegt_b64_s32(op1, op2).into() }
}
#[doc = "While decrementing scalar is greater than"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilegt_b8[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilegt))]
pub fn svwhilegt_b8_s64(op1: i64, op2: i64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilegt.nxv16i1.i64"
        )]
        fn _svwhilegt_b8_s64(op1: i64, op2: i64) -> svbool_t;
    }
    unsafe { _svwhilegt_b8_s64(op1, op2) }
}
#[doc = "While decrementing scalar is greater than"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilegt_b16[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilegt))]
pub fn svwhilegt_b16_s64(op1: i64, op2: i64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilegt.nxv8i1.i64"
        )]
        fn _svwhilegt_b16_s64(op1: i64, op2: i64) -> svbool8_t;
    }
    unsafe { _svwhilegt_b16_s64(op1, op2).into() }
}
#[doc = "While decrementing scalar is greater than"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilegt_b32[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilegt))]
pub fn svwhilegt_b32_s64(op1: i64, op2: i64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilegt.nxv4i1.i64"
        )]
        fn _svwhilegt_b32_s64(op1: i64, op2: i64) -> svbool4_t;
    }
    unsafe { _svwhilegt_b32_s64(op1, op2).into() }
}
#[doc = "While decrementing scalar is greater than"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilegt_b64[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilegt))]
pub fn svwhilegt_b64_s64(op1: i64, op2: i64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilegt.nxv2i1.i64"
        )]
        fn _svwhilegt_b64_s64(op1: i64, op2: i64) -> svbool2_t;
    }
    unsafe { _svwhilegt_b64_s64(op1, op2).into() }
}
#[doc = "While decrementing scalar is greater than"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilegt_b8[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilehi))]
pub fn svwhilegt_b8_u32(op1: u32, op2: u32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilehi.nxv16i1.i32"
        )]
        fn _svwhilegt_b8_u32(op1: i32, op2: i32) -> svbool_t;
    }
    unsafe { _svwhilegt_b8_u32(op1.as_signed(), op2.as_signed()) }
}
#[doc = "While decrementing scalar is greater than"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilegt_b16[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilehi))]
pub fn svwhilegt_b16_u32(op1: u32, op2: u32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilehi.nxv8i1.i32"
        )]
        fn _svwhilegt_b16_u32(op1: i32, op2: i32) -> svbool8_t;
    }
    unsafe { _svwhilegt_b16_u32(op1.as_signed(), op2.as_signed()).into() }
}
#[doc = "While decrementing scalar is greater than"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilegt_b32[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilehi))]
pub fn svwhilegt_b32_u32(op1: u32, op2: u32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilehi.nxv4i1.i32"
        )]
        fn _svwhilegt_b32_u32(op1: i32, op2: i32) -> svbool4_t;
    }
    unsafe { _svwhilegt_b32_u32(op1.as_signed(), op2.as_signed()).into() }
}
#[doc = "While decrementing scalar is greater than"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilegt_b64[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilehi))]
pub fn svwhilegt_b64_u32(op1: u32, op2: u32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilehi.nxv2i1.i32"
        )]
        fn _svwhilegt_b64_u32(op1: i32, op2: i32) -> svbool2_t;
    }
    unsafe { _svwhilegt_b64_u32(op1.as_signed(), op2.as_signed()).into() }
}
#[doc = "While decrementing scalar is greater than"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilegt_b8[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilehi))]
pub fn svwhilegt_b8_u64(op1: u64, op2: u64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilehi.nxv16i1.i64"
        )]
        fn _svwhilegt_b8_u64(op1: i64, op2: i64) -> svbool_t;
    }
    unsafe { _svwhilegt_b8_u64(op1.as_signed(), op2.as_signed()) }
}
#[doc = "While decrementing scalar is greater than"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilegt_b16[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilehi))]
pub fn svwhilegt_b16_u64(op1: u64, op2: u64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilehi.nxv8i1.i64"
        )]
        fn _svwhilegt_b16_u64(op1: i64, op2: i64) -> svbool8_t;
    }
    unsafe { _svwhilegt_b16_u64(op1.as_signed(), op2.as_signed()).into() }
}
#[doc = "While decrementing scalar is greater than"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilegt_b32[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilehi))]
pub fn svwhilegt_b32_u64(op1: u64, op2: u64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilehi.nxv4i1.i64"
        )]
        fn _svwhilegt_b32_u64(op1: i64, op2: i64) -> svbool4_t;
    }
    unsafe { _svwhilegt_b32_u64(op1.as_signed(), op2.as_signed()).into() }
}
#[doc = "While decrementing scalar is greater than"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilegt_b64[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilehi))]
pub fn svwhilegt_b64_u64(op1: u64, op2: u64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilehi.nxv2i1.i64"
        )]
        fn _svwhilegt_b64_u64(op1: i64, op2: i64) -> svbool2_t;
    }
    unsafe { _svwhilegt_b64_u64(op1.as_signed(), op2.as_signed()).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
unsafe fn svwhilerw_8ptr<T>(op1: *const T, op2: *const T) -> svbool_t {
    let op1 = op1 as *const crate::ffi::c_void;
    let op2 = op2 as *const crate::ffi::c_void;
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilerw.b.nxv16i1.p0"
        )]
        fn _svwhilerw_8ptr(
            op1: *const crate::ffi::c_void,
            op2: *const crate::ffi::c_void,
        ) -> svbool_t;
    }
    _svwhilerw_8ptr(op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
unsafe fn svwhilerw_16ptr<T>(op1: *const T, op2: *const T) -> svbool_t {
    let op1 = op1 as *const crate::ffi::c_void;
    let op2 = op2 as *const crate::ffi::c_void;
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilerw.h.nxv8i1.p0"
        )]
        fn _svwhilerw_16ptr(
            op1: *const crate::ffi::c_void,
            op2: *const crate::ffi::c_void,
        ) -> svbool8_t;
    }
    _svwhilerw_16ptr(op1, op2).into()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
unsafe fn svwhilerw_32ptr<T>(op1: *const T, op2: *const T) -> svbool_t {
    let op1 = op1 as *const crate::ffi::c_void;
    let op2 = op2 as *const crate::ffi::c_void;
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilerw.s.nxv4i1.p0"
        )]
        fn _svwhilerw_32ptr(
            op1: *const crate::ffi::c_void,
            op2: *const crate::ffi::c_void,
        ) -> svbool4_t;
    }
    _svwhilerw_32ptr(op1, op2).into()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
unsafe fn svwhilerw_64ptr<T>(op1: *const T, op2: *const T) -> svbool_t {
    let op1 = op1 as *const crate::ffi::c_void;
    let op2 = op2 as *const crate::ffi::c_void;
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilerw.d.nxv2i1.p0"
        )]
        fn _svwhilerw_64ptr(
            op1: *const crate::ffi::c_void,
            op2: *const crate::ffi::c_void,
        ) -> svbool2_t;
    }
    _svwhilerw_64ptr(op1, op2).into()
}
#[doc = "While free of read-after-write conflicts"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilerw[_f32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilerw))]
pub unsafe fn svwhilerw_f32(op1: *const f32, op2: *const f32) -> svbool_t {
    svwhilerw_32ptr::<f32>(op1, op2)
}
#[doc = "While free of read-after-write conflicts"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilerw[_f64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilerw))]
pub unsafe fn svwhilerw_f64(op1: *const f64, op2: *const f64) -> svbool_t {
    svwhilerw_64ptr::<f64>(op1, op2)
}
#[doc = "While free of read-after-write conflicts"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilerw[_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilerw))]
pub unsafe fn svwhilerw_s8(op1: *const i8, op2: *const i8) -> svbool_t {
    svwhilerw_8ptr::<i8>(op1, op2)
}
#[doc = "While free of read-after-write conflicts"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilerw[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilerw))]
pub unsafe fn svwhilerw_s16(op1: *const i16, op2: *const i16) -> svbool_t {
    svwhilerw_16ptr::<i16>(op1, op2)
}
#[doc = "While free of read-after-write conflicts"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilerw[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilerw))]
pub unsafe fn svwhilerw_s32(op1: *const i32, op2: *const i32) -> svbool_t {
    svwhilerw_32ptr::<i32>(op1, op2)
}
#[doc = "While free of read-after-write conflicts"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilerw[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilerw))]
pub unsafe fn svwhilerw_s64(op1: *const i64, op2: *const i64) -> svbool_t {
    svwhilerw_64ptr::<i64>(op1, op2)
}
#[doc = "While free of read-after-write conflicts"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilerw[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilerw))]
pub unsafe fn svwhilerw_u8(op1: *const u8, op2: *const u8) -> svbool_t {
    svwhilerw_8ptr::<u8>(op1, op2)
}
#[doc = "While free of read-after-write conflicts"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilerw[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilerw))]
pub unsafe fn svwhilerw_u16(op1: *const u16, op2: *const u16) -> svbool_t {
    svwhilerw_16ptr::<u16>(op1, op2)
}
#[doc = "While free of read-after-write conflicts"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilerw[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilerw))]
pub unsafe fn svwhilerw_u32(op1: *const u32, op2: *const u32) -> svbool_t {
    svwhilerw_32ptr::<u32>(op1, op2)
}
#[doc = "While free of read-after-write conflicts"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilerw[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilerw))]
pub unsafe fn svwhilerw_u64(op1: *const u64, op2: *const u64) -> svbool_t {
    svwhilerw_64ptr::<u64>(op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
unsafe fn svwhilewr_8ptr<T>(op1: *const T, op2: *const T) -> svbool_t {
    let op1 = op1 as *const crate::ffi::c_void;
    let op2 = op2 as *const crate::ffi::c_void;
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilewr.b.nxv16i1.p0"
        )]
        fn _svwhilewr_8ptr(
            op1: *const crate::ffi::c_void,
            op2: *const crate::ffi::c_void,
        ) -> svbool_t;
    }
    _svwhilewr_8ptr(op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
unsafe fn svwhilewr_16ptr<T>(op1: *const T, op2: *const T) -> svbool_t {
    let op1 = op1 as *const crate::ffi::c_void;
    let op2 = op2 as *const crate::ffi::c_void;
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilewr.h.nxv8i1.p0"
        )]
        fn _svwhilewr_16ptr(
            op1: *const crate::ffi::c_void,
            op2: *const crate::ffi::c_void,
        ) -> svbool8_t;
    }
    _svwhilewr_16ptr(op1, op2).into()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
unsafe fn svwhilewr_32ptr<T>(op1: *const T, op2: *const T) -> svbool_t {
    let op1 = op1 as *const crate::ffi::c_void;
    let op2 = op2 as *const crate::ffi::c_void;
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilewr.s.nxv4i1.p0"
        )]
        fn _svwhilewr_32ptr(
            op1: *const crate::ffi::c_void,
            op2: *const crate::ffi::c_void,
        ) -> svbool4_t;
    }
    _svwhilewr_32ptr(op1, op2).into()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
unsafe fn svwhilewr_64ptr<T>(op1: *const T, op2: *const T) -> svbool_t {
    let op1 = op1 as *const crate::ffi::c_void;
    let op2 = op2 as *const crate::ffi::c_void;
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            link_name = "llvm.aarch64.sve.whilewr.d.nxv2i1.p0"
        )]
        fn _svwhilewr_64ptr(
            op1: *const crate::ffi::c_void,
            op2: *const crate::ffi::c_void,
        ) -> svbool2_t;
    }
    _svwhilewr_64ptr(op1, op2).into()
}
#[doc = "While free of write-after-read conflicts"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilewr[_f32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilewr))]
pub unsafe fn svwhilewr_f32(op1: *const f32, op2: *const f32) -> svbool_t {
    svwhilewr_32ptr::<f32>(op1, op2)
}
#[doc = "While free of write-after-read conflicts"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilewr[_f64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilewr))]
pub unsafe fn svwhilewr_f64(op1: *const f64, op2: *const f64) -> svbool_t {
    svwhilewr_64ptr::<f64>(op1, op2)
}
#[doc = "While free of write-after-read conflicts"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilewr[_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilewr))]
pub unsafe fn svwhilewr_s8(op1: *const i8, op2: *const i8) -> svbool_t {
    svwhilewr_8ptr::<i8>(op1, op2)
}
#[doc = "While free of write-after-read conflicts"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilewr[_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilewr))]
pub unsafe fn svwhilewr_s16(op1: *const i16, op2: *const i16) -> svbool_t {
    svwhilewr_16ptr::<i16>(op1, op2)
}
#[doc = "While free of write-after-read conflicts"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilewr[_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilewr))]
pub unsafe fn svwhilewr_s32(op1: *const i32, op2: *const i32) -> svbool_t {
    svwhilewr_32ptr::<i32>(op1, op2)
}
#[doc = "While free of write-after-read conflicts"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilewr[_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilewr))]
pub unsafe fn svwhilewr_s64(op1: *const i64, op2: *const i64) -> svbool_t {
    svwhilewr_64ptr::<i64>(op1, op2)
}
#[doc = "While free of write-after-read conflicts"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilewr[_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilewr))]
pub unsafe fn svwhilewr_u8(op1: *const u8, op2: *const u8) -> svbool_t {
    svwhilewr_8ptr::<u8>(op1, op2)
}
#[doc = "While free of write-after-read conflicts"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilewr[_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilewr))]
pub unsafe fn svwhilewr_u16(op1: *const u16, op2: *const u16) -> svbool_t {
    svwhilewr_16ptr::<u16>(op1, op2)
}
#[doc = "While free of write-after-read conflicts"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilewr[_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilewr))]
pub unsafe fn svwhilewr_u32(op1: *const u32, op2: *const u32) -> svbool_t {
    svwhilewr_32ptr::<u32>(op1, op2)
}
#[doc = "While free of write-after-read conflicts"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svwhilewr[_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(whilewr))]
pub unsafe fn svwhilewr_u64(op1: *const u64, op2: *const u64) -> svbool_t {
    svwhilewr_64ptr::<u64>(op1, op2)
}
#[doc = "Bitwise exclusive OR and rotate right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svxar[_n_s8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(xar, IMM3 = 1))]
pub fn svxar_n_s8<const IMM3: i32>(op1: svint8_t, op2: svint8_t) -> svint8_t {
    static_assert_range!(IMM3, 1, 8);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.xar.nxv16i8")]
        fn _svxar_n_s8(op1: svint8_t, op2: svint8_t, imm3: i32) -> svint8_t;
    }
    unsafe { _svxar_n_s8(op1, op2, IMM3) }
}
#[doc = "Bitwise exclusive OR and rotate right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svxar[_n_s16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(xar, IMM3 = 1))]
pub fn svxar_n_s16<const IMM3: i32>(op1: svint16_t, op2: svint16_t) -> svint16_t {
    static_assert_range!(IMM3, 1, 16);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.xar.nxv8i16")]
        fn _svxar_n_s16(op1: svint16_t, op2: svint16_t, imm3: i32) -> svint16_t;
    }
    unsafe { _svxar_n_s16(op1, op2, IMM3) }
}
#[doc = "Bitwise exclusive OR and rotate right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svxar[_n_s32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(xar, IMM3 = 1))]
pub fn svxar_n_s32<const IMM3: i32>(op1: svint32_t, op2: svint32_t) -> svint32_t {
    static_assert_range!(IMM3, 1, 32);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.xar.nxv4i32")]
        fn _svxar_n_s32(op1: svint32_t, op2: svint32_t, imm3: i32) -> svint32_t;
    }
    unsafe { _svxar_n_s32(op1, op2, IMM3) }
}
#[doc = "Bitwise exclusive OR and rotate right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svxar[_n_s64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(xar, IMM3 = 1))]
pub fn svxar_n_s64<const IMM3: i32>(op1: svint64_t, op2: svint64_t) -> svint64_t {
    static_assert_range!(IMM3, 1, 64);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", link_name = "llvm.aarch64.sve.xar.nxv2i64")]
        fn _svxar_n_s64(op1: svint64_t, op2: svint64_t, imm3: i32) -> svint64_t;
    }
    unsafe { _svxar_n_s64(op1, op2, IMM3) }
}
#[doc = "Bitwise exclusive OR and rotate right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svxar[_n_u8])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(xar, IMM3 = 1))]
pub fn svxar_n_u8<const IMM3: i32>(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    static_assert_range!(IMM3, 1, 8);
    unsafe { svxar_n_s8::<IMM3>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Bitwise exclusive OR and rotate right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svxar[_n_u16])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(xar, IMM3 = 1))]
pub fn svxar_n_u16<const IMM3: i32>(op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    static_assert_range!(IMM3, 1, 16);
    unsafe { svxar_n_s16::<IMM3>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Bitwise exclusive OR and rotate right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svxar[_n_u32])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(xar, IMM3 = 1))]
pub fn svxar_n_u32<const IMM3: i32>(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    static_assert_range!(IMM3, 1, 32);
    unsafe { svxar_n_s32::<IMM3>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[doc = "Bitwise exclusive OR and rotate right"]
#[doc = ""]
#[doc = "[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/svxar[_n_u64])"]
#[inline]
#[target_feature(enable = "sve,sve2")]
#[cfg_attr(test, assert_instr(xar, IMM3 = 1))]
pub fn svxar_n_u64<const IMM3: i32>(op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    static_assert_range!(IMM3, 1, 64);
    unsafe { svxar_n_s64::<IMM3>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
