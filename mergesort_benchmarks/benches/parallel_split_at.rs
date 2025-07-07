use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use disjoint_mut_test::disjoint_verified::{self, exec_pcell::Array, split_at::ArrayAbstraction};
use mergesort_benchmarks::{
    get_threshold,
    merge_sorts::{
        merge_sort, merge_sort_parallel, merge_sort_parallel_unchecked, merge_sort_threadpool,
        merge_sort_unchecked, parallel_profiled::Stats,
    },
};
use rand::RngCore;
use rayon::slice::ParallelSliceMut;
use std::{
    hint::black_box,
    sync::{Arc, Mutex},
    time::Duration,
};

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
        let threshold = get_threshold(size);
        c.bench_with_input(
            BenchmarkId::new("parallel_mergesort", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let mut arr = black_box(get_input_array(size));
                    let mut out_arr = black_box(vec![0; arr.len()]);
                    merge_sort_parallel(
                        black_box(&mut arr),
                        black_box(&mut out_arr),
                        black_box(threshold),
                    );
                })
            },
        );
    }
}

fn parallel_unchecked_mergesort(c: &mut Criterion) {
    for size in ARRAY_SIZES {
        let threshold = get_threshold(size);
        c.bench_with_input(
            BenchmarkId::new("parallel_mergesort_unchecked", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let mut arr = black_box(get_input_array(size));
                    let mut out_arr = black_box(vec![0; arr.len()]);
                    merge_sort_parallel_unchecked(
                        black_box(&mut arr),
                        black_box(&mut out_arr),
                        black_box(threshold),
                    );
                })
            },
        );
    }
}

fn threadpool_mergesort(c: &mut Criterion) {
    for size in ARRAY_SIZES {
        let threshold = get_threshold(size);
        c.bench_with_input(
            BenchmarkId::new("threadpool_mergesort", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let mut arr = black_box(get_input_array(size));
                    let mut out_arr = black_box(vec![0; arr.len()]);
                    merge_sort_threadpool(
                        black_box(&mut arr),
                        black_box(&mut out_arr),
                        black_box(threshold),
                    );
                })
            },
        );
    }
}

fn seq_mergesort(c: &mut Criterion) {
    for size in ARRAY_SIZES {
        c.bench_with_input(BenchmarkId::new("seq_mergesort", size), &size, |b, _| {
            b.iter(|| {
                let mut arr = black_box(get_input_array(size));
                let mut out_arr = black_box(vec![0; arr.len()]);
                merge_sort(black_box(&mut arr), black_box(&mut out_arr));
            });
        });
    }
}

fn unchecked_seq_mergesort(c: &mut Criterion) {
    for size in ARRAY_SIZES {
        c.bench_with_input(
            BenchmarkId::new("unchecked_seq_mergesort", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let mut arr = black_box(get_input_array(size));
                    let mut out_arr = vec![0; arr.len()];
                    merge_sort_unchecked(black_box(&mut arr), black_box(&mut out_arr));
                });
            },
        );
    }
}

fn array_seq_mergesort(c: &mut Criterion) {
    for size in ARRAY_SIZES {
        c.bench_with_input(
            BenchmarkId::new("array_seq_mergesort", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let arr = black_box(get_input_array(size));
                    let mut arr = ArrayAbstraction::new(arr);
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
        let threshold = get_threshold(size);
        c.bench_with_input(
            BenchmarkId::new("array_par_mergesort", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let arr = black_box(get_input_array(size));
                    let mut arr = ArrayAbstraction::new(arr);
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
        c.bench_with_input(
            BenchmarkId::new("rayon_par_mergesort", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let mut arr = black_box(get_input_array(size));
                    let arr = black_box(&mut arr);
                    arr.as_mut_slice().par_sort();
                    black_box(arr);
                });
            },
        );
    }
}

fn without_ghost_array_par_mergesort(c: &mut Criterion) {
    for size in ARRAY_SIZES {
        let threshold = get_threshold(size);
        c.bench_with_input(
            BenchmarkId::new("without_ghost_array_par_mergesort", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let arr = black_box(get_input_array(size));
        let mut arr =
            mergesort_benchmarks::merge_sorts::verus_without_ghost::ArrayAbstraction::new(arr);
                    let out_array = vec![0; arr.array.length()];
                    mergesort_benchmarks::merge_sorts::verus_without_ghost::mergesort::merge_sort_parallel_abstraction(
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

fn without_ghost_changed_array_par_mergesort(c: &mut Criterion) {
    for size in ARRAY_SIZES {
        let threshold = get_threshold(size);
        c.bench_with_input(
            BenchmarkId::new("without_ghost_changed_array_par_mergesort", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let arr = black_box(get_input_array(size));
                    let mut arr =
                        mergesort_benchmarks::merge_sorts::verus_without_ghost_changed::ArrayAbstraction::new(
                            arr,
                        );
                    let out_array = vec![0; arr.array.length()];
                    mergesort_benchmarks::merge_sorts::verus_without_ghost_changed::mergesort::merge_sort_parallel_abstraction(
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

fn verus_changed2_par_mergesort(c: &mut Criterion) {
    for size in ARRAY_SIZES {
        let threshold = get_threshold(size);
        c.bench_with_input(
            BenchmarkId::new("verus_no_ghost_no_maybeuninit_no_arc", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let arr = black_box(get_input_array(size));
                    let mut arr = mergesort_benchmarks::merge_sorts::verus_changed2::ArrayAbstraction::new(arr);
                    let out_array = vec![0; arr.array.length()];
                    mergesort_benchmarks::merge_sorts::verus_changed2::mergesort::merge_sort_parallel_abstraction(
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

fn verus_changed3_par_mergesort(c: &mut Criterion) {
    for size in ARRAY_SIZES {
        let threshold = get_threshold(size);
        c.bench_with_input(
            BenchmarkId::new("verus_no_ghost_no_maybeuninit_no_arc_no_unsafecell", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let arr = black_box(get_input_array(size));
                    let mut arr = mergesort_benchmarks::merge_sorts::verus_changed3::ArrayAbstraction::new(arr);
                    let out_array = vec![0; arr.array.length()];
                    mergesort_benchmarks::merge_sorts::verus_changed3::mergesort::merge_sort_parallel_abstraction(
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

fn verus_changed5_par_mergesort(c: &mut Criterion) {
    for size in ARRAY_SIZES {
        let threshold = get_threshold(size);
        c.bench_with_input(
            BenchmarkId::new("verus_no_ghost_no_maybeuninit_no_arc_inlined", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let arr = black_box(get_input_array(size));
                    let mut arr = mergesort_benchmarks::merge_sorts::verus_changed5::ArrayAbstraction::new(arr);
                    let out_array = vec![0; arr.array.length()];
                    mergesort_benchmarks::merge_sorts::verus_changed5::mergesort::merge_sort_parallel_abstraction(
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

fn par_mergesort_profiled(c: &mut Criterion) {
    let stats = Mutex::new(Stats::new());
    for size in ARRAY_SIZES {
        let threshold = get_threshold(size);
        c.bench_with_input(
            BenchmarkId::new("verus_no_ghost_no_maybeuninit_no_arc_inlined", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let mut arr = black_box(get_input_array(size));
                    let mut out_array = vec![0; arr.len()];
                    mergesort_benchmarks::merge_sorts::parallel_profiled::merge_sort_parallel(
                        &mut arr,
                        black_box(&mut out_array),
                        threshold,
                        &stats,
                    );
                });
            },
        );
    }
}

static ARRAY_SIZES: [usize; 1] = [
    // /* 50_000,*/ /* 100_000, 500_000, */ 1_000_000,
    // 2_000_000,
    // 4_000_000,
    // 8_000_000,
    // 20_000_000,
    50_000_000,
    // 100_000_000,
];

fn small_config() -> Criterion {
    Criterion::default()
        .sample_size(10)
        .measurement_time(Duration::from_secs(40))
}

criterion_group! {
    name = merge_sorts;
    config = small_config();
    /*unchecked_seq_mergesort*//* without_ghost_array_par_mergesort, without_ghost_changed_array_par_mergesort,*/ /* verus_changed2_par_mergesort, verus_changed3_par_mergesort */
    targets = parallel_mergesort, /*seq_mergesort,*/ /* threadpool_mergesort,*/ /*array_seq_mergesort,*/
    array_par_mergesort,
        // verus_changed4_par_mergesort,
        // verus_changed5_par_mergesort
    // targets = rayon_par_mergesort
    // targets = unchecked_seq_mergesort
    // targets = parallel_unchecked_mergesort, rayon_par_mergesort
}
criterion_main!(merge_sorts);
