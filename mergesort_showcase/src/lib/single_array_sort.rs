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
    while left_lo < left_hi {
        helper_buf.set(helper_buf_lo, array.read(left_lo));
        left_lo += 1;
        helper_buf_lo += 1;
    }
    while right_lo < right_hi {
        helper_buf.set(helper_buf_lo, array.read(right_lo));
        right_lo += 1;
        helper_buf_lo += 1;
    }
}

fn merge_sort(arr: Array, mut lo: usize, hi: usize, helper_buf: Array, mut helper_buf_lo: usize) {
    let mid = lo + (hi - lo) / 2;
    if mid == lo {
        return;
    }
    merge_sort(arr, lo, mid, helper_buf, helper_buf_lo);
    merge_sort(arr, mid, hi, helper_buf, helper_buf_lo);
    merge(arr, lo, mid, hi, helper_buf, helper_buf_lo);
    while lo < hi {
        arr.set(lo, helper_buf.read(helper_buf_lo));
        helper_buf_lo += 1;
        lo += 1;
    }
}

fn _merge_sort_parallel(
    arr: Array,
    mut lo: usize,
    hi: usize,
    helper_buf: Array,
    mut helper_buf_lo: usize,
    threshold: usize,
) {
    let mid = lo + (hi - lo) / 2;
    let helper_buf_mid = helper_buf_lo + (hi - lo) / 2;
    if mid == lo {
        return;
    }
    if hi - lo <= threshold {
        merge_sort(arr, lo, hi, helper_buf, helper_buf_lo);
        return;
    }
    std::thread::scope(|scope| {
        let left_perms = scope.spawn(move || {
            _merge_sort_parallel(arr, lo, mid, helper_buf, helper_buf_lo, threshold);
        });
        _merge_sort_parallel(arr, mid, hi, helper_buf, helper_buf_mid, threshold);
        left_perms.join().unwrap();
    });
    merge(arr, lo, mid, hi, helper_buf, helper_buf_lo);
    while lo < hi {
        arr.set(lo, helper_buf.read(helper_buf_lo));
        helper_buf_lo += 1;
        lo += 1;
    }
}

pub fn merge_sort_parallel(input: &mut [i32], threshold: usize) {
    let arr = Array(input.as_ptr() as *mut i32);
    let helper_buf = vec![0; input.len()];
    let helper_buf_array = Array(helper_buf.as_ptr() as *mut i32);
    _merge_sort_parallel(arr, 0, input.len(), helper_buf_array, 0, threshold);
}


#[test]
fn test_seq() {
    let mut a = vec![2, 3, 5, 1, 4];
    merge_sort_parallel(&mut a, 5);
    assert_eq!(a, vec![1, 2, 3, 4, 5]);
}
