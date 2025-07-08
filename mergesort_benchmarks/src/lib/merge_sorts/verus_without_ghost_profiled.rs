pub mod mergesort {
    use crate::merge_sorts::{
        parallel_profiled::{update_stats, MergeTimes, Stats, MERGE_PROFILE_THRESH},
        verus_without_ghost::{exec_pcell::Array, region_array, ArrayAbstraction},
    };

    use super::*;
    use std::{
        sync::{Arc, Mutex},
        time::Instant,
    };
    fn merge(
        array: &Array<i32>,
        mut left_lo: usize,
        left_hi: usize,
        mut right_lo: usize,
        right_hi: usize,
        out_array: &Array<i32>,
        mut out_lo: usize,
        merge_times: &mut MergeTimes,
    ) {
        let start = if left_hi - left_lo + right_hi - right_lo > MERGE_PROFILE_THRESH {
            Some(Instant::now())
        } else {
            None
        };

        while left_lo < left_hi && right_lo < right_hi {
            let element: i32;
            if region_array::read(array, left_lo) < region_array::read(array, right_lo) {
                element = *region_array::read(array, left_lo);
                left_lo += 1;
            } else {
                element = *region_array::read(array, right_lo);
                right_lo += 1;
            }
            region_array::replace(out_array, out_lo, element);
            out_lo += 1;
        }
        if left_lo < left_hi {
            while left_lo < left_hi {
                let e = *region_array::read(array, left_lo);
                region_array::replace(out_array, out_lo, e);
                left_lo += 1;
                out_lo += 1;
            }
        } else if right_lo < right_hi {
            while right_lo < right_hi {
                let e = *region_array::read(array, right_lo);
                region_array::replace(out_array, out_lo, e);
                right_lo += 1;
                out_lo += 1;
            }
        }
        if let Some(start) = start {
            merge_times.push(start.elapsed());
        }
    }
    pub fn merge_sort_abstraction(
        arr: &mut ArrayAbstraction<i32>,
        out_arr: Vec<i32>,
        merge_times: &mut MergeTimes,
    ) {
        let out_arr = region_array::new(out_arr);
        merge_sort(
            &arr.array,
            0,
            (&*arr.array).length(),
            &out_arr,
            0,
            merge_times,
        )
    }
    fn merge_sort(
        arr: &Array<i32>,
        mut lo: usize,
        hi: usize,
        out_arr: &Array<i32>,
        mut out_lo: usize,
        merge_times: &mut MergeTimes,
    ) {
        let mid = lo + (hi - lo) / 2;
        if mid == lo {
            return;
        }
        merge_sort(arr, lo, mid, out_arr, out_lo, merge_times);
        merge_sort(arr, mid, hi, out_arr, out_lo, merge_times);
        merge(arr, lo, mid, mid, hi, out_arr, out_lo, merge_times);
        while lo < hi {
            let e = *region_array::read(out_arr, out_lo);
            region_array::replace(arr, lo, e);
            out_lo += 1;
            lo += 1;
        }
    }
    pub fn merge_sort_parallel_abstraction(
        arr: &mut ArrayAbstraction<i32>,
        out_arr: Vec<i32>,
        threshold: usize,
        stats: &Mutex<Stats>,
        merge_times: &mut MergeTimes,
    ) -> Result<(), &'static str> {
        let out_arr = region_array::new(out_arr);
        merge_sort_parallel(
            Arc::clone(&arr.array),
            0,
            (&*arr.array).length(),
            Arc::new(out_arr),
            0,
            threshold,
            stats,
            merge_times,
        )
    }
    fn merge_sort_parallel(
        arr: Arc<Array<i32>>,
        mut lo: usize,
        hi: usize,
        out_arr: Arc<Array<i32>>,
        mut out_lo: usize,
        threshold: usize,
        stats: &Mutex<Stats>,
        merge_times: &mut MergeTimes,
    ) -> Result<(), &'static str> {
        let old_lo = lo;
        let start = Instant::now();

        let mid = lo + (hi - lo) / 2;
        let out_mid = out_lo + (hi - lo) / 2;
        if mid == lo {
            return Ok(());
        }
        if hi - lo <= threshold {
            merge_sort(&*arr, lo, hi, &*out_arr, out_lo, merge_times);
            let end = Instant::now();
            update_stats(hi - lo, end - start, stats);
            return Ok(());
        }
        let arr_r1 = Arc::clone(&arr);
        let arr_r2 = Arc::clone(&arr);
        let out_arr_r1 = Arc::clone(&out_arr);
        let out_arr_r2 = Arc::clone(&out_arr);
        let mut merge_times_left = vec![];
        let merge_times_left_r = &mut merge_times_left;
        let mut merge_times_right = vec![];
        std::thread::scope(|scope| {
            let left_perms = scope.spawn(move || -> Result<(), ()> {
                let t = merge_sort_parallel(
                    arr_r1,
                    lo,
                    mid,
                    out_arr_r1,
                    out_lo,
                    threshold,
                    stats,
                    merge_times_left_r,
                );
                if t.is_err() {
                    return Err(());
                } else {
                    Ok(())
                }
            });
            match merge_sort_parallel(
                arr_r2,
                mid,
                hi,
                out_arr_r2,
                out_mid,
                threshold,
                stats,
                &mut merge_times_right,
            ) {
                Ok(()) => {}
                Err(_) => {
                    return Result::Err("error while joining threads");
                }
            };
            let left_perms = left_perms.join();
            match left_perms {
                Result::Ok(Ok(l)) => Ok(()),
                _ => {
                    return Result::Err("error while joining threads");
                }
            }
        })?;
        merge_times.extend_from_slice(&merge_times_left);
        merge_times.extend_from_slice(&merge_times_right);
        merge(&arr, lo, mid, mid, hi, &out_arr, out_lo, merge_times);
        while lo < hi {
            let e = *region_array::read(&*out_arr, out_lo);
            region_array::replace(&*arr, lo, e);
            out_lo += 1;
            lo += 1;
        }
        let end = Instant::now();
        update_stats(hi - old_lo, end - start, stats);
        Ok(())
    }
}

mod tests {
    use std::sync::Mutex;

    use crate::{
        get_threshold,
        merge_sorts::{
            parallel_profiled::Stats,
            verus_without_ghost::{exec_pcell::Array, ArrayAbstraction},
            verus_without_ghost_profiled::mergesort::merge_sort_parallel_abstraction,
        },
    };

    #[test]
    fn test() {
        let mut arr = vec![0; 1_000_000];
        let mut out_arr = vec![0; arr.len()];

        let threshold = get_threshold(arr.len());
        let mut arr = ArrayAbstraction::new(arr);
        let mut state = (Mutex::new(Stats::new()), vec![]);
        merge_sort_parallel_abstraction(&mut arr, out_arr, threshold, &state.0, &mut state.1)
            .unwrap();
        let vec = arr.clone_to_vec();

        println!("{:#?}", state.1);
    }
}
