use crate::Sort;

#[derive(Clone, Copy)]
struct Array(pub *mut i32);

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

fn copy(from: Array, mut from_lo: usize, from_hi: usize, to: Array, mut to_lo: usize) {
    while from_lo < from_hi {
        to.set(to_lo, from.read(from_lo));
        from_lo += 1;
        to_lo += 1;
    }
}

fn merge(
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

fn merge_sort(arr: Array, lo: usize, hi: usize, helper_buf: Array) {
    let mid = lo + (hi - lo) / 2;
    if mid == lo {
        return;
    }
    merge_sort(arr, lo, mid, helper_buf);
    merge_sort(arr, mid, hi, helper_buf);
    merge(arr, lo, mid, hi, helper_buf, lo);
    copy(helper_buf, lo, hi, arr, lo);
}

fn _merge_sort_parallel(arr: Array, lo: usize, hi: usize, helper_buf: Array, threshold: usize) {
    let mid = lo + (hi - lo) / 2;
    if mid == lo {
        return;
    }
    if hi - lo <= threshold {
        merge_sort(arr, lo, hi, helper_buf);
        return;
    }
    std::thread::scope(|scope| {
        let left_perms = scope.spawn(move || {
            _merge_sort_parallel(arr, lo, mid, helper_buf, threshold);
        });
        _merge_sort_parallel(arr, mid, hi, helper_buf, threshold);
        left_perms.join().unwrap();
    });
    merge(arr, lo, mid, hi, helper_buf, lo);
    copy(helper_buf, lo, hi, arr, lo);
}

pub struct SingleArray;
impl Sort for SingleArray {
    fn sort(input: &mut [i32], threshold: usize) {
        let arr = Array(input.as_ptr() as *mut i32);
        let helper_buf = vec![0; input.len()];
        let helper_buf_array = Array(helper_buf.as_ptr() as *mut i32);
        _merge_sort_parallel(arr, 0, input.len(), helper_buf_array, threshold);
    }
}

#[test]
fn test_seq() {
    let mut a = vec![2, 3, 5, 1, 4];
    SingleArray::sort(&mut a, 5);
    assert_eq!(a, vec![1, 2, 3, 4, 5]);
}
