use crate::sorts::{Element, Sort};

fn copy(from: &[i32], mut from_lo: usize, from_hi: usize, to: &mut [i32], mut to_lo: usize) {
    while from_lo < from_hi {
        unsafe {
            *to.get_unchecked_mut(to_lo) = *from.get_unchecked(from_lo);
        }
        from_lo += 1;
        to_lo += 1;
    }
}

fn merge(left: &[i32], right: &[i32], helper_buf: &mut [i32]) {
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

fn merge_sort(arr: &mut [i32], helper_buf: &mut [i32]) {
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

fn _merge_sort_parallel(arr: &mut [i32], helper_buf: &mut [i32], threshold: usize) {
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }
    if arr.len() <= threshold {
        merge_sort(arr, helper_buf);
        return;
    }
    let (left, right) = unsafe { arr.split_at_mut_unchecked(mid) };
    let (out_left, out_right) = unsafe { helper_buf.split_at_mut_unchecked(mid) };
    std::thread::scope(|s| {
        let left_handle = s.spawn(|| {
            _merge_sort_parallel(&mut *left, out_left, threshold);
        });
        _merge_sort_parallel(&mut *right, out_right, threshold);
        left_handle.join().unwrap();
    });
    merge(left, right, helper_buf);
    copy(helper_buf, 0, helper_buf.len(), arr, 0);
}

pub struct SlicesUnchecked;
impl Sort for SlicesUnchecked {
    type Array = Vec<Element>;

    fn prepare_array(input: Vec<Element>) -> (Self::Array, Self::Array) {
        let buf = vec![0; input.len()];
        (input, buf)
    }

    fn sort(input: &mut Self::Array, buf: &mut Self::Array) {
        merge_sort(input, buf);
    }

    fn sort_parallel(input: &mut Self::Array, buf: &mut Self::Array, threshold: usize) {
        _merge_sort_parallel(input, buf, threshold);
    }

    fn name() -> &'static str {
        "slices unchecked"
    }
}

#[test]
fn test() {
    let mut a = vec![2, 3, 5, 1, 4];
    let mut buf = vec![0; 5];
    _merge_sort_parallel(&mut a, &mut buf, 2);
    assert_eq!(a, vec![1, 2, 3, 4, 5]);
}
