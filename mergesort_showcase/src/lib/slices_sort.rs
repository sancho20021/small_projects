fn merge(left: &[i32], right: &[i32], helper_buf: &mut [i32]) {
    let mut left_index = 0;
    let mut right_index = 0;
    let mut helper_buf_index = 0;
    while left_index < left.len() && right_index < right.len() {
        let element: i32;
        if left[left_index] <= right[right_index] {
            element = left[left_index];
            left_index += 1;
        } else {
            element = right[right_index];
            right_index += 1;
        }
        helper_buf[helper_buf_index] = element;
        helper_buf_index += 1;
    }
    while left_index < left.len() {
        helper_buf[helper_buf_index] = left[left_index];
        left_index += 1;
        helper_buf_index += 1;
    }
    while right_index < right.len() {
        helper_buf[helper_buf_index] = right[right_index];
        right_index += 1;
        helper_buf_index += 1;
    }
}

fn merge_sort(arr: &mut [i32], helper_buf: &mut [i32]) {
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }
    merge_sort(&mut arr[0..mid], &mut helper_buf[0..mid]);
    merge_sort(&mut arr[mid..], &mut helper_buf[mid..]);
    merge(&arr[0..mid], &arr[mid..], helper_buf);
    let mut i = 0;
    while i < arr.len() {
        arr[i] = helper_buf[i];
        i += 1;
    }
}

fn _merge_sort_parallel(arr: &mut [i32], helper_buf: &mut [i32], threshold: usize) {
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }
    if arr.len() <= threshold {
        merge_sort(arr, helper_buf);
        return;
    }
    let (left, right) = arr.split_at_mut(mid);
    let (helper_buf_left, helper_buf_right) = helper_buf.split_at_mut(mid);
    std::thread::scope(|s| {
        let left_handle = s.spawn(|| {
            _merge_sort_parallel(&mut *left, helper_buf_left, threshold);
        });
        _merge_sort_parallel(&mut *right, helper_buf_right, threshold);
        left_handle.join().unwrap();
    });
    merge(left, right, helper_buf);
    let mut i = 0;
    while i < arr.len() {
        arr[i] = helper_buf[i];
        i += 1;
    }
}

pub fn merge_sort_parallel(arr: &mut [i32], threshold: usize) {
    let mut helper_buf = vec![0; arr.len()];
    _merge_sort_parallel(arr, &mut helper_buf, threshold);
}
