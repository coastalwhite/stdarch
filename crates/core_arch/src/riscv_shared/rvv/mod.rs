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

macro_rules! undef {
    () => {{
        simd_reinterpret(())
    }};
}

#[inline]
#[target_feature(enable = "v")]
pub unsafe fn vmv_v_x_i8(rs: u8, vl: usize) -> vint8 {
    #[allow(improper_ctypes)]
    extern "C" {
        #[link_name = "llvm.riscv.vmv.v.x.nxv8i8"]
        fn _vmv_v_x_i8(_poison: vint8, rs: i8, vl: i64) -> vint8;
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

macro_rules! impl_rvv_type {
    ($(($v:vis, $elem_type:ty, $name:ident, $elt:literal))*) => ($(
        #[repr(simd, scalable($elt))]
        #[allow(non_camel_case_types)]
        $v struct $name {
            _ty: [$elem_type],
        }
    )*)
}

macro_rules! intrinsic_fn {
    ($self:ty, $name:ident {
        intrinsic: $llvm_intrinsic:literal,
        signature: ($($param:ident: $param_ty:ty),*) $(-> $ret:ty)?,
        llvm_signature: ($($llvm_param:ident: $llvm_param_ty:ty),*) $(-> $llvm_ret:ty)?,
        llvm_eval: ($($eval_param:expr),*) $(,)?
    }) => {
        #[inline]
        #[target_feature(enable = "v")]
        pub unsafe fn $name($($param: $param_ty),*) $(-> $ret)? {
            #[allow(improper_ctypes)]
            extern "unadjusted" {
                #[link_name = $llvm_intrinsic]
                fn __llvm_intrinsic($($llvm_param: $llvm_param_ty),*) $(-> $llvm_ret)?;
            }

            unsafe { __llvm_intrinsic($($eval_param),*) }
        }
    };
}

macro_rules! impl_rvv_fns {
    ($({$name:ty, $unsigned_elem_type:ty, $signed_elem_type:ty {
        vl_v: $vl_v:ident => $vl_v_llvm_intrinsic:literal,
        vs_v: $vs_v:ident => $vs_v_llvm_intrinsic:literal,
        vadd_vv: $vadd_vv:ident => $vadd_vv_llvm_intrinsic:literal,
    }})*) => (
        $(
        intrinsic_fn!(
            $name, $vl_v {
                intrinsic: $vl_v_llvm_intrinsic,
                signature: (ptr: *const $unsigned_elem_type, vl: usize) -> $name,
                llvm_signature: (_undef: $name, ptr: *const $signed_elem_type, vl: i64) -> $name,
                llvm_eval: (undef!(), ptr as *const $signed_elem_type, vl as i64),
            }
        );

        intrinsic_fn!(
            $name, $vs_v {
                intrinsic: $vs_v_llvm_intrinsic,
                signature: (vs: $name, ptr: *mut $unsigned_elem_type, vl: usize),
                llvm_signature: (vs: $name, ptr: *mut $signed_elem_type, vl: i64),
                llvm_eval: (vs, ptr as *mut $signed_elem_type, vl as i64),
            }
        );

        intrinsic_fn!(
            $name, $vadd_vv {
                intrinsic: $vadd_vv_llvm_intrinsic,
                signature: (vs1: $name, vs2: $name, vl: usize) -> $name,
                llvm_signature: (_undef: $name, vs1: $name, vs2: $name, vl: i64) -> $name,
                llvm_eval: (undef!(), vs1, vs2, vl as i64),
            }
        );
    )*)
}

impl_rvv_type! {
    (pub, u8, vint8, 8)
    (pub, u16, vint16, 16)
    (pub, u32, vint32, 32)
    (pub, u64, vint64, 16)
}

impl_rvv_fns! {
    {
        vint8, u8, i8 {
            vl_v: vle8_v => "llvm.riscv.vle.nxv8i8.i64",
            vs_v: vse8_v => "llvm.riscv.vse.nxv8i8.i64",
            vadd_vv: vadd_e8_vv => "llvm.riscv.vadd.nxv8i8.nxv8i8.i64",
        }
    }
    {
        vint16, u16, i16 {
            vl_v: vle16_v => "llvm.riscv.vle.nxv8i16.i64",
            vs_v: vse16_v => "llvm.riscv.vse.nxv8i16.i64",
            vadd_vv: vadd_e16_vv => "llvm.riscv.vadd.nxv8i16.nxv8i16.i64",
        }
    }
}
