use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};
use mergesort_showcase::{single_array_sort, slices_sort};

use crate::utils::benchmark_sort;

const SEQ_ARRAY_SIZES: [usize; 1] = [
    100_000,
    // 1_000_000,
    // 10_000_000
];
const PAR_ARRAY_SIZES: [usize; 1] = [
    10_000_000,
    // 20_000_000,
    // 100_000_000
];

fn config() -> Criterion {
    Criterion::default()
        .sample_size(10)
        // .measurement_time(Duration::from_secs(60))
}

fn bench_slices_parallel(c: &mut Criterion) {
    benchmark_sort(
        c,
        &PAR_ARRAY_SIZES,
        true,
        "slices parallel",
        slices_sort::merge_sort_parallel,
    );
}

fn bench_single_array_parallel(c: &mut Criterion) {
    benchmark_sort(
        c,
        &PAR_ARRAY_SIZES,
        true,
        "single array parallel",
        single_array_sort::merge_sort_parallel,
    );
}

fn bench_slices_sequential(c: &mut Criterion) {
    benchmark_sort(
        c,
        &SEQ_ARRAY_SIZES,
        false,
        "slices sequential",
        slices_sort::merge_sort_parallel,
    );
}

fn bench_single_array_sequential(c: &mut Criterion) {
    benchmark_sort(
        c,
        &SEQ_ARRAY_SIZES,
        true,
        "single array sequential",
        single_array_sort::merge_sort_parallel,
    );
}

criterion_group! {
    name = seq_merge_sorts;
    config = config();
    targets = bench_slices_sequential, bench_single_array_sequential,
}
criterion_group! {
    name = par_merge_sorts;
    config = config();
    targets = bench_slices_parallel, bench_single_array_parallel,
}
criterion_main!(seq_merge_sorts, par_merge_sorts);

mod utils {
    use criterion::{BenchmarkId, Criterion, black_box};
    use mergesort_showcase::threshold_calc::get_threshold;
    use rand::RngCore;

    pub fn get_input_array(size: usize) -> Vec<i32> {
        let mut rng = rand::rng();
        let mut v = vec![];
        for _ in 0..size {
            v.push(rng.next_u32() as i32);
        }
        v
    }

    pub fn benchmark_sort(
        c: &mut Criterion,
        sizes: &[usize],
        parallel: bool,
        name: &'static str,
        sort: impl Fn(&mut [i32], usize),
    ) {
        for &size in sizes {
            let threshold = if parallel { get_threshold(size) } else { size };
            let input = get_input_array(size);
            c.bench_with_input(BenchmarkId::new(name, size), &size, |b, _| {
                b.iter(|| {
                    let mut arr = black_box(input.clone());
                    sort(&mut arr, threshold);
                });
            });
        }
    }
}
