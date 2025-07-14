use crate::sorts::{Element, Sort};

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

fn _merge_sort_parallel(
    arr: Array,
    lo: usize,
    hi: usize,
    helper_buf: Array,
    threshold: usize,
) -> Result<(), ()> {
    let mid = lo + (hi - lo) / 2;
    if mid == lo {
        return Ok(());
    }
    if hi - lo <= threshold {
        merge_sort(arr, lo, hi, helper_buf);
        return Ok(());
    }
    std::thread::scope(|scope| {
        let left_perms = scope.spawn(move || {
            let t = _merge_sort_parallel(arr, lo, mid, helper_buf, threshold);
            if t.is_err() { Err(()) } else { Ok(()) }
        });
        match _merge_sort_parallel(arr, mid, hi, helper_buf, threshold) {
            Ok(_) => {}
            Err(_) => {
                return Err(());
            }
        };
        match left_perms.join() {
            Ok(Ok(())) => {}
            _ => return Err(()),
        };
        Ok(())
    })?;
    merge(arr, lo, mid, hi, helper_buf, lo);
    copy(helper_buf, lo, hi, arr, lo);
    Ok(())
}

pub struct NakedVerus;
impl Sort for NakedVerus {
    type Array = Vec<Element>;

    fn prepare_array(input: Vec<Element>) -> (Self::Array, Self::Array) {
        let helper_buf = vec![0; input.len()];
        (input, helper_buf)
    }

    fn sort(input: &mut Self::Array, buf: &mut Self::Array) {
        let input_a = Array(input.as_ptr() as *mut i32);
        let buf_a = Array(buf.as_ptr() as *mut i32);
        merge_sort(input_a, 0, input.len(), buf_a);
    }

    fn sort_parallel(input: &mut Self::Array, buf: &mut Self::Array, threshold: usize) {
        let input_a = Array(input.as_ptr() as *mut i32);
        let buf_a = Array(buf.as_ptr() as *mut i32);
        _merge_sort_parallel(input_a, 0, input.len(), buf_a, threshold).unwrap();
    }

    fn name() -> &'static str {
        "naked verus"
    }
}

#[test]
fn test_seq() {
    let mut a = vec![2, 3, 5, 1, 4];
    NakedVerus::sort(&mut a, &mut vec![0; 5]);
    assert_eq!(a, vec![1, 2, 3, 4, 5]);
}
