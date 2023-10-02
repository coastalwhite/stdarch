use crate::core_arch::simd_llvm::simd_reinterpret;

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

pub fn vmv_v_x_i8(rs: u8, vl: usize) -> vint8_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[link_name = "llvm.riscv.vmv.v.x.nxv8i8"]
        fn _vmv_v_x_i8(_poison: vint8_t, rs: i8, vl: i64) -> vint8_t;
    }

    unsafe { _vmv_v_x_i8(undef!(), rs as i8, vl as i64) }
}

#[inline]
#[rustc_legacy_const_generics(1)]
pub fn vsetvli<const ELEMENT: i64>(vl: usize) -> usize {
    #[allow(improper_ctypes)]
    extern "C" {
        #[link_name = "llvm.riscv.vsetvli.i64"]
        fn _vsetvli(vl: i64, ei: i64, mi: i64) -> i64;
    }

    unsafe { _vsetvli(vl as i64, ELEMENT, 5) as usize }
}

#[inline]
pub fn vle8_v(ptr: *const u8, vl: usize) -> vint8_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[link_name = "llvm.riscv.vle.nxv8i8.i64"]
        fn _vle8_v(_poison: vint8_t, ptr: *const u8, vl: i64) -> vint8_t;
    }

    unsafe { _vle8_v(undef!(), ptr, vl as i64) }
}

#[inline]
pub fn vse8_v(vs: vint8_t, ptr: *mut u8, vl: usize) {
    #[allow(improper_ctypes)]
    extern "C" {
        #[link_name = "llvm.riscv.vse.nxv8i8.i64"]
        fn _vse8_v(vs: vint8_t, ptr: *mut u8, vl: i64);
    }

    unsafe { _vse8_v(vs, ptr, vl as i64) }
}

#[inline]
pub fn vadd_vv(vs1: vint8_t, vs2: vint8_t, vl: usize) -> vint8_t {
    #[allow(improper_ctypes)]
    extern "C" {
        #[link_name = "llvm.riscv.vadd.nxv8i8.nxv8i8.i64"]
        fn _vadd_vv(_poison: vint8_t, op1: vint8_t, op2: vint8_t, vl: i64) -> vint8_t;
    }

    unsafe { _vadd_vv(undef!(), vs1, vs2, vl as i64) }
}
