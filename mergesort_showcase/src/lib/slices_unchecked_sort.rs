fn merge(left: &[i32], right: &[i32], out: &mut [i32]) {
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

fn merge_sort(arr: &mut [i32], out_arr: &mut [i32]) {
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

fn _merge_sort_parallel(arr: &mut [i32], out_arr: &mut [i32], threshold: usize) {
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
            _merge_sort_parallel(&mut *left, out_left, threshold);
        });
        _merge_sort_parallel(&mut *right, out_right, threshold);
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

pub fn merge_sort_parallel(arr: &mut [i32], threshold: usize) {
    let mut helper_buf = vec![0; arr.len()];
    _merge_sort_parallel(arr, &mut helper_buf, threshold);
}
