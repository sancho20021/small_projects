use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use disjoint_mut_test::disjoint_verified::{self, exec_pcell::Array};
use mergesort_benchmarks::merge_sorts::{merge_sort, merge_sort_parallel, merge_sort_threadpool};
use rayon::slice::ParallelSliceMut;
use std::{hint::black_box, sync::Arc, time::Duration};

// good threshold according to internet :)
// hmmm but I shouldn't spawn threads, use threadpools more
// static SORT_PARALLEL_THRESHOLD: usize = 2048;

// maybe I can mitigate that problem by raising the threashold
static SORT_PARALLEL_THRESHOLD: usize = 10_000;

fn get_reversed_array(size: usize) -> Vec<i32> {
    let mut v = vec![];
    for i in 0..size {
        v.push(i as i32);
    }
    v.reverse();
    v
}

fn parallel_mergesort(c: &mut Criterion) {
    for size in ARRAY_SIZES {
        let mut arr = black_box(get_reversed_array(size));
        let slice = arr.as_mut_slice();
        c.bench_with_input(
            BenchmarkId::new("parallel_mergesort", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let slice_ref = &mut *slice;
                    merge_sort_parallel(black_box(slice_ref), black_box(SORT_PARALLEL_THRESHOLD));
                    black_box(slice_ref);
                })
            },
        );
    }
}

fn threadpool_mergesort(c: &mut Criterion) {
    for size in ARRAY_SIZES {
        let mut arr = black_box(get_reversed_array(size));
        let slice = arr.as_mut_slice();
        c.bench_with_input(
            BenchmarkId::new("threadpool_mergesort", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let slice_ref = &mut *slice;
                    merge_sort_threadpool(black_box(slice_ref), black_box(SORT_PARALLEL_THRESHOLD));
                    black_box(slice_ref);
                })
            },
        );
    }
}

fn seq_mergesort(c: &mut Criterion) {
    for size in ARRAY_SIZES {
        let mut arr = black_box(get_reversed_array(size));
        let slice = arr.as_mut_slice();
        c.bench_with_input(BenchmarkId::new("seq_mergesort", size), &size, |b, _| {
            b.iter(|| {
                let slice_ref = &mut *slice;
                merge_sort(black_box(slice_ref));
                black_box(slice_ref);
            });
        });
    }
}

fn array_seq_mergesort(c: &mut Criterion) {
    for size in ARRAY_SIZES {
        let arr = black_box(get_reversed_array(size));
        let (arr, mut perms) = Array::new(arr);
        c.bench_with_input(
            BenchmarkId::new("array_seq_mergesort", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let perms = black_box(&mut perms);
                    disjoint_verified::merge_sort::merge_sort(
                        black_box(&arr),
                        0,
                        arr.length(),
                        perms,
                    );
                    black_box(perms);
                });
            },
        );
    }
}

fn array_par_mergesort(c: &mut Criterion) {
    for size in ARRAY_SIZES {
        let arr = black_box(get_reversed_array(size));
        let (arr, mut perms) = Array::new(arr);
        let arr = Arc::new(arr);
        c.bench_with_input(
            BenchmarkId::new("array_par_mergesort", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let perms = black_box(&mut perms);
                    disjoint_verified::merge_sort::merge_sort_parallel(
                        black_box(Arc::clone(&arr)),
                        0,
                        arr.length(),
                        perms,
                        SORT_PARALLEL_THRESHOLD,
                    )
                    .unwrap();
                    black_box(perms);
                });
            },
        );
    }
}

fn rayon_par_mergesort(c: &mut Criterion) {
    for size in ARRAY_SIZES {
        let mut arr = black_box(get_reversed_array(size));
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

static ARRAY_SIZES: [usize; 6] = [50_000, 100_000, 500_000, 1_000_000, 2_000_000, 4_000_000];

fn small_config() -> Criterion {
    Criterion::default()
        .sample_size(10)
        .measurement_time(Duration::from_secs(40))
}

criterion_group! {
    name = merge_sorts;
    config = small_config();
    targets = /* parallel_mergesort,*/ /* seq_mergesort, */ /* threadpool_mergesort,*/ /* array_seq_mergesort, */ array_par_mergesort
    // targets = rayon_par_mergesort
}
criterion_main!(merge_sorts);
