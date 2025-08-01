use cidre::{cf, ns};
use criterion::{Criterion, criterion_group, criterion_main};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("cf::uuid", |b| {
        b.iter(|| {
            cf::Uuid::new();
        })
    });

    c.bench_function("ns::uuid", |b| {
        b.iter(|| {
            ns::Uuid::new();
        })
    });

    c.bench_function("rust::uuid_v4", |b| {
        b.iter(|| {
            uuid::Uuid::new_v4();
        })
    });

    c.bench_function("rust::uuid_v7", |b| {
        b.iter(|| {
            uuid::Uuid::now_v7();
        })
    });

    c.bench_function("cf::uuid::to_cf_string", |b| {
        b.iter(|| {
            cf::Uuid::new().to_cf_string();
        })
    });

    c.bench_function("ns::uuid::string", |b| {
        b.iter(|| {
            ns::Uuid::new().string();
        })
    });

    c.bench_function("rust::uuid::to_string", |b| {
        b.iter(|| {
            uuid::Uuid::new_v4().to_string();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
