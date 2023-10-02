#![allow(incomplete_features)]
#![feature(stdsimd, riscv_target_feature, unsized_locals, unsized_fn_params)]

use core_arch::arch::riscv64::{
    vadd_e16_vv, vadd_e8_vv, vle16_v, vle8_v, vse16_v, vse8_v, vsetvli, SelectedElementWidth,
    VectorLengthMultiplier,
};

// examples/vadd.rs
#[inline(never)]
#[no_mangle]
#[target_feature(enable = "v")]
unsafe fn vadd8(x: &[u8], y: &[u8], result: &mut [u8]) {
    assert!(x.len() == y.len() && x.len() == result.len());

    let mut vl = x.len();

    let mut x = x as *const [u8] as *const u8;
    let mut y = y as *const [u8] as *const u8;
    let mut result = result as *mut [u8] as *mut u8;

    while vl > 0 {
        let avl = unsafe { vsetvli(vl, SelectedElementWidth::E8, VectorLengthMultiplier::M1) };

        let vx = unsafe { vle8_v(x, vl) };
        let vy = unsafe { vle8_v(y, vl) };

        let vresult = unsafe { vadd_e8_vv(vx, vy, vl) };

        unsafe { vse8_v(vresult, result, vl) };

        vl -= avl;

        unsafe {
            x = x.offset(avl as isize);
            y = y.offset(avl as isize);
            result = result.offset(avl as isize);
        }
    }
}

#[inline(never)]
#[no_mangle]
#[target_feature(enable = "v")]
unsafe fn vadd16(x: &[u16], y: &[u16], result: &mut [u16]) {
    assert!(x.len() == y.len() && x.len() == result.len());

    let mut vl = x.len();

    let mut x = x as *const [u16] as *const u16;
    let mut y = y as *const [u16] as *const u16;
    let mut result = result as *mut [u16] as *mut u16;

    while vl > 0 {
        let avl = unsafe { vsetvli(vl, SelectedElementWidth::E16, VectorLengthMultiplier::M1) };

        let vx = unsafe { vle16_v(x, vl) };
        let vy = unsafe { vle16_v(y, vl) };

        let vresult = unsafe { vadd_e16_vv(vx, vy, vl) };

        unsafe { vse16_v(vresult, result, vl) };

        vl -= avl;

        unsafe {
            x = x.offset(avl as isize);
            y = y.offset(avl as isize);
            result = result.offset(avl as isize);
        }
    }
}

fn main() {
    let mut result = [0u8; 3];

    unsafe { vadd8(&[1, 2, 3], &[6, 3, 5], &mut result) };

    assert_eq!(result, [7, 5, 8]);

    let x: [u8; 435] = std::array::from_fn(|i| (i % 10) as u8);
    let y: [u8; 435] = std::array::from_fn(|i| (i % 23) as u8);
    let mut result = [0u8; 435];

    unsafe { vadd8(&x, &y, &mut result) };

    assert_eq!(
        result,
        std::array::from_fn(|i| (i % 10) as u8 + (i % 23) as u8)
    );

    let mut result = [0u16; 3];

    unsafe { vadd16(&[1, 2, 3], &[6, 3, 5], &mut result) };

    assert_eq!(result, [7, 5, 8]);

    let x: [u16; 435] = std::array::from_fn(|i| (i % 10) as u16);
    let y: [u16; 435] = std::array::from_fn(|i| (i % 23) as u16);
    let mut result = [0u16; 435];

    unsafe { vadd16(&x, &y, &mut result) };

    assert_eq!(
        result,
        std::array::from_fn(|i| (i % 10) as u16 + (i % 23) as u16)
    );
}
