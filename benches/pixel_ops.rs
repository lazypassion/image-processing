#[macro_use]
extern crate criterion;
extern crate image_processing;

use criterion::Criterion;
use criterion::black_box;
use image;
use core::time::Duration;

use image_processing::pixel_ops::*;

pub fn logarithm(c: &mut Criterion) {
    let mut group = c.benchmark_group("Logarithm");
    group.confidence_level(0.05);
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(150));

    let mut image = image::open("./images/england-hampton-court-palace.jpg")
        .expect("image not found")
        .to_luma();

    let mut second_image = image.clone();

    group.bench_function("LUT", |b| {
        b.iter(|| {
            logarithm_mut(black_box(&mut image));
        });
    });

    group.finish();
}

pub fn invert(c: &mut Criterion) {
    let mut group = c.benchmark_group("Invert");
    group.confidence_level(0.05);
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(150));

    let mut image = image::open("./images/england-hampton-court-palace.jpg")
        .expect("image not found")
        .to_luma();

    let mut second_image = image.clone();

    group.bench_function("Calculated", |b| {
        b.iter(|| {
            invert_mut(black_box(&mut image));
        });
    });

    group.finish();
}

criterion_group!(benches, logarithm, invert);
criterion_main!(benches);