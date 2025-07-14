use crate::sorts::{Element, slices_unchecked};

pub use slices_unchecked::merge_sort;

/// includes logic similar to vstd::spawn
/// to see whether this affects performance
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
    let left_reborrow = &mut *left;
    let (helper_buf_left, helper_buf_right) = unsafe { helper_buf.split_at_mut_unchecked(mid) };
    std::thread::scope(|s| {
        let left_handle = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            s.spawn(move || {
                let t = _merge_sort_parallel(left_reborrow, helper_buf_left, threshold);
                if t.is_err() { Err(()) } else { Ok(()) }
            })
        })) {
            Ok(res) => res,
            Err(_) => {
                println!("panic on spawn");
                std::process::abort();
            }
        };
        match _merge_sort_parallel(&mut *right, helper_buf_right, threshold) {
            Ok(_) => {}
            Err(_) => {
                return Err(());
            }
        };
        let joined = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            match left_handle.join() {
                Ok(v) => Ok(v),
                Err(_) => Err(()),
            }
        })) {
            Ok(res) => res,
            Err(_) => {
                println!("panic on join");
                std::process::abort();
            }
        };
        match joined {
            Ok(Ok(())) => {}
            _ => return Err(()),
        };
        Ok(())
    })?;
    slices_unchecked::merge(left, right, helper_buf);
    slices_unchecked::copy(helper_buf, 0, helper_buf.len(), arr, 0);
    Ok(())
}
