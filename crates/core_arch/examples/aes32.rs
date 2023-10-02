#![feature(stdsimd, riscv_ext_intrinsics)]

fn main() {
    println!("Hello");

    #[cfg(target_feature = "zk")]
    let _ = std::hint::black_box({ unsafe { core_arch::arch::riscv64::aes64ks1i(4, 2) } });

    println!("World!");
}
