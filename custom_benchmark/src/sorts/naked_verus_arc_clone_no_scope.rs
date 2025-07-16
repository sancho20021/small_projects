use std::sync::Arc;

use crate::sorts::Element;

#[derive(Clone, Copy)]
pub struct Array(pub *mut i32);

unsafe impl Send for Array {}
unsafe impl Sync for Array {}

impl Array {
    #[inline(always)]
    fn set(self, i: usize, x: i32) {
        unsafe {
            *self.0.add(i) = x;
        }
    }

    #[inline(always)]
    fn read(self: Array, i: usize) -> i32 {
        unsafe { *self.0.add(i) }
    }
}

pub fn copy(from: Array, mut from_lo: usize, from_hi: usize, to: Array, mut to_lo: usize) {
    while from_lo < from_hi {
        to.set(to_lo, from.read(from_lo));
        from_lo += 1;
        to_lo += 1;
    }
}

pub fn merge(
    array: Array,
    mut left_lo: usize,
    mut right_lo: usize,
    right_hi: usize,
    helper_buf: Array,
    mut helper_buf_lo: usize,
) {
    let left_hi = right_lo;
    while left_lo < left_hi && right_lo < right_hi {
        let element: i32;
        if array.read(left_lo) < array.read(right_lo) {
            element = array.read(left_lo);
            left_lo += 1;
        } else {
            element = array.read(right_lo);
            right_lo += 1;
        }
        helper_buf.set(helper_buf_lo, element);
        helper_buf_lo += 1;
    }
    copy(array, left_lo, left_hi, helper_buf, helper_buf_lo);
    helper_buf_lo += left_hi - left_lo;
    copy(array, right_lo, right_hi, helper_buf, helper_buf_lo);
}

pub fn merge_sort(arr: Array, lo: usize, hi: usize, helper_buf: Array) {
    let mid = lo + (hi - lo) / 2;
    if mid == lo {
        return;
    }
    merge_sort(arr, lo, mid, helper_buf);
    merge_sort(arr, mid, hi, helper_buf);
    merge(arr, lo, mid, hi, helper_buf, lo);
    copy(helper_buf, lo, hi, arr, lo);
}

pub fn _merge_sort_parallel(
    arr: Arc<Array>,
    lo: usize,
    hi: usize,
    helper_buf: Arc<Array>,
    threshold: usize,
) -> Result<(), ()> {
    let mid = lo + (hi - lo) / 2;
    if mid == lo {
        return Ok(());
    }
    if hi - lo <= threshold {
        merge_sort(*arr, lo, hi, *helper_buf);
        return Ok(());
    }

    let arr_r1 = Arc::clone(&arr);
    let arr_r2 = Arc::clone(&arr);

    let helper_buf_r1 = Arc::clone(&helper_buf);
    let helper_buf_r2 = Arc::clone(&helper_buf);

    let left_perms = std::thread::spawn(move || {
        let t = _merge_sort_parallel(arr_r1, lo, mid, helper_buf_r1, threshold);
        if t.is_err() { Err(()) } else { Ok(()) }
    });
    match _merge_sort_parallel(arr_r2, mid, hi, helper_buf_r2, threshold) {
        Ok(_) => {}
        Err(_) => {
            return Err(());
        }
    };
    match left_perms.join() {
        Ok(Ok(())) => {}
        _ => return Err(()),
    };
    merge(*arr, lo, mid, hi, *helper_buf, lo);
    copy(*helper_buf, lo, hi, *arr, lo);
    Ok(())
}
