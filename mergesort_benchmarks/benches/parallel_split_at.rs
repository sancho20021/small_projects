use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use disjoint_mut_test::disjoint_verified::{self, exec_pcell::Array, split_at::ArrayAbstraction};
use mergesort_benchmarks::{
    get_threshold,
    merge_sorts::{
        merge_sort, merge_sort_parallel, merge_sort_parallel_unchecked, merge_sort_threadpool,
        merge_sort_unchecked,
        minimalistic_sorts::{no_splits::PCell, no_splits_raw, no_splits_super_raw},
        parallel_profiled::{self, MergeTimes, Stats},
    },
};
use paste::paste;
use rand::RngCore;
use rayon::slice::ParallelSliceMut;
use std::io::Write;
use std::{
    collections::HashMap,
    fs::File,
    hint::black_box,
    sync::{Arc, Mutex},
    time::Duration,
};

pub trait BenchmarkableParallelSort {
    type ExtraState;

    fn get_name() -> &'static str;
    fn run(input: Vec<i32>, out: Vec<i32>, threshold: usize, extra_state: &mut Self::ExtraState);
}

pub fn get_input_array(size: usize) -> Vec<i32> {
    let mut rng = rand::rng();

    let mut v = vec![];
    for i in 0..size {
        v.push(rng.next_u32() as i32);
    }
    v.reverse();
    v
}

fn benchmark_parallel_sort<Sort: BenchmarkableParallelSort>(
    c: &mut Criterion,
    extra_state: &mut Sort::ExtraState,
) {
    for size in ARRAY_SIZES {
        let threshold = get_threshold(size);
        let input = get_input_array(size);
        c.bench_with_input(BenchmarkId::new(Sort::get_name(), size), &size, |b, _| {
            b.iter(|| {
                let arr = black_box(input.clone());
                let out_arr = black_box(vec![0; arr.len()]);
                Sort::run(arr, out_arr, threshold, extra_state);
            });
        });
    }
}

fn benchmark_sequential_sort<Sort: BenchmarkableParallelSort<ExtraState = ()>>(c: &mut Criterion) {
    for size in ARRAY_SIZES {
        let input = get_input_array(size);
        c.bench_with_input(
            BenchmarkId::new(format!("sequentially executed {}", Sort::get_name()), size),
            &size,
            |b, _| {
                b.iter(|| {
                    let arr = black_box(input.clone());
                    let out_arr = black_box(vec![0; arr.len()]);
                    Sort::run(arr, out_arr, size, &mut ());
                });
            },
        );
    }
}

macro_rules! bench_sequential_sort {
    ($type:ty) => {
        paste! {
            fn [<sequentially_executed_ $type>](c: &mut Criterion) {
                benchmark_sequential_sort::<$type>(c);
            }
        }
    };
}

struct ParallelMergesort;
impl BenchmarkableParallelSort for ParallelMergesort {
    type ExtraState = ();

    fn get_name() -> &'static str {
        "parallel mergesort"
    }

    fn run(mut input: Vec<i32>, mut out: Vec<i32>, threshold: usize, _: &mut ()) {
        merge_sort_parallel(&mut input, &mut out, threshold);
    }
}

fn parallel_mergesort(c: &mut Criterion) {
    benchmark_parallel_sort::<ParallelMergesort>(c, &mut ());
}

struct ParallelUnchecked;
impl BenchmarkableParallelSort for ParallelUnchecked {
    type ExtraState = ();

    fn get_name() -> &'static str {
        "parallel unchecked"
    }

    fn run(
        mut input: Vec<i32>,
        mut out: Vec<i32>,
        threshold: usize,
        extra_state: &mut Self::ExtraState,
    ) {
        merge_sort_parallel_unchecked(&mut input, &mut out, threshold);
    }
}

fn parallel_unchecked_mergesort(c: &mut Criterion) {
    benchmark_parallel_sort::<ParallelUnchecked>(c, &mut ());
}

struct ThreadPoolSort;
impl BenchmarkableParallelSort for ThreadPoolSort {
    type ExtraState = ();

    fn get_name() -> &'static str {
        "threadpool mergesort"
    }

    fn run(mut input: Vec<i32>, mut out: Vec<i32>, threshold: usize, _: &mut ()) {
        merge_sort_threadpool(&mut input, &mut out, threshold);
    }
}

fn threadpool_mergesort(c: &mut Criterion) {
    benchmark_parallel_sort::<ThreadPoolSort>(c, &mut ());
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

struct VerusParallelSort;
impl BenchmarkableParallelSort for VerusParallelSort {
    type ExtraState = ();

    fn get_name() -> &'static str {
        "verus parallel sort"
    }

    fn run(input: Vec<i32>, out: Vec<i32>, threshold: usize, _: &mut ()) {
        let mut arr = ArrayAbstraction::new(input);
        disjoint_verified::split_at::mergesort::merge_sort_parallel_abstraction(
            &mut arr, out, threshold,
        )
        .unwrap();
    }
}

fn array_par_mergesort(c: &mut Criterion) {
    benchmark_parallel_sort::<VerusParallelSort>(c, &mut ());
}


struct RayonSort;
impl BenchmarkableParallelSort for RayonSort {
    type ExtraState = ();

    fn get_name() -> &'static str {
        "rayon parallel sort"
    }

    fn run(mut input: Vec<i32>, mut out: Vec<i32>, threshold: usize, _: &mut ()) {
        input.as_mut_slice().par_sort();
    }
}

fn rayon_par_mergesort(c: &mut Criterion) {
    benchmark_parallel_sort::<RayonSort>(c, &mut ());
}

struct VerusParallelNoGhost;
impl BenchmarkableParallelSort for VerusParallelNoGhost {
    type ExtraState = ();

    fn get_name() -> &'static str {
        "verus parallel without ghost code"
    }

    fn run(input: Vec<i32>, out: Vec<i32>, threshold: usize, _: &mut ()) {
        let mut arr =
            mergesort_benchmarks::merge_sorts::verus_without_ghost::ArrayAbstraction::new(input);
        mergesort_benchmarks::merge_sorts::verus_without_ghost::mergesort::merge_sort_parallel_abstraction(
            &mut arr,
            out,
            threshold,
        )
        .unwrap();
    }
}

fn without_ghost_array_par_mergesort(c: &mut Criterion) {
    benchmark_parallel_sort::<VerusParallelNoGhost>(c, &mut ());
}

struct VerusChanged5;
impl BenchmarkableParallelSort for VerusChanged5 {
    type ExtraState = ();

    fn get_name() -> &'static str {
        "verus parallel without ghost, without maybeuninit, no arc, inlined"
    }

    fn run(input: Vec<i32>, mut out: Vec<i32>, threshold: usize, _: &mut ()) {
        let mut arr =
            mergesort_benchmarks::merge_sorts::verus_changed5::ArrayAbstraction::new(input);
        mergesort_benchmarks::merge_sorts::verus_changed5::mergesort::merge_sort_parallel_abstraction(
            &mut arr,
            out,
            threshold,
        )
        .unwrap();
    }
}

fn verus_changed5_par_mergesort(c: &mut Criterion) {
    benchmark_parallel_sort::<VerusChanged5>(c, &mut ());
}

type ProfiledState = (Mutex<Stats>, MergeTimes);

struct ParallelProfiled;
impl BenchmarkableParallelSort for ParallelProfiled {
    type ExtraState = ProfiledState;

    fn get_name() -> &'static str {
        "parallel profiled"
    }

    fn run(
        mut input: Vec<i32>,
        mut out: Vec<i32>,
        threshold: usize,
        extra_state: &mut Self::ExtraState,
    ) {
        mergesort_benchmarks::merge_sorts::parallel_profiled::merge_sort_parallel(
            &mut input,
            &mut out,
            threshold,
            &extra_state.0,
            &mut extra_state.1,
        );
    }
}

fn format_merge_times(merge_times: &MergeTimes) -> HashMap<usize, Vec<u128>> {
    merge_times
        .iter()
        .map(|(size, d)| (*size, d.iter().map(|d| d.as_micros()).collect()))
        .collect()
}

fn par_mergesort_profiled(c: &mut Criterion) {
    let mut stats = ProfiledState::default();
    benchmark_parallel_sort::<ParallelProfiled>(c, &mut stats);
    let mut file = File::create("parallel_stats.txt").unwrap();
    println!("merge_time_len={}", stats.1.len());
    writeln!(file, "{:?}", format_merge_times(&stats.1)).unwrap();
}

struct VerusNoGhostProfiled;
impl BenchmarkableParallelSort for VerusNoGhostProfiled {
    type ExtraState = ProfiledState;

    fn get_name() -> &'static str {
        "verus no ghost profiled"
    }

    fn run(input: Vec<i32>, out: Vec<i32>, threshold: usize, extra_state: &mut Self::ExtraState) {
        let mut arr =
            mergesort_benchmarks::merge_sorts::verus_without_ghost::ArrayAbstraction::new(input);
        mergesort_benchmarks::merge_sorts::verus_without_ghost_profiled::mergesort::merge_sort_parallel_abstraction(
            &mut arr, out, threshold, &extra_state.0, &mut extra_state.1
        ).unwrap();
    }
}

struct MinimalStandard;
impl BenchmarkableParallelSort for MinimalStandard {
    type ExtraState = ();

    fn get_name() -> &'static str {
        "minimal standard parallel"
    }

    fn run(mut input: Vec<i32>, mut out: Vec<i32>, threshold: usize, _: &mut Self::ExtraState) {
        mergesort_benchmarks::merge_sorts::minimalistic_sorts::standard::merge_sort_parallel(
            &mut input, &mut out, threshold,
        );
    }
}

fn minimal_standard(c: &mut Criterion) {
    benchmark_parallel_sort::<MinimalStandard>(c, &mut ());
}

bench_sequential_sort!(MinimalStandard);

struct MinimalStandardUnchecked;
impl BenchmarkableParallelSort for MinimalStandardUnchecked {
    type ExtraState = ();

    fn get_name() -> &'static str {
        "minimal standard unchecked parallel"
    }

    fn run(mut input: Vec<i32>, mut out: Vec<i32>, threshold: usize, _: &mut Self::ExtraState) {
        mergesort_benchmarks::merge_sorts::minimalistic_sorts::standard_unchecked::merge_sort_parallel(
            &mut input, &mut out, threshold,
        );
    }
}

fn minimal_standard_unchecked(c: &mut Criterion) {
    benchmark_parallel_sort::<MinimalStandardUnchecked>(c, &mut ());
}

struct MinimalVerus;
impl BenchmarkableParallelSort for MinimalVerus {
    type ExtraState = ();

    fn get_name() -> &'static str {
        "minimal verus parallel"
    }

    fn run(input: Vec<i32>, out: Vec<i32>, threshold: usize, _: &mut Self::ExtraState) {
        // LOOK: danger :) Hopefully this is not UB because I take input by value, not by reference
        // otherwise, if I took by reference, the compiler could optimize all this code away
        // try looking at godbolt functions on shared references with optimizations on
        let arr = unsafe { std::mem::transmute(input) };
        let out = unsafe { std::mem::transmute(out) };
        mergesort_benchmarks::merge_sorts::minimalistic_sorts::no_splits::merge_sort_parallel(
            &arr,
            0,
            arr.len(),
            &out,
            0,
            threshold,
        )
        .unwrap();
    }
}

fn minimal_verus(c: &mut Criterion) {
    benchmark_parallel_sort::<MinimalVerus>(c, &mut ());
}

bench_sequential_sort!(MinimalVerus);

struct MinimalVerusSlice;
impl BenchmarkableParallelSort for MinimalVerusSlice {
    type ExtraState = ();

    fn get_name() -> &'static str {
        "minimal verus with slice parallel"
    }

    fn run(input: Vec<i32>, out: Vec<i32>, threshold: usize, _: &mut Self::ExtraState) {
        let arr: Vec<PCell> = unsafe { std::mem::transmute(input) };
        let out: Vec<PCell> = unsafe { std::mem::transmute(out) };
        mergesort_benchmarks::merge_sorts::minimalistic_sorts::no_splits_slice::merge_sort_parallel(
            &arr,
            0,
            arr.len(),
            &out,
            0,
            threshold,
        )
        .unwrap();
    }
}

fn minimal_verus_slice(c: &mut Criterion) {
    benchmark_parallel_sort::<MinimalVerusSlice>(c, &mut ());
}

struct MinimalVerusRaw;
impl BenchmarkableParallelSort for MinimalVerusRaw {
    type ExtraState = ();

    fn get_name() -> &'static str {
        "minimal verus with *const parallel"
    }

    fn run(input: Vec<i32>, out: Vec<i32>, threshold: usize, _: &mut Self::ExtraState) {
        let arr: Vec<PCell> = unsafe { std::mem::transmute(input) };
        let out: Vec<PCell> = unsafe { std::mem::transmute(out) };
        mergesort_benchmarks::merge_sorts::minimalistic_sorts::no_splits_raw::merge_sort_parallel(
            no_splits_raw::Array(arr.as_ptr()),
            0,
            arr.len(),
            no_splits_raw::Array(out.as_ptr()),
            0,
            threshold,
        )
        .unwrap();
    }
}

fn minimal_verus_raw(c: &mut Criterion) {
    benchmark_parallel_sort::<MinimalVerusRaw>(c, &mut ());
}

struct MinimalVerusSliceUnwrap;
impl BenchmarkableParallelSort for MinimalVerusSliceUnwrap {
    type ExtraState = ();

    fn get_name() -> &'static str {
        "minimal verus with slice and easier spawning parallel"
    }

    fn run(input: Vec<i32>, out: Vec<i32>, threshold: usize, _: &mut Self::ExtraState) {
        let arr: Vec<PCell> = unsafe { std::mem::transmute(input) };
        let out: Vec<PCell> = unsafe { std::mem::transmute(out) };
        mergesort_benchmarks::merge_sorts::minimalistic_sorts::no_splits_slice_easier_spawning::merge_sort_parallel(
            &arr,
            0,
            arr.len(),
            &out,
            0,
            threshold,
        );
    }
}

fn minimal_verus_slice_unwrap(c: &mut Criterion) {
    benchmark_parallel_sort::<MinimalVerusSliceUnwrap>(c, &mut ());
}

struct MinimalVerusSuperRaw;
impl BenchmarkableParallelSort for MinimalVerusSuperRaw {
    type ExtraState = ();

    fn get_name() -> &'static str {
        "minimal verus with *mut no unsafecell parallel"
    }

    fn run(input: Vec<i32>, out: Vec<i32>, threshold: usize, _: &mut Self::ExtraState) {
        mergesort_benchmarks::merge_sorts::minimalistic_sorts::no_splits_super_raw::merge_sort_parallel(
            no_splits_super_raw::Array(input.as_ptr() as *mut i32),
            0,
            input.len(),
            no_splits_super_raw::Array(out.as_ptr() as *mut i32),
            0,
            threshold,
        );
    }
}

fn minimal_verus_super_raw(c: &mut Criterion) {
    benchmark_parallel_sort::<MinimalVerusSuperRaw>(c, &mut ());
}

struct MinimalVerusSuperRawLessArgs;
impl BenchmarkableParallelSort for MinimalVerusSuperRawLessArgs {
    type ExtraState = ();

    fn get_name() -> &'static str {
        "minimal verus with *mut no unsafecell, less args parallel"
    }

    fn run(input: Vec<i32>, out: Vec<i32>, threshold: usize, _: &mut Self::ExtraState) {
        mergesort_benchmarks::merge_sorts::minimalistic_sorts::no_splits_super_raw_less_args::merge_sort_parallel(
            no_splits_super_raw::Array(input.as_ptr() as *mut i32),
            0,
            input.len(),
            no_splits_super_raw::Array(out.as_ptr() as *mut i32),
            0,
            threshold,
        );
    }
}

fn minimal_verus_super_raw_less_args(c: &mut Criterion) {
    benchmark_parallel_sort::<MinimalVerusSuperRawLessArgs>(c, &mut ());
}

bench_sequential_sort!(MinimalVerusSuperRawLessArgs);

fn print_stats(stats: &Mutex<Stats>) {
    let stats = stats.lock().unwrap();
    let stats = stats
        .iter()
        .map(|(size, durs)| {
            (
                *size,
                durs.into_iter().map(|d| d.as_millis()).collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<_, _>>();
    println!("{stats:#?}");
}

fn verus_no_ghost_profiled(c: &mut Criterion) {
    let mut stats = ProfiledState::default();
    benchmark_parallel_sort::<VerusNoGhostProfiled>(c, &mut stats);

    let mut file = File::create("verus_no_ghost_stats.txt").unwrap();
    println!("merge_time_len={}", stats.1.len());
    writeln!(file, "{:?}", format_merge_times(&stats.1)).unwrap();
}

static ARRAY_SIZES: [usize; 4] = [
    // /* 50_000,*/ /* 100_000, 500_000, */ 1_000_000,
    // 100_000,
    // 1_000_000,
    // 2_000_000,
    4_000_000,
    8_000_000,
    20_000_000,
    50_000_000,
    // 100_000_000,
];

fn small_config() -> Criterion {
    Criterion::default().sample_size(10)
    .measurement_time(Duration::from_secs(60))
}

criterion_group! {
    name = merge_sorts;
    config = small_config();
    /*unchecked_seq_mergesort*//* without_ghost_array_par_mergesort, without_ghost_changed_array_par_mergesort,*/ /* verus_changed2_par_mergesort, verus_changed3_par_mergesort */
    targets =
    // parallel_mergesort, /*seq_mergesort,*/ /* threadpool_mergesort,*/ /*array_seq_mergesort,*/
    // array_par_mergesort,
    // par_mergesort_profiled,
    // verus_no_ghost_profiled,
        // verus_changed4_par_mergesort,
        // verus_changed5_par_mergesort
    // targets = rayon_par_mergesort
    // targets = unchecked_seq_mergesort
    // targets = parallel_unchecked_mergesort, rayon_par_mergesort
    // minimal_standard,
    // minimal_standard_unchecked,
    // minimal_verus_slice,
    // minimal_verus,
    // minimal_verus_raw,
    // minimal_verus_slice_unwrap,
    // minimal_verus_super_raw,
    // minimal_verus_super_raw_less_args,
    sequentially_executed_MinimalStandard,
    sequentially_executed_MinimalVerus,
    sequentially_executed_MinimalVerusSuperRawLessArgs,
}
criterion_main!(merge_sorts);
