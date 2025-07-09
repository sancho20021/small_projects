pub mod no_splits {
    mod pcell {
        pub type PCell<V> = V;
    }

    pub mod exec_pcell {
        use crate::merge_sorts::minimalistic_sorts::no_splits::pcell::PCell;
        pub struct Array<T> {
            pub ptrs: Vec<PCell<T>>,
        }
        impl<T> Array<T> {
            pub fn length(&self) -> usize {
                self.ptrs.len()
            }

            pub fn new(mut data: Vec<T>) -> Self {
                let mut data_rev = Vec::<T>::new();
                while data.len() > 0 {
                    data_rev.push(data.pop().unwrap());
                }
                {};
                let mut ptrs = Vec::<PCell<T>>::new();
                let mut i: usize = 0;
                while data_rev.len() > 0 {
                    let x = data_rev.pop().unwrap();
                    let ptr = {
                        let p = x;
                        p
                    };
                    ptrs.push(ptr);
                    {}
                    i += 1;
                }
                Self { ptrs }
            }
        }
    }

    use crate::merge_sorts::minimalistic_sorts::no_splits::exec_pcell::Array;

    pub struct ArrayAbstraction<T> {
        pub array: Array<T>,
    }
    impl<T> ArrayAbstraction<T> {
        pub fn new(data: Vec<T>) -> Self
        where
            Self: std::marker::Sized,
        {
            let array = region_array::new(data);
            Self { array: array }
        }
    }
    pub mod region_array {
        use crate::merge_sorts::minimalistic_sorts::no_splits::exec_pcell::Array;

        pub fn new<T>(data: Vec<T>) -> Array<T> {
            Array::new(data)
        }

        #[allow(invalid_reference_casting)]
        pub fn replace<T>(aself: &Array<T>, i: usize, x: T) -> T {
            let vi = unsafe { aself.ptrs.get_unchecked(i) };
            unsafe { std::mem::replace(&mut *(vi as *const T as *mut T), x) }
        }
        pub fn read<'a, T>(aself: &'a Array<T>, i: usize) -> &'a T {
            unsafe { aself.ptrs.get_unchecked(i) }
        }
    }
    pub mod mergesort {
        use super::*;
        fn merge(
            array: &Array<i32>,
            mut left_lo: usize,
            left_hi: usize,
            mut right_lo: usize,
            right_hi: usize,
            out_array: &Array<i32>,
            mut out_lo: usize,
        ) {
            while left_lo < left_hi && right_lo < right_hi {
                let element: i32;
                if region_array::read(array, left_lo) < region_array::read(array, right_lo) {
                    element = *region_array::read(array, left_lo);
                    left_lo += 1;
                } else {
                    element = *region_array::read(array, right_lo);
                    right_lo += 1;
                }
                region_array::replace(out_array, out_lo, element);
                out_lo += 1;
            }
            if left_lo < left_hi {
                while left_lo < left_hi {
                    let e = *region_array::read(array, left_lo);
                    region_array::replace(out_array, out_lo, e);
                    left_lo += 1;
                    out_lo += 1;
                }
            } else if right_lo < right_hi {
                while right_lo < right_hi {
                    let e = *region_array::read(array, right_lo);
                    region_array::replace(out_array, out_lo, e);
                    right_lo += 1;
                    out_lo += 1;
                }
            }
        }
        pub fn merge_sort_abstraction(arr: &mut ArrayAbstraction<i32>, out_arr: Vec<i32>) {
            let out_arr = region_array::new(out_arr);
            merge_sort(&arr.array, 0, arr.array.length(), &out_arr, 0)
        }
        fn merge_sort(
            arr: &Array<i32>,
            mut lo: usize,
            hi: usize,
            out_arr: &Array<i32>,
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
                let e = *region_array::read(out_arr, out_lo);
                region_array::replace(arr, lo, e);
                out_lo += 1;
                lo += 1;
            }
        }
        pub fn merge_sort_parallel_abstraction(
            arr: &mut ArrayAbstraction<i32>,
            out_arr: Vec<i32>,
            threshold: usize,
        ) -> Result<(), &'static str> {
            let out_arr = region_array::new(out_arr);
            merge_sort_parallel(&arr.array, 0, arr.array.length(), &out_arr, 0, threshold)
        }
        fn merge_sort_parallel(
            arr: &Array<i32>,
            mut lo: usize,
            hi: usize,
            out_arr: &Array<i32>,
            mut out_lo: usize,
            threshold: usize,
        ) -> Result<(), &'static str> {
            let mid = lo + (hi - lo) / 2;
            let out_mid = out_lo + (hi - lo) / 2;
            if mid == lo {
                return Ok(());
            }
            if hi - lo <= threshold {
                merge_sort(&*arr, lo, hi, &*out_arr, out_lo);
                return Ok(());
            }
            let arr_r1 = &*arr;
            let arr_r2 = &*arr;
            let out_arr_r1 = &*out_arr;
            let out_arr_r2 = &*out_arr;
            std::thread::scope(|scope| {
                let left_perms = scope.spawn(move || -> Result<(), ()> {
                    let t = merge_sort_parallel(arr_r1, lo, mid, out_arr_r1, out_lo, threshold);
                    if t.is_err() {
                        return Err(());
                    } else {
                        Ok(())
                    }
                });
                match merge_sort_parallel(arr_r2, mid, hi, out_arr_r2, out_mid, threshold) {
                    Ok(()) => {}
                    Err(_) => {
                        return Result::Err("error while joining threads");
                    }
                };
                match left_perms.join() {
                    Result::Ok(Ok(l)) => Ok(()),
                    _ => {
                        return Result::Err("error while joining threads");
                    }
                }
            })?;
            merge(&arr, lo, mid, mid, hi, &out_arr, out_lo);
            while lo < hi {
                let e = *region_array::read(&*out_arr, out_lo);
                region_array::replace(&*arr, lo, e);
                out_lo += 1;
                lo += 1;
            }
            Ok(())
        }
    }
}
