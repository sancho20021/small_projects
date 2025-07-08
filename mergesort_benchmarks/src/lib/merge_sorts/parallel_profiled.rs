use std::{
    collections::HashMap,
    sync::Mutex,
    time::{Duration, Instant},
};

pub type Stats = HashMap<usize, Vec<Duration>>;

// if merging smaller arrays, skip time measuring
pub const MERGE_PROFILE_THRESH: usize = 100_000;

pub fn update_stats(len: usize, dur: Duration, stats: &Mutex<Stats>) {
    let mut stats = stats.lock().unwrap();
    // println!("Parallel call took {dur:?}");
    stats
        .entry(len)
        .and_modify(|times| times.push(dur))
        .or_insert(vec![dur]);
}

pub type MergeTimes = Vec<Duration>;

pub fn merge_sort_parallel<T: Ord + Send + Copy>(
    arr: &mut [T],
    out_arr: &mut [T],
    threshold: usize,
    stats: &Mutex<Stats>,
    merge_times: &mut Vec<Duration>,
) {
    let start = Instant::now();
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }

    if arr.len() <= threshold {
        merge_sort(arr, out_arr, merge_times);
        let end = Instant::now();
        update_stats(arr.len(), end - start, stats);
        return;
    }

    let (left, right) = arr.split_at_mut(mid);
    let (out_left, out_right) = out_arr.split_at_mut(mid);

    let mut merge_times_left = vec![];
    let mut merge_times_right = vec![];
    std::thread::scope(|s| {
        let left_handle = s.spawn(|| {
            merge_sort_parallel(
                &mut *left,
                out_left,
                threshold,
                stats,
                &mut merge_times_left,
            );
        });
        merge_sort_parallel(
            &mut *right,
            out_right,
            threshold,
            stats,
            &mut merge_times_right,
        );
        left_handle.join().unwrap();
    });
    merge_times.extend_from_slice(&merge_times_left);
    merge_times.extend_from_slice(&merge_times_right);

    merge(left, right, out_arr, merge_times);
    let mut i = 0;
    while i < arr.len() {
        arr[i] = out_arr[i];
        i += 1;
    }
    let end = Instant::now();
    update_stats(arr.len(), end - start, stats);
}

pub fn merge<T: Ord + Copy>(
    left: &[T],
    right: &[T],
    out: &mut [T],
    merge_times: &mut Vec<Duration>,
) {
    let start = if out.len() > MERGE_PROFILE_THRESH {
        Some(Instant::now())
    } else {
        None
    };

    let mut left_index = 0;
    let mut right_index = 0;
    let mut out_index = 0;
    while left_index < left.len() && right_index < right.len() {
        let element: T;
        if left[left_index] <= right[right_index] {
            element = left[left_index];
            left_index += 1;
        } else {
            element = right[right_index];
            right_index += 1;
        }
        out[out_index] = element;
        out_index += 1;
    }

    if left_index < left.len() {
        while left_index < left.len() {
            out[out_index] = left[left_index];
            left_index += 1;
            out_index += 1;
        }
    } else if right_index < right.len() {
        while right_index < right.len() {
            out[out_index] = right[right_index];
            right_index += 1;
            out_index += 1;
        }
    }
    if let Some(start) = start {
        let elapsed = start.elapsed();
        merge_times.push(elapsed);
    }
}

pub fn merge_sort<T: Ord + Copy>(
    arr: &mut [T],
    out_arr: &mut [T],
    merge_times: &mut Vec<Duration>,
) {
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }

    merge_sort(&mut arr[0..mid], &mut out_arr[0..mid], merge_times);
    merge_sort(&mut arr[mid..], &mut out_arr[mid..], merge_times);

    merge(&arr[0..mid], &arr[mid..], out_arr, merge_times);

    let mut i = 0;
    while i < arr.len() {
        arr[i] = out_arr[i];
        i += 1;
    }
}

mod tests {
    use std::{collections::HashMap, sync::Mutex};

    use crate::{get_threshold, merge_sorts::parallel_profiled::merge_sort_parallel};

    #[test]
    fn test() {
        let mut arr = vec![0; 1_000_000];
        let mut out_arr = vec![0; arr.len()];
        let stats = Mutex::new(HashMap::new());

        let threshold = get_threshold(arr.len());
        let mut merge_times = vec![];
        merge_sort_parallel(&mut arr, &mut out_arr, threshold, &stats, &mut merge_times);

        let stats = stats.lock().unwrap();
        println!("{stats:#?}");
        println!("merge_times len = {}", merge_times.len());
        println!("{merge_times:#?}")
    }
}
