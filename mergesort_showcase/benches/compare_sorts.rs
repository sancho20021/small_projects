use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};
use mergesort_showcase::{
    single_array_sort::SingleArray, slices_sort::Slices, slices_unchecked_sort::SlicesUnchecked,
    threshold_calc::get_threshold,
};

use crate::utils::bench_sort;

const SEQ_ARRAY_SIZES: &[usize] = &[
    512, 2_048, 16_384, 65_536, 250_000, 1_000_000, 5_000_000, 10_000_000,
];
const PAR_ARRAY_SIZES: &[usize] = &[
    250_000,
    500_000,
    2_000_000,
    8_000_000,
    16_000_000,
    64_000_000,
    100_000_000,
];

fn config() -> Criterion {
    Criterion::default().sample_size(20)
    .measurement_time(Duration::from_secs(50))
}

fn bench_sequential_sorts(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sequential");
    for &size in SEQ_ARRAY_SIZES {
        let threshold = size;
        bench_sort::<Slices>(&mut group, size, threshold, "Slices");
        bench_sort::<SlicesUnchecked>(&mut group, size, threshold, "Slices Unchecked");
        bench_sort::<SingleArray>(&mut group, size, threshold, "Single Array");
    }
    group.finish();
}

fn bench_parallel_sorts(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parallel");
    for &size in PAR_ARRAY_SIZES {
        let threshold = get_threshold(size);
        bench_sort::<Slices>(&mut group, size, threshold, "Slices");
        bench_sort::<SlicesUnchecked>(&mut group, size, threshold, "Slices Unchecked");
        bench_sort::<SingleArray>(&mut group, size, threshold, "Single Array");
    }
    group.finish();
}

criterion_group! {
    name = seq_merge_sorts;
    config = config();
    targets = bench_sequential_sorts,
}
criterion_group! {
    name = par_merge_sorts;
    config = config();
    targets = bench_parallel_sorts,
}
criterion_main!(seq_merge_sorts, par_merge_sorts);

mod utils {
    use criterion::{BatchSize, BenchmarkGroup, BenchmarkId, black_box, measurement::WallTime};
    use mergesort_showcase::Sort;
    use rand::RngCore;

    pub fn get_input_array(size: usize) -> Vec<i32> {
        let mut rng = rand::rng();
        let mut v = vec![];
        for _ in 0..size {
            v.push(rng.next_u32() as i32);
        }
        v
    }

    pub fn bench_sort<Algorithm: Sort>(
        group: &mut BenchmarkGroup<WallTime>,
        array_size: usize,
        threshold: usize,
        sort_name: &'static str,
    ) {
        group.bench_with_input(
            BenchmarkId::new(sort_name, array_size),
            &array_size,
            |b, size| {
                b.iter_batched_ref(
                    || black_box(get_input_array(*size)),
                    |input| Algorithm::sort(input, threshold),
                    BatchSize::LargeInput,
                );
            },
        );
    }
}
