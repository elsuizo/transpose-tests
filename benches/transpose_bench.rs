use criterion::{criterion_group, criterion_main, Criterion};
use static_math::traits::LinearAlgebra;
use static_math::{m55_new, M55};
use vec_of_vec::transpose;
use vec_of_vec::Matrix;

fn transpose1() {
    let m1 = vec![
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 8, 9, 10],
        vec![11, 12, 13, 14, 15],
        vec![16, 17, 18, 19, 20],
        vec![21, 22, 23, 24, 25],
    ];
    let _ = transpose(m1);
}

fn transpose2() {
    let m1 = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    ];
    let matrix = Matrix::new(5, 5, m1);
    let _ = matrix.transpose();
}

fn transpose3() {
    let m = m55_new!(1.0,   2.0,  3.0,  4.0,  5.0;
                     6.0,   7.0,  8.0,  9.0, 10.0;
                     11.0, 12.0, 13.0, 14.0, 15.0;
                     16.0, 17.0, 18.0, 19.0, 20.0;
                     21.0, 22.0, 23.0, 24.0, 25.0);
    let _ = m.transpose();
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("transpose1", |b| b.iter(|| transpose1()));
    c.bench_function("transpose2", |b| b.iter(|| transpose2()));
    c.bench_function("transpose3", |b| b.iter(|| transpose3()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
