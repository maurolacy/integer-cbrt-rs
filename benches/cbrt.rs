#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

extern crate integer_cbrt;
use integer_cbrt::IntegerCubeRoot;

// Use f64::cbrt to compute the integer cbrt
fn icbrt_via_f64(n: u64) -> u64 {
    let cand = (n as f64).cbrt() as u64;
    // Rounding can cause off-by-one errors
    if let Some(prod) = cand.checked_mul(cand) {
        if prod <= n {
            return cand;
        }
    }
    cand - 1
}

#[bench]
fn icbrt_u64_small(b: &mut Bencher) {
    let small = 511u64;
    b.iter(|| {
        let n = black_box(small);
        assert_eq!(n.integer_cbrt_checked(), Some(7));
    })
}

#[bench]
fn icbrt_u64_med(b: &mut Bencher) {
    let med = 1_000_000_000_000_000u64; // 10^15
    b.iter(|| {
        let n = black_box(med);
        assert_eq!(n.integer_cbrt_checked(), Some(100_000)); // 10^5
    })
}

#[bench]
fn icbrt_u64_large(b: &mut Bencher) {
    let large = u64::MAX;
    b.iter(|| {
        let n = black_box(large);
        assert_eq!(n.integer_cbrt_checked(), Some(2642245));
    })
}

#[bench]
fn icbrt_u128_small(b: &mut Bencher) {
    let small = 511u128;
    b.iter(|| {
        let n = black_box(small);
        assert_eq!(n.integer_cbrt_checked(), Some(7));
    })
}

#[bench]
fn icbrt_u128_med(b: &mut Bencher) {
    let med = 1_000_000_000_000_000u128; // 10^15
    b.iter(|| {
        let n = black_box(med);
        assert_eq!(n.integer_cbrt_checked(), Some(100_000)); // 10^5
    })
}

#[bench]
fn icbrt_u128_large(b: &mut Bencher) {
    let large = u128::MAX;
    b.iter(|| {
        let n = black_box(large);
        assert_eq!(n.integer_cbrt_checked(), Some(6981463658331));
    })
}

#[bench]
fn icbrt_f64_small(b: &mut Bencher) {
    let small = 511u64;
    b.iter(|| {
        let n = black_box(small);
        assert_eq!(icbrt_via_f64(n), 7);
    })
}

#[bench]
fn icbrt_f64_med(b: &mut Bencher) {
    let med = 1_000_000_000_000_000u64; // 10^15
    b.iter(|| {
        let n = black_box(med);
        assert_eq!(icbrt_via_f64(n), 100_000); // 10^5
    })
}

#[bench]
fn icbrt_f64_large(b: &mut Bencher) {
    let large = u64::MAX;
    b.iter(|| {
        let n = black_box(large);
        assert_eq!(icbrt_via_f64(n), 2642245);
    })
}
