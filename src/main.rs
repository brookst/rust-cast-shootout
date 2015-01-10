//! Shootout of float casting functions
//! See http://www.reddit.com/r/rust/comments/2moc2u/float_to_u8_conversion_help
extern crate test;

#[cfg(test)]
use test::Bencher;

use std::num::{Float,FromPrimitive,ToPrimitive,NumCast};
#[allow(dead_code)]
/// Original function by Morphit (http://www.reddit.com/user/Morphit)
fn attenuate_if<T: Float + FromPrimitive>(channel: T) -> u8 {
    if channel > FromPrimitive::from_u8(255).unwrap() {
        255u8
    } else if channel < FromPrimitive::from_u8(0).unwrap() {
        0u8
    } else {
        channel.to_u8().unwrap()
    }
}

#[allow(dead_code)]
/// Min/Max version by arthurprs (http://www.reddit.com/user/arthurprs)
fn attenuate_minmax<T: Float + FromPrimitive>(channel: T) -> u8 {
    channel.min(FromPrimitive::from_f64(255.0).unwrap()).max(FromPrimitive::from_f64(0.0).unwrap()).to_u8().unwrap()
}

#[allow(dead_code)]
/// Iterator version by thiez (http://www.reddit.com/user/thiez)
fn attenuate_iter<T: Float>(channel: T) -> u8 {
    NumCast::from(255u8)
        .into_iter()
        .zip(NumCast::from(0u8).into_iter())
        .map(|(hi,lo)| channel.min(hi).max(lo))
        .next()
        .and_then(|t|t.to_u8())
        .unwrap_or(0)
}

#[allow(dead_code)]
/// Genericised version by arthurprs (http://www.reddit.com/user/arthurprs)
fn attenuate_generic<T: PartialOrd + FromPrimitive + ToPrimitive>(channel: T) -> u8 {
    if channel > FromPrimitive::from_u8(255).unwrap() {
        255u8
    } else if channel < FromPrimitive::from_u8(0).unwrap() {
        0u8
    } else {
        channel.to_u8().unwrap()
    }
}

#[allow(dead_code)]
fn main() {
    assert_eq!(attenuate_if ( 0.0f32 ) , 0u8);
    assert_eq!(attenuate_if ( 255.0f64 ) , 255u8);
    assert_eq!(attenuate_if ( 256.0f32 ) , 255u8);
    assert_eq!(attenuate_if ( - 1.0f64 ) , 0u8);
}

#[test]
fn test_if() {
    assert_eq!(attenuate_if ( 0.0f32 ) , 0u8);
    assert_eq!(attenuate_if ( 255.0f64 ) , 255u8);
    assert_eq!(attenuate_if ( 256.0f32 ) , 255u8);
    assert_eq!(attenuate_if ( - 1.0f64 ) , 0u8);
}

#[bench]
fn bench_if_values(b: &mut Bencher) {
    let v: Vec<f32> = (0..256).map(|n| n as f32).collect();
    b.iter(|| v.iter().map(|n| attenuate_if(*n)));
}

#[bench]
fn bench_if_zero(b: &mut Bencher) {
    b.iter(|| attenuate_if(0.0f32));
}

#[bench]
fn bench_if_max(b: &mut Bencher) {
    b.iter(|| attenuate_if(255.0f32));
}

#[bench]
fn bench_if_over(b: &mut Bencher) {
    b.iter(|| attenuate_if(256.0f32));
}

#[bench]
fn bench_if_under(b: &mut Bencher) {
    b.iter(|| attenuate_if(-1.0f32));
}

#[test]
fn test_minmax() {
    assert_eq!(attenuate_minmax ( 0.0f32 ) , 0u8);
    assert_eq!(attenuate_minmax ( 255.0f64 ) , 255u8);
    assert_eq!(attenuate_minmax ( 256.0f32 ) , 255u8);
    assert_eq!(attenuate_minmax ( - 1.0f64 ) , 0u8);
}

#[bench]
fn bench_minmax_values(b: &mut Bencher) {
    let v: Vec<f32> = (0..256).map(|n| n as f32).collect();
    b.iter(|| v.iter().map(|n| attenuate_minmax(*n)));
}

#[bench]
fn bench_minmax_zero(b: &mut Bencher) {
    b.iter(|| attenuate_minmax(0.0f32));
}

#[bench]
fn bench_minmax_max(b: &mut Bencher) {
    b.iter(|| attenuate_minmax(255.0f32));
}

#[bench]
fn bench_minmax_over(b: &mut Bencher) {
    b.iter(|| attenuate_minmax(256.0f32));
}

#[bench]
fn bench_minmax_under(b: &mut Bencher) {
    b.iter(|| attenuate_minmax(-1.0f32));
}

#[test]
fn test_iter() {
    assert_eq!(attenuate_iter ( 0.0f32 ) , 0u8);
    assert_eq!(attenuate_iter ( 255.0f64 ) , 255u8);
    assert_eq!(attenuate_iter ( 256.0f32 ) , 255u8);
    assert_eq!(attenuate_iter ( - 1.0f64 ) , 0u8);
}

#[bench]
fn bench_iter_values(b: &mut Bencher) {
    let v: Vec<f32> = (0..256).map(|n| n as f32).collect();
    b.iter(|| v.iter().map(|n| attenuate_iter(*n)));
}

#[bench]
fn bench_iter_zero(b: &mut Bencher) {
    b.iter(|| attenuate_iter(0.0f32));
}

#[bench]
fn bench_iter_max(b: &mut Bencher) {
    b.iter(|| attenuate_iter(255.0f32));
}

#[bench]
fn bench_iter_over(b: &mut Bencher) {
    b.iter(|| attenuate_iter(256.0f32));
}

#[bench]
fn bench_iter_under(b: &mut Bencher) {
    b.iter(|| attenuate_iter(-1.0f32));
}

#[test]
fn test_generic() {
    assert_eq!(attenuate_generic ( 0.0f32 ) , 0u8);
    assert_eq!(attenuate_generic ( 255.0f64 ) , 255u8);
    assert_eq!(attenuate_generic ( 256.0f32 ) , 255u8);
    assert_eq!(attenuate_generic ( - 1.0f64 ) , 0u8);
}

#[bench]
fn bench_generic_values(b: &mut Bencher) {
    let v: Vec<f32> = (0..256).map(|n| n as f32).collect();
    b.iter(|| v.iter().map(|n| attenuate_generic(*n)));
}

#[bench]
fn bench_generic_zero(b: &mut Bencher) {
    b.iter(|| attenuate_generic(0.0f32));
}

#[bench]
fn bench_generic_max(b: &mut Bencher) {
    b.iter(|| attenuate_generic(255.0f32));
}

#[bench]
fn bench_generic_over(b: &mut Bencher) {
    b.iter(|| attenuate_generic(256.0f32));
}

#[bench]
fn bench_generic_under(b: &mut Bencher) {
    b.iter(|| attenuate_generic(-1.0f32));
}
