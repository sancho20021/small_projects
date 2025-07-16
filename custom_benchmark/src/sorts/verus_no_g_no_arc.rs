use std::{cell::UnsafeCell, sync::Arc};

use crate::sorts::verus_no_g_no_arc::permissions_array::Array;

pub struct ArrayForSorting<T> {
    pub array: Array<T>,
}

struct PCell<T>(UnsafeCell<T>);

unsafe impl<T> Send for PCell<T> {}

unsafe impl<T> Sync for PCell<T> {}

impl<T> PCell<T> {
    pub fn new(x: T) -> Self {
        PCell(UnsafeCell::new(x))
    }

    pub fn replace(&self, mut x: T) -> T {
        unsafe {
            std::mem::swap(&mut *self.0.get(), &mut x);
        }
        x
    }

    pub fn borrow(&self) -> &T {
        unsafe { &*self.0.get() }
    }
}

impl<T> ArrayForSorting<T> {
    pub fn new(data: Vec<T>) -> Self
    where
        Self: std::marker::Sized,
    {
        let array = region_array::new(data);
        Self {
            array: array,
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

pub fn merge_sort(arr: &mut ArrayForSorting<i32>, out_arr: &mut ArrayForSorting<i32>) {
    _merge_sort(&arr.array, 0, arr.array.length(), &out_arr.array, 0)
}

pub fn merge_sort_parallel(
    arr: &mut ArrayForSorting<i32>,
    out_arr: &mut ArrayForSorting<i32>,
    threshold: usize,
) -> Result<(), ()> {
    _merge_sort_parallel(
        &arr.array,
        0,
        arr.array.length(),
        &out_arr.array,
        0,
        threshold,
    )
}

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

fn _merge_sort(
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

    _merge_sort(arr, lo, mid, out_arr, out_lo);
    _merge_sort(arr, mid, hi, out_arr, out_lo);

    merge(arr, lo, mid, mid, hi, out_arr, out_lo);
    while lo < hi {
        let e = *region_array::read(out_arr, out_lo);
        region_array::replace(arr, lo, e);
        out_lo += 1;
        lo += 1;
    }
}

fn _merge_sort_parallel(
    arr: &Array<i32>,
    mut lo: usize,
    hi: usize,
    out_arr: &Array<i32>,
    mut out_lo: usize,
    threshold: usize,
) -> Result<(), ()> {
    let mid = lo + (hi - lo) / 2;
    let out_mid = out_lo + (hi - lo) / 2;
    if mid == lo {
        return Ok(());
    }

    if hi - lo <= threshold {
        _merge_sort(&*arr, lo, hi, &*out_arr, out_lo);
        return Ok(());
    }

    std::thread::scope(|scope| {
        let left_perms = scope.spawn(move || {
            let t = _merge_sort_parallel(arr, lo, mid, out_arr, out_lo, threshold);
            if t.is_err() { Err(()) } else { Ok(()) }
        });

        match _merge_sort_parallel(arr, mid, hi, out_arr, out_mid, threshold) {
            Ok(()) => {}
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

    merge(&arr, lo, mid, mid, hi, &out_arr, out_lo);
    while lo < hi {
        let e = *region_array::read(out_arr, out_lo);
        region_array::replace(arr, lo, e);
        out_lo += 1;
        lo += 1;
    }
    Ok(())
}

#[test]
fn test_par_array() {
    let arr = region_array::new(vec![5, 4, 3, 2, 1]);
    let len = arr.length();
    let out_arr = region_array::new(vec![0, 0, 0, 0, 0]);
    _merge_sort_parallel(&arr, 0, len, &out_arr, 0, 2).unwrap();
    let arr = region_array::clone_to_vec(&arr);
    assert_eq!(arr, vec![1, 2, 3, 4, 5]);
}

mod permissions_array {
    use std::sync::Arc;

    use crate::sorts::verus_no_g_no_arc::PCell;

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
            let mut ptrs = Vec::<PCell<T>>::new();
            let mut i: usize = 0;
            while data_rev.len() > 0 {
                let x = data_rev.pop().unwrap();
                let ptr = PCell::new(x);
                ptrs.push(ptr);
                i += 1;
            }
            Self { ptrs }
        }

        #[inline]
        fn vec_replace(v: &Vec<PCell<T>>, i: usize, e: T) -> T {
            // if i >= v.len() {
            //     panic!("Lol");
            // }
            let vi = unsafe { v.get_unchecked(i) };
            // panic!();
            vi.replace(e)
        }

        #[inline]
        fn vec_borrow<'a>(v: &'a Vec<PCell<T>>, i: usize) -> &'a T {
            // if i >= v.len() {
            //     panic!("Lol");
            // }
            let vi = unsafe { v.get_unchecked(i) };
            vi.borrow()
        }

        #[inline]
        pub fn replace(&self, i: usize, x: T) -> T {
            let res = Self::vec_replace(&self.ptrs, i, x);
            // let res = self.ptrs[i].replace(Tracked(&mut perm), x);
            res
        }

        #[inline]
        pub fn read<'a>(&'a self, i: usize) -> &'a T {
            Self::vec_borrow(&self.ptrs, i)
            // self.ptrs[i].borrow(Tracked(perm))
        }

        /// probably it should be implemented not as clone,
        /// but as consuming into Vec<T>,
        /// but to make it faster I have to cast?? TODO
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

    fn f() {
        let a = Array::new(vec![0, 1]);
        let a = Arc::new(a);
        let a_r1 = Arc::clone(&a);
        let a_r2 = Arc::clone(&a);

        let t1 = std::thread::spawn(move || {
            (*a_r1).replace(0, 66);
        });
        let t2 = std::thread::spawn(move || {
            (*a_r2).replace(1, 77);
        });
    }

    fn main() {
        let array = Array::new(vec![0, 1]);
        let array = Arc::new(array);
        let array_r1 = Arc::clone(&array);
        let array_r2 = Arc::clone(&array);

        let perms0 = std::thread::spawn(move || -> () {
            (*array_r1).replace(0, 66);
        });
        let perms1 = std::thread::spawn(move || -> () {
            (*array_r2).replace(1, 77);
        });

        let res0 = perms0.join();
        let res1 = perms1.join();

        let (()) = match (res0, res1) {
            (Result::Ok(x0), Result::Ok(x1)) => {}
            _ => {
                // panic!("error");
                return;
            }
        };

        print(&array);
    }

    fn print<T: std::fmt::Debug>(a: &Array<T>) {
        for i in 0..a.length() {
            let b = a.read(i);
            print!("{:?} ", b);
        }
        println!("");
    }
}

mod region_array {
    use crate::sorts::verus_no_g_no_arc::permissions_array::Array;

    pub fn new<T>(data: Vec<T>) -> Array<T> {
        let arr = Array::new(data);
        let length = arr.length();
        arr
    }

    #[inline]
    pub fn replace<T>(aself: &Array<T>, i: usize, x: T) -> T {
        <Array<T>>::replace(aself, i, x)
    }

    #[inline]
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
