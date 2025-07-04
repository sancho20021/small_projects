pub fn merge_sort<T: Ord + Copy>(arr: &mut [T], out_arr: &mut [T]) {
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

pub fn merge<T: Ord + Copy>(left: &[T], right: &[T], out: &mut [T]) {
    let mut left_index = 0;
    let mut right_index = 0;
    let mut out_index = 0;
    while left_index < left.len() && right_index < right.len() {
        let element: T;
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

pub fn merge_sort_unchecked<T: Ord + Copy>(arr: &mut [T], out_arr: &mut [T]) {
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }

    merge_sort(&mut arr[0..mid], &mut out_arr[0..mid]);
    merge_sort(&mut arr[mid..], &mut out_arr[mid..]);

    merge(&arr[0..mid], &arr[mid..], out_arr);
    arr.copy_from_slice(&out_arr);
}

fn merge_unchecked<T: Ord + Copy>(left: &[T], right: &[T], out: &mut [T]) {
    let mut left_index = 0;
    let mut right_index = 0;
    let mut out_index = 0;
    while left_index < left.len() && right_index < right.len() {
        let element: T;
        unsafe {
            if left.get_unchecked(left_index) <= right.get_unchecked(right_index) {
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

pub fn merge_sort_parallel<T: Ord + Send + Copy>(
    arr: &mut [T],
    out_arr: &mut [T],
    threshold: usize,
) {
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

pub fn merge_sort_parallel_unchecked<T: Ord + Send + Copy>(
    arr: &mut [T],
    out_arr: &mut [T],
    threshold: usize,
) {
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }

    if arr.len() <= threshold {
        merge_sort_unchecked(arr, out_arr);
        return;
    }

    let ((left, right), (out_left, out_right)) = unsafe {
        (
            arr.split_at_mut_unchecked(mid),
            out_arr.split_at_mut_unchecked(mid),
        )
    };

    std::thread::scope(|s| {
        let left_handle = s.spawn(|| {
            merge_sort_parallel_unchecked(&mut *left, out_left, threshold);
        });
        merge_sort_parallel_unchecked(&mut *right, out_right, threshold);
        left_handle.join().unwrap();
    });

    merge_unchecked(left, right, out_arr);
    let mut i = 0;
    while i < arr.len() {
        unsafe {
            *arr.get_unchecked_mut(i) = *out_arr.get_unchecked(i);
        }
        i += 1;
    }
}

pub fn merge_sort_threadpool<T: Ord + Send + Copy>(
    arr: &mut [T],
    out_arr: &mut [T],
    threshold: usize,
) {
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

    rayon::join(
        || merge_sort_threadpool(&mut *left, out_left, threshold),
        || merge_sort_threadpool(&mut *right, out_right, threshold),
    );

    merge(left, right, out_arr);
    arr.copy_from_slice(out_arr);
}

mod runtime_checked_analogue {
    // use std::sync::Arc;

    // use ghost_cell::{GhostCell, GhostToken};

    // fn merge(left: &[i32], right: &[i32], out: &mut [i32]) {
    //     let mut left_lo = 0;
    //     let mut right_lo = 0;
    //     let mut out_lo = 0;
    //     while left_lo < left.len() && right_lo < right.len() {
    //         let element: i32;
    //         if left[left_lo] < right[right_lo] {
    //             element = left[left_lo];
    //             left_lo += 1;
    //         } else {
    //             element = right[right_lo];
    //             right_lo += 1;
    //         }
    //         out[out_lo] = element;
    //         out_lo += 1;
    //     }

    //     if left_lo < left.len() {
    //         while left_lo < left.len() {
    //             let e = left[left_lo];
    //             out[out_lo] = e;
    //             left_lo += 1;
    //             out_lo += 1;
    //         }
    //     } else if right_lo < right.len() {
    //         while right_lo < right.len() {
    //             let e = right[right_lo];
    //             out[out_lo] = e;
    //             right_lo += 1;
    //             out_lo += 1;
    //         }
    //     }
    // }

    // pub fn merge_sort(arr: &mut [i32], out_arr: &mut [i32]) {
    //     let mid = arr.len() / 2;
    //     if mid == 0 {
    //         return;
    //     }

    //     merge_sort(&mut arr[..mid], &mut out_arr[..mid]);
    //     merge_sort(&mut arr[mid..], &mut out_arr[mid..]);
    //     merge(&arr[..mid], &arr[mid..], out_arr);

    //     let mut lo = 0;
    //     while lo < arr.len() {
    //         let e = out_arr[lo];
    //         arr[lo] = e;
    //         lo += 1;
    //     }
    // }

    // type Slice<'id, 'a> = GhostCell<'id, &'a mut [i32]>;

    // pub fn merge_sort_parallel<'id1, 'id2>(
    //     arr: Arc<Slice<'id1, '_>>,
    //     perm: &mut GhostToken<'id1>,
    //     out_arr: Arc<Slice<'id2, '_>>,
    //     out_perm: &mut GhostToken<'id2>,
    //     threshold: usize,
    // ) -> Result<(), &'static str> {
    //     let mid = arr.borrow(perm).len() / 2;
    //     if mid == 0 {
    //         return Ok(());
    //     }

    //     if arr.borrow(perm).len() < threshold {
    //         merge_sort(arr.borrow_mut(perm), out_arr.borrow_mut(out_perm));
    //         return Ok(());
    //     }

    //     let arr_r1 = Arc::clone(&arr);
    //     let arr_r2 = Arc::clone(&arr);

    //     let out_arr_r1 = Arc::clone(&out_arr);
    //     let out_arr_r2 = Arc::clone(&out_arr);

    //     let left_perms = vstd::thread::spawn(
    //         move || -> Result<(Tracked<Region<i32>>, Tracked<Region<i32>>), ()> {
    //             let t = merge_sort_parallel(
    //                 arr_r1,
    //                 lo,
    //                 mid,
    //                 Tracked(&mut left_perms),
    //                 out_arr_r1,
    //                 out_lo,
    //                 Tracked(&mut out_left_perms),
    //                 threshold,
    //             );
    //             if t.is_err() {
    //                 return Err(());
    //             } else {
    //                 Ok((Tracked(left_perms), Tracked(out_left_perms)))
    //             }
    //         },
    //     );

    //     let right_perms = vstd::thread::spawn(
    //         move || -> Result<(Tracked<Region<i32>>, Tracked<Region<i32>>), ()> {
    //             match merge_sort_parallel(
    //                 arr_r2,
    //                 mid,
    //                 hi,
    //                 Tracked(&mut right_perms),
    //                 out_arr_r2,
    //                 out_mid,
    //                 Tracked(&mut out_right_perms),
    //                 threshold,
    //             ) {
    //                 Ok(()) => Ok((Tracked(right_perms), Tracked(out_right_perms))),
    //                 Err(_) => Err(()),
    //             }
    //         },
    //     );

    //     let left_perms = left_perms.join();
    //     let right_perms = right_perms.join();

    //     let (
    //         (Tracked(mut left_perms), Tracked(mut out_left_perms)),
    //         (Tracked(mut right_perms), Tracked(mut out_right_perms)),
    //     ) = match (left_perms, right_perms) {
    //         (Result::Ok(Ok(l)), Result::Ok(Ok(r))) => (l, r),
    //         _ => {
    //             return Result::Err("error while joining threads");
    //         }
    //     };

    //     merge(
    //         &arr,
    //         Tracked(perms),
    //         lo,
    //         mid,
    //         mid,
    //         hi,
    //         &out_arr,
    //         Tracked(out_perms),
    //         out_lo,
    //     );
    //     while lo < hi {
    //         let e = *region_array::read(&*out_arr, out_lo, Tracked(out_perms));
    //         region_array::replace(&*arr, lo, e, Tracked(perms));
    //         out_lo += 1;
    //         lo += 1;
    //     }
    //     Ok(())
    // }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use disjoint_mut_test::disjoint_verified::{
        self, exec_pcell::Array, split_at::ArrayAbstraction,
    };

    use super::{merge_sort, merge_sort_parallel, merge_sort_threadpool};

    #[test]
    fn test_seq_sort() {
        let mut array = vec![5, 4, 3, 2, 1];
        merge_sort(array.as_mut_slice(), &mut vec![0; 5]);
        assert_eq!(array, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_par_sort() {
        let mut array = vec![5, 4, 3, 2, 1];
        merge_sort_parallel(array.as_mut_slice(), &mut vec![0; 5], 2);
        assert_eq!(array, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_tp_sort() {
        let mut array = vec![5, 4, 3, 2, 1];
        merge_sort_threadpool(array.as_mut_slice(), &mut vec![0; 5], 2);
        assert_eq!(array, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_seq_array() {
        let mut arr = ArrayAbstraction::new(vec![5, 4, 3, 2, 1]);
        disjoint_verified::split_at::mergesort::merge_sort_abstraction(&mut arr, vec![0; 5]);
        let arr = arr.clone_to_vec();
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_par_array() {
        let mut arr = ArrayAbstraction::new(vec![5, 4, 3, 2, 1]);
        disjoint_verified::split_at::mergesort::merge_sort_parallel_abstraction(
            &mut arr,
            vec![0; 5],
            2,
        )
        .unwrap();
        let arr = arr.clone_to_vec();
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
}
