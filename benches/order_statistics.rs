extern crate rand;
extern crate statrs;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use rand::prelude::*;
// use statrs::distribution::Categorical;
// use statrs::distribution::InverseCDF;
use statrs::statistics::*;

fn bench_order_statistic(c: &mut Criterion) {
    let mut rng = thread_rng();
    let to_random_owned = |data: &[f64]| -> Vec<f64> {
        let mut rng = thread_rng();
        let mut owned = data.to_vec();
        owned.shuffle(&mut rng);
        owned
    };
    let k = black_box(rng.gen());
    let tau = black_box(rng.gen_range(0.0, 1.0));
    let mut group = c.benchmark_group("order statistic");
    let data: Vec<_> = (0..100).map(|x| x as f64).collect();
    group.bench_function("order_statistic", |b| {
        b.iter_batched(
            || to_random_owned(&data),
            |mut data| data.order_statistic(k),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("median", |b| {
        b.iter_batched(
            || to_random_owned(&data),
            |data| data.median(),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("quantile", |b| {
        b.iter_batched(
            || to_random_owned(&data),
            |mut data| data.quantile(tau),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("percentile", |b| {
        b.iter_batched(
            || to_random_owned(&data),
            |mut data| data.percentile(k),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("lower_quartile", |b| {
        b.iter_batched(
            || to_random_owned(&data),
            |mut data| data.lower_quartile(),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("upper_quartile", |b| {
        b.iter_batched(
            || to_random_owned(&data),
            |mut data| data.upper_quartile(),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("interquartile_range", |b| {
        b.iter_batched(
            || to_random_owned(&data),
            |mut data| data.interquartile_range(),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("ranks: RankTieBreaker::First", |b| {
        b.iter_batched(
            || to_random_owned(&data),
            |mut data| data.ranks(RankTieBreaker::First),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("ranks: RankTieBreaker::Average", |b| {
        b.iter_batched(
            || to_random_owned(&data),
            |mut data| data.ranks(RankTieBreaker::Average),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("ranks: RankTieBreaker::Min", |b| {
        b.iter_batched(
            || to_random_owned(&data),
            |mut data| data.ranks(RankTieBreaker::Min),
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

// fn bench_categorical_distribution(c: &mut Criterion) {
//     c.bench_function("new categorical distribution", |b| {
//         b.iter_batched(
//             || rand_vec(100),
//             |slice| Categorical::new(&slice),
//             BatchSize::SmallInput,
//         )
//     });
// }

// fn categorical_distribution_inverse_cdf(c: &mut Criterion) {
//     let x = black_box(5.0);
//     c.bench_function("inverse_cdf", |b| {
//         b.iter_batched(
//             || Categorical::new(&rand_vec(100)).unwrap(),
//             |categorical| categorical.inverse_cdf(x),
//             BatchSize::SmallInput,
//         )
//     });
// }

criterion_group!(benches, bench_order_statistic);
criterion_main!(benches);
