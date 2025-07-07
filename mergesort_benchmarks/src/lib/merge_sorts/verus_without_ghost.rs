mod pcell {
    use core::cell::UnsafeCell;
    use core::{mem, mem::MaybeUninit};

    pub struct PCell<V> {
        ucell: UnsafeCell<MaybeUninit<V>>,
    }

    /// `PCell` is _always_ safe to `Send` or `Sync`. Rather, it is the [`PointsTo`] object where `Send` and `Sync` matter.
    /// (It doesn't matter if you move the bytes to another thread if you can't access them.)
    unsafe impl<T> Sync for PCell<T> {}

    unsafe impl<T> Send for PCell<T> {}

    impl<V> PCell<V> {
        #[inline(always)]
        pub const fn new(v: V) -> PCell<V> {
            let p = PCell {
                ucell: UnsafeCell::new(MaybeUninit::new(v)),
            };
            p
        }

        #[inline(always)]
        pub fn replace(&self, in_v: V) -> V {
            // LOOK (not just ucell.replace)
            unsafe {
                let mut m = MaybeUninit::new(in_v);
                mem::swap(&mut m, &mut *self.ucell.get());
                m.assume_init()
            }
        }

        #[inline(always)]
        pub fn borrow<'a>(&'a self) -> &'a V {
            unsafe { (*self.ucell.get()).assume_init_ref() }
        }
    }
}

pub mod exec_pcell {
    use crate::merge_sorts::verus_without_ghost::pcell::PCell;
    pub struct Array<T> {
        ptrs: Vec<PCell<T>>,
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
                let ptr = PCell::new(x);
                ptrs.push(ptr);
                {}
                i += 1;
            }
            Self { ptrs }
        }

        fn vec_replace(v: &Vec<PCell<T>>, i: usize, e: T) -> T {
            let vi = unsafe { v.get_unchecked(i) };
            vi.replace(e)
        }

        fn vec_borrow<'a>(v: &'a Vec<PCell<T>>, i: usize) -> &'a T {
            let vi = unsafe { v.get_unchecked(i) };
            vi.borrow()
        }
        pub fn replace(&self, i: usize, x: T) -> T {
            let res = Self::vec_replace(&self.ptrs, i, x);
            {}
            res
        }
        pub fn read<'a>(&'a self, i: usize) -> &'a T {
            Self::vec_borrow(&self.ptrs, i)
        }
        #[doc = " probably it should be implemented not as clone,"]
        #[doc = " but as consuming into Vec<T>,"]
        #[doc = " but to make it faster I have to cast?? TODO"]
        pub fn clone_to_vec<'a>(&'a self) -> Vec<T>
        where
            T: Clone,
        {
            let mut res: Vec<T> = Vec::with_capacity(self.length());
            let mut i: usize = 0;
            while i < self.length() {
                res.push(self.read(i).clone());
                i += 1;
            }
            res
        }
    }

    fn print<T: std::fmt::Debug>(a: &Array<T>) {
        for i in 0..a.length() {
            let b = a.read(i);
            {
                println!("{0:?} ", b);
            };
        }
        {
            println!("\n");
        };
    }
}
use std::marker::Sized;
use std::sync::Arc;

use crate::merge_sorts::verus_without_ghost::exec_pcell::Array;

pub struct ArrayAbstraction<T> {
    pub array: Arc<Array<T>>,
}
impl<T> ArrayAbstraction<T> {
    pub fn new(data: Vec<T>) -> Self
    where
        Self: std::marker::Sized,
    {
        let array = region_array::new(data);
        Self {
            array: Arc::new(array),
        }
    }
    pub fn clone_to_vec(&self) -> Vec<T>
    where
        Self: Sized,
        T: Clone,
    {
        region_array::clone_to_vec(&self.array)
    }
}
pub mod region_array {
    use crate::merge_sorts::verus_without_ghost::exec_pcell::Array;

    pub fn new<T>(data: Vec<T>) -> Array<T> {
        let arr = Array::new(data);
        {};
        let length = arr.length();
        arr
    }
    pub fn replace<T>(aself: &Array<T>, i: usize, x: T) -> T {
        <Array<T>>::replace(aself, i, x)
    }
    pub fn read<'a, T>(aself: &'a Array<T>, i: usize) -> &'a T {
        <Array<T>>::read(aself, i)
    }
    pub fn clone_to_vec<T>(aself: &Array<T>) -> Vec<T>
    where
        T: Clone,
        T: Clone,
    {
        <Array<T>>::clone_to_vec(aself)
    }
}
pub mod mergesort {
    use super::*;
    use std::sync::Arc;
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
        merge_sort(&arr.array, 0, (&*arr.array).length(), &out_arr, 0)
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
        merge_sort_parallel(
            Arc::clone(&arr.array),
            0,
            (&*arr.array).length(),
            Arc::new(out_arr),
            0,
            threshold,
        )
    }
    fn merge_sort_parallel(
        arr: Arc<Array<i32>>,
        mut lo: usize,
        hi: usize,
        out_arr: Arc<Array<i32>>,
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
        let arr_r1 = Arc::clone(&arr);
        let arr_r2 = Arc::clone(&arr);
        let out_arr_r1 = Arc::clone(&out_arr);
        let out_arr_r2 = Arc::clone(&out_arr);
        let left_perms = std::thread::spawn(move || -> Result<(), ()> {
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
        let left_perms = left_perms.join();
        let () = match left_perms {
            Result::Ok(Ok(l)) => l,
            _ => {
                return Result::Err("error while joining threads");
            }
        };
        {}
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

mod tests {
    use crate::{
        get_threshold,
        merge_sorts::verus_without_ghost::{
            exec_pcell::Array,
            {mergesort::merge_sort_parallel_abstraction, ArrayAbstraction},
        },
    };

    #[test]
    fn test() {
        let arr = vec![1, 2, 3, 3, 2, 1];
        let threshold = get_threshold(arr.len());
        let mut arr = ArrayAbstraction::new(arr);
        let out_array = vec![0; arr.array.length()];
        merge_sort_parallel_abstraction(&mut arr, out_array, threshold).unwrap();
        let vec = arr.clone_to_vec();
        assert_eq!(vec, vec![1, 1, 2, 2, 3, 3]);
    }
}
