use std::{
    num::{NonZero, NonZeroUsize},
    sync::Mutex,
};

use mergesort_benchmarks::{
    get_threshold,
    merge_sorts::{merge, merge_sort},
};

pub fn merge_sort_parallel<T: Ord + Send + Copy>(
    arr: &mut [T],
    out_arr: &mut [T],
    threshold: usize,
    threads_count: &Mutex<usize>,
    max_threads: &Mutex<usize>,
) {
    {
        let mut max_threads = max_threads.lock().unwrap();
        let threads_count_n = *threads_count.lock().unwrap();
        *max_threads = std::cmp::max(*max_threads, threads_count_n);
    }
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }

    if arr.len() < threshold {
        merge_sort(arr, out_arr);
        return;
    }

    let (left, right) = arr.split_at_mut(mid);
    let (out_left, out_right) = out_arr.split_at_mut(mid);

    std::thread::scope(|s| {
        *threads_count.lock().unwrap() += 1;
        let left_handle = s.spawn(|| {
            merge_sort_parallel(&mut *left, out_left, threshold, threads_count, max_threads);
        });
        merge_sort_parallel(
            &mut *right,
            out_right,
            threshold,
            threads_count,
            max_threads,
        );
        left_handle.join().unwrap();
        *threads_count.lock().unwrap() -= 1;
    });

    merge(left, right, out_arr);
    let mut i = 0;
    while i < arr.len() {
        arr[i] = out_arr[i];
        i += 1;
    }
}

fn test_big_array() {
    let mut arr = vec![0; 100_000_000];
    let mut out_arr = vec![0; arr.len()];
    let threadsn = Mutex::new(0);
    let maxthreads = Mutex::new(0);
    let threshold = get_threshold(arr.len());
    println!("threshold = {threshold}");
    merge_sort_parallel(&mut arr, &mut out_arr, threshold, &threadsn, &maxthreads);

    assert_eq!(*threadsn.lock().unwrap(), 0);
    println!("maxthreads = {}", *maxthreads.lock().unwrap());
}

fn main() {
    let n = std::thread::available_parallelism().unwrap();
    println!("{n}");
    test_big_array();
}
