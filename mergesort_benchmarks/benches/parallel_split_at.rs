use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use disjoint_mut_test::disjoint_verified::{self, exec_pcell::Array, split_at::ArrayAbstraction};
use mergesort_benchmarks::{
    get_threshold,
    merge_sorts::{merge_sort, merge_sort_parallel, merge_sort_threadpool, merge_sort_unchecked},
};
use rand::RngCore;
use rayon::slice::ParallelSliceMut;
use std::{hint::black_box, sync::Arc, time::Duration};

pub fn get_input_array(size: usize) -> Vec<i32> {
    let mut rng = rand::rng();

    let mut v = vec![];
    for i in 0..size {
        v.push(rng.next_u32() as i32);
    }
    v.reverse();
    v
}

fn parallel_mergesort(c: &mut Criterion) {
    for size in ARRAY_SIZES {
        let mut arr = black_box(get_input_array(size));
        let mut out_arr = black_box(vec![0; arr.len()]);
        let threshold = get_threshold(arr.len());
        let slice = arr.as_mut_slice();
        c.bench_with_input(
            BenchmarkId::new("parallel_mergesort", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let slice_ref = &mut *slice;
                    merge_sort_parallel(
                        black_box(slice_ref),
                        black_box(&mut out_arr),
                        black_box(threshold),
                    );
                    black_box(slice_ref);
                })
            },
        );
    }
}

fn threadpool_mergesort(c: &mut Criterion) {
    for size in ARRAY_SIZES {
        let mut arr = black_box(get_input_array(size));
        let mut out_arr = black_box(vec![0; arr.len()]);
        let threshold = get_threshold(arr.len());
        let slice = arr.as_mut_slice();
        c.bench_with_input(
            BenchmarkId::new("threadpool_mergesort", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let slice_ref = &mut *slice;
                    merge_sort_threadpool(
                        black_box(slice_ref),
                        black_box(&mut out_arr),
                        black_box(threshold),
                    );
                    black_box(slice_ref);
                })
            },
        );
    }
}

fn seq_mergesort(c: &mut Criterion) {
    for size in ARRAY_SIZES {
        let mut arr = black_box(get_input_array(size));
        let mut out_arr = black_box(vec![0; arr.len()]);
        let slice = arr.as_mut_slice();
        c.bench_with_input(BenchmarkId::new("seq_mergesort", size), &size, |b, _| {
            b.iter(|| {
                let slice_ref = &mut *slice;
                merge_sort(black_box(slice_ref), black_box(&mut out_arr));
                black_box(slice_ref);
            });
        });
    }
}

fn unchecked_seq_mergesort(c: &mut Criterion) {
    for size in ARRAY_SIZES {
        let mut arr = black_box(get_input_array(size));
        let mut out_arr = black_box(vec![0; arr.len()]);
        let slice = arr.as_mut_slice();
        c.bench_with_input(
            BenchmarkId::new("unchecked_seq_mergesort", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let slice_ref = &mut *slice;
                    merge_sort_unchecked(black_box(slice_ref), black_box(&mut out_arr));
                    black_box(slice_ref);
                });
            },
        );
    }
}

fn array_seq_mergesort(c: &mut Criterion) {
    for size in ARRAY_SIZES {
        let arr = black_box(get_input_array(size));
        let mut arr = ArrayAbstraction::new(arr);
        c.bench_with_input(
            BenchmarkId::new("array_seq_mergesort", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let out_arr = vec![0; arr.array.length()];
                    disjoint_verified::split_at::mergesort::merge_sort_abstraction(
                        black_box(&mut arr),
                        black_box(out_arr),
                    );
                });
            },
        );
    }
}

fn array_par_mergesort(c: &mut Criterion) {
    for size in ARRAY_SIZES {
        let arr = black_box(get_input_array(size));
        let threshold = get_threshold(arr.len());
        let mut arr = ArrayAbstraction::new(arr);
        c.bench_with_input(
            BenchmarkId::new("array_par_mergesort", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let out_array = vec![0; arr.array.length()];
                    disjoint_verified::split_at::mergesort::merge_sort_parallel_abstraction(
                        &mut arr,
                        black_box(out_array),
                        threshold,
                    )
                    .unwrap();
                });
            },
        );
    }
}

fn rayon_par_mergesort(c: &mut Criterion) {
    for size in ARRAY_SIZES {
        let mut arr = black_box(get_input_array(size));
        c.bench_with_input(
            BenchmarkId::new("rayon_par_mergesort", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let arr = black_box(&mut arr);
                    arr.as_mut_slice().par_sort();
                    black_box(arr);
                });
            },
        );
    }
}

static ARRAY_SIZES: [usize; 2] = [
    /* 50_000,*/ /* 100_000, 500_000, */ /* 1_000_000,*/ 2_000_000, 4_000_000, /* 100_000_000 ,*/
];

fn small_config() -> Criterion {
    Criterion::default()
        .sample_size(10)
        .measurement_time(Duration::from_secs(40))
}

criterion_group! {
    name = merge_sorts;
    config = small_config();
    targets = parallel_mergesort, seq_mergesort, /* threadpool_mergesort,*/ /* array_seq_mergesort, array_par_mergesort, unchecked_seq_mergesort*/
    // targets = rayon_par_mergesort
    // targets = unchecked_seq_mergesort
}
criterion_main!(merge_sorts);
