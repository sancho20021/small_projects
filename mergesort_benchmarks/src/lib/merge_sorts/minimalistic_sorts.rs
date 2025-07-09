pub type IntSlice<'a> = &'a mut [i32];
pub type IntSliceShared<'a> = &'a [i32];

pub mod standard {
    use crate::merge_sorts::minimalistic_sorts::{IntSlice, IntSliceShared};

    pub fn merge_sort(arr: IntSlice, out_arr: IntSlice) {
        let mid = arr.len() / 2;
        if mid == 0 {
            return;
        }

        merge_sort(&mut arr[0..mid], &mut out_arr[0..mid]);
        merge_sort(&mut arr[mid..], &mut out_arr[mid..]);

        merge(&arr[0..mid], &arr[mid..], out_arr);

        let mut i = 0;
        while i < arr.len() {
            arr[i] = out_arr[i];
            i += 1;
        }
    }

    pub fn merge(left: IntSliceShared, right: IntSliceShared, out: IntSlice) {
        let mut left_index = 0;
        let mut right_index = 0;
        let mut out_index = 0;
        while left_index < left.len() && right_index < right.len() {
            let element: i32;
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
    }

    pub fn merge_sort_parallel(arr: IntSlice, out_arr: IntSlice, threshold: usize) {
        let mid = arr.len() / 2;
        if mid == 0 {
            return;
        }

        if arr.len() <= threshold {
            merge_sort(arr, out_arr);
            return;
        }

        let (left, right) = arr.split_at_mut(mid);
        let (out_left, out_right) = out_arr.split_at_mut(mid);

        std::thread::scope(|s| {
            let left_handle = s.spawn(|| {
                merge_sort_parallel(&mut *left, out_left, threshold);
            });
            merge_sort_parallel(&mut *right, out_right, threshold);
            left_handle.join().unwrap();
        });

        merge(left, right, out_arr);
        let mut i = 0;
        while i < arr.len() {
            arr[i] = out_arr[i];
            i += 1;
        }
    }
}

pub mod standard_unchecked {
    use crate::merge_sorts::minimalistic_sorts::{IntSlice, IntSliceShared};

    pub fn merge_sort(arr: IntSlice, out_arr: IntSlice) {
        let mid = arr.len() / 2;
        if mid == 0 {
            return;
        }

        merge_sort(unsafe { arr.get_unchecked_mut(0..mid) }, unsafe {
            out_arr.get_unchecked_mut(0..mid)
        });
        merge_sort(unsafe { arr.get_unchecked_mut(mid..) }, unsafe {
            out_arr.get_unchecked_mut(mid..)
        });

        merge(
            unsafe { arr.get_unchecked(0..mid) },
            unsafe { arr.get_unchecked(mid..) },
            out_arr,
        );

        let mut i = 0;
        while i < arr.len() {
            unsafe {
                *arr.get_unchecked_mut(i) = *out_arr.get_unchecked(i);
            }
            i += 1;
        }
    }

    pub fn merge(left: IntSliceShared, right: IntSliceShared, out: IntSlice) {
        let mut left_index = 0;
        let mut right_index = 0;
        let mut out_index = 0;
        while left_index < left.len() && right_index < right.len() {
            let element: i32;
            unsafe {
                if *left.get_unchecked(left_index) <= *right.get_unchecked(right_index) {
                    element = *left.get_unchecked(left_index);
                    left_index += 1;
                } else {
                    element = *right.get_unchecked(right_index);
                    right_index += 1;
                }
                *out.get_unchecked_mut(out_index) = element;
            }
            out_index += 1;
        }

        if left_index < left.len() {
            while left_index < left.len() {
                unsafe {
                    *out.get_unchecked_mut(out_index) = *left.get_unchecked(left_index);
                }
                left_index += 1;
                out_index += 1;
            }
        } else if right_index < right.len() {
            while right_index < right.len() {
                unsafe {
                    *out.get_unchecked_mut(out_index) = *right.get_unchecked(right_index);
                }
                right_index += 1;
                out_index += 1;
            }
        }
    }

    pub fn merge_sort_parallel(arr: IntSlice, out_arr: IntSlice, threshold: usize) {
        let mid = arr.len() / 2;
        if mid == 0 {
            return;
        }

        if arr.len() <= threshold {
            merge_sort(arr, out_arr);
            return;
        }

        let (left, right) = unsafe { arr.split_at_mut_unchecked(mid) };
        let (out_left, out_right) = unsafe { out_arr.split_at_mut_unchecked(mid) };

        std::thread::scope(|s| {
            let left_handle = s.spawn(|| {
                merge_sort_parallel(&mut *left, out_left, threshold);
            });
            merge_sort_parallel(&mut *right, out_right, threshold);
            left_handle.join().unwrap();
        });

        merge(left, right, out_arr);
        let mut i = 0;
        while i < arr.len() {
            unsafe {
                *arr.get_unchecked_mut(i) = *out_arr.get_unchecked(i);
            }
            i += 1;
        }
    }
}

pub mod no_splits {
    use std::cell::UnsafeCell;

    pub struct PCell(UnsafeCell<i32>);

    unsafe impl Send for PCell {}
    unsafe impl Sync for PCell {}

    #[inline(always)]
    pub fn set(aself: &Vec<PCell>, i: usize, x: i32) {
        unsafe {
            let ai = aself.get_unchecked(i);
            *ai.0.get() = x;
        }
    }

    #[inline(always)]
    pub fn read(aself: &Vec<PCell>, i: usize) -> i32 {
        unsafe { *aself.get_unchecked(i).0.get() }
    }

    fn merge(
        array: &Vec<PCell>,
        mut left_lo: usize, // look, these are the same pairs of (lo, hi) that would be stored in a slice otherwise
        left_hi: usize,
        mut right_lo: usize,
        right_hi: usize,
        out_array: &Vec<PCell>,
        mut out_lo: usize,
    ) {
        while left_lo < left_hi && right_lo < right_hi {
            let element: i32;
            if read(array, left_lo) < read(array, right_lo) {
                element = read(array, left_lo);
                left_lo += 1;
            } else {
                element = read(array, right_lo);
                right_lo += 1;
            }
            set(out_array, out_lo, element);
            out_lo += 1;
        }
        if left_lo < left_hi {
            while left_lo < left_hi {
                let e = read(array, left_lo);
                set(out_array, out_lo, e);
                left_lo += 1;
                out_lo += 1;
            }
        } else if right_lo < right_hi {
            while right_lo < right_hi {
                let e = read(array, right_lo);
                set(out_array, out_lo, e);
                right_lo += 1;
                out_lo += 1;
            }
        }
    }

    fn merge_sort(
        arr: &Vec<PCell>,
        mut lo: usize,
        hi: usize,
        out_arr: &Vec<PCell>,
        mut out_lo: usize,
    ) {
        let mid = lo + (hi - lo) / 2;
        if mid == lo {
            return;
        }
        merge_sort(arr, lo, mid, out_arr, out_lo);
        merge_sort(arr, mid, hi, out_arr, out_lo);
        merge(arr, lo, mid, mid, hi, out_arr, out_lo);
        while lo < hi {
            let e = read(out_arr, out_lo);
            set(arr, lo, e);
            out_lo += 1;
            lo += 1;
        }
    }

    pub fn merge_sort_parallel(
        arr: &Vec<PCell>,
        mut lo: usize,
        hi: usize,
        out_arr: &Vec<PCell>,
        mut out_lo: usize,
        threshold: usize,
    ) -> Result<(), &'static str> {
        let mid = lo + (hi - lo) / 2;
        let out_mid = out_lo + (hi - lo) / 2;
        if mid == lo {
            return Ok(());
        }
        if hi - lo <= threshold {
            merge_sort(arr, lo, hi, out_arr, out_lo);
            return Ok(());
        }
        std::thread::scope(|scope| {
            let left_perms = scope.spawn(move || -> Result<(), ()> {
                let t = merge_sort_parallel(arr, lo, mid, out_arr, out_lo, threshold);
                if t.is_err() {
                    return Err(());
                } else {
                    Ok(())
                }
            });
            match merge_sort_parallel(arr, mid, hi, out_arr, out_mid, threshold) {
                Ok(()) => {}
                Err(_) => {
                    return Result::Err("error while joining threads");
                }
            };
            match left_perms.join() {
                Result::Ok(Ok(_)) => Ok(()),
                _ => {
                    return Result::Err("error while joining threads");
                }
            }
        })?;
        merge(&arr, lo, mid, mid, hi, &out_arr, out_lo);
        while lo < hi {
            let e = read(out_arr, out_lo);
            set(arr, lo, e);
            out_lo += 1;
            lo += 1;
        }
        Ok(())
    }
}
