use core::marker::ConstParamTy;

use crate::core_arch::simd_llvm::simd_reinterpret;

#[repr(isize)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SelectedElementWidth {
    E8 = 0b000,
    E16 = 0b001,
    E32 = 0b010,
    E64 = 0b011,
}

#[repr(isize)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VectorLengthMultiplier {
    F8 = 0b101,
    F4 = 0b110,
    F2 = 0b111,
    M1 = 0b000,
    M2 = 0b001,
    M4 = 0b010,
    M8 = 0b011,
}

impl ConstParamTy for SelectedElementWidth {}
impl ConstParamTy for VectorLengthMultiplier {}

#[repr(simd, scalable(8))]
#[allow(non_camel_case_types)]
pub struct vint8_t {
    _ty: [u8],
}

macro_rules! undef {
    () => {{
        simd_reinterpret(())
    }};
}

#[inline]
#[target_feature(enable = "v")]
pub unsafe fn vmv_v_x_i8(rs: u8, vl: usize) -> vint8_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[link_name = "llvm.riscv.vmv.v.x.nxv8i8"]
        fn _vmv_v_x_i8(_poison: vint8_t, rs: i8, vl: i64) -> vint8_t;
    }

    unsafe { _vmv_v_x_i8(undef!(), rs as i8, vl as i64) }
}

#[inline]
#[target_feature(enable = "v")]
#[rustc_legacy_const_generics(1, 2)]
pub unsafe fn vsetvli<const SEW: SelectedElementWidth, const LMUL: VectorLengthMultiplier>(
    vl: usize,
) -> usize {
    #[allow(improper_ctypes)]
    extern "C" {
        #[link_name = "llvm.riscv.vsetvli.i64"]
        fn _vsetvli(vl: i64, ei: i64, mi: i64) -> i64;
    }

    unsafe { _vsetvli(vl as i64, SEW as isize as i64, LMUL as isize as i64) as usize }
}

#[inline]
#[target_feature(enable = "v")]
pub unsafe fn vle8_v(ptr: *const u8, vl: usize) -> vint8_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[link_name = "llvm.riscv.vle.nxv8i8.i64"]
        fn _vle8_v(_poison: vint8_t, ptr: *const u8, vl: i64) -> vint8_t;
    }

    unsafe { _vle8_v(undef!(), ptr, vl as i64) }
}

#[inline]
#[target_feature(enable = "v")]
pub unsafe fn vse8_v(vs: vint8_t, ptr: *mut u8, vl: usize) {
    #[allow(improper_ctypes)]
    extern "C" {
        #[link_name = "llvm.riscv.vse.nxv8i8.i64"]
        fn _vse8_v(vs: vint8_t, ptr: *mut u8, vl: i64);
    }

    unsafe { _vse8_v(vs, ptr, vl as i64) }
}

#[inline]
#[target_feature(enable = "v")]
pub unsafe fn vadd_vv(vs1: vint8_t, vs2: vint8_t, vl: usize) -> vint8_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[link_name = "llvm.riscv.vadd.nxv8i8.nxv8i8.i64"]
        fn _vadd_vv(_poison: vint8_t, op1: vint8_t, op2: vint8_t, vl: i64) -> vint8_t;
    }

    unsafe { _vadd_vv(undef!(), vs1, vs2, vl as i64) }
}
