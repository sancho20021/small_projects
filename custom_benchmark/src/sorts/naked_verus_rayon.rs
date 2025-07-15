use crate::sorts::naked_verus::{Array, copy, merge, merge_sort};

pub fn _merge_sort_parallel(arr: Array, lo: usize, hi: usize, helper_buf: Array) -> Result<(), ()> {
    let mid = lo + (hi - lo) / 2;
    if mid == lo {
        return Ok(());
    }
    if hi - lo <= 2024 {
        merge_sort(arr, lo, hi, helper_buf);
        return Ok(());
    }
    let (res1, res2) = rayon::join(
        || {
            let t = _merge_sort_parallel(arr, lo, mid, helper_buf);
            if t.is_err() { Err(()) } else { Ok(()) }
        },
        || {
            let t = _merge_sort_parallel(arr, mid, hi, helper_buf);
            if t.is_err() { Err(()) } else { Ok(()) }
        },
    );
    res1?;
    res2?;
    merge(arr, lo, mid, hi, helper_buf, lo);
    copy(helper_buf, lo, hi, arr, lo);
    Ok(())
}
