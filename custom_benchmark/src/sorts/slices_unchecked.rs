use crate::sorts::{Element};

pub fn copy(from: &[i32], mut from_lo: usize, from_hi: usize, to: &mut [i32], mut to_lo: usize) {
    while from_lo < from_hi {
        unsafe {
            *to.get_unchecked_mut(to_lo) = *from.get_unchecked(from_lo);
        }
        from_lo += 1;
        to_lo += 1;
    }
}

pub fn merge(left: &[i32], right: &[i32], helper_buf: &mut [i32]) {
    let mut left_i = 0;
    let mut right_i = 0;
    let mut helper_buf_i = 0;
    while left_i < left.len() && right_i < right.len() {
        let element: i32;
        unsafe {
            if *left.get_unchecked(left_i) <= *right.get_unchecked(right_i) {
                element = *left.get_unchecked(left_i);
                left_i += 1;
            } else {
                element = *right.get_unchecked(right_i);
                right_i += 1;
            }
            *helper_buf.get_unchecked_mut(helper_buf_i) = element;
        }
        helper_buf_i += 1;
    }
    copy(left, left_i, left.len(), helper_buf, helper_buf_i);
    helper_buf_i += left.len() - left_i;
    copy(right, right_i, right.len(), helper_buf, helper_buf_i);
}

pub fn merge_sort(arr: &mut [i32], helper_buf: &mut [i32]) {
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }
    merge_sort(unsafe { arr.get_unchecked_mut(0..mid) }, unsafe {
        helper_buf.get_unchecked_mut(0..mid)
    });
    merge_sort(unsafe { arr.get_unchecked_mut(mid..) }, unsafe {
        helper_buf.get_unchecked_mut(mid..)
    });
    merge(
        unsafe { arr.get_unchecked(0..mid) },
        unsafe { arr.get_unchecked(mid..) },
        helper_buf,
    );
    copy(helper_buf, 0, helper_buf.len(), arr, 0);
}

pub fn _merge_sort_parallel(
    arr: &mut [i32],
    helper_buf: &mut [i32],
    threshold: usize,
) -> Result<(), ()> {
    let mid = arr.len() / 2;
    if mid == 0 {
        return Ok(());
    }
    if arr.len() <= threshold {
        merge_sort(arr, helper_buf);
        return Ok(());
    }
    let (left, right) = unsafe { arr.split_at_mut_unchecked(mid) };
    let (helper_buf_left, helper_buf_right) = unsafe { helper_buf.split_at_mut_unchecked(mid) };
    std::thread::scope(|s| {
        let left_handle = s.spawn(|| {
            let t = _merge_sort_parallel(&mut *left, helper_buf_left, threshold);
            if t.is_err() { Err(()) } else { Ok(()) }
        });
        match _merge_sort_parallel(&mut *right, helper_buf_right, threshold) {
            Ok(_) => {}
            Err(_) => {
                return Err(());
            }
        };
        match left_handle.join() {
            Ok(Ok(())) => {}
            _ => return Err(()),
        };
        Ok(())
    })?;
    merge(left, right, helper_buf);
    copy(helper_buf, 0, helper_buf.len(), arr, 0);
    Ok(())
}
