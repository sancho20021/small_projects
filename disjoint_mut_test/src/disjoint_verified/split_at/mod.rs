use std::sync::Arc;

use vstd::prelude::*;

use crate::disjoint_verified::exec_pcell::SpecPerms;

verus! {

use std::marker::Sized;
use super::exec_pcell::{Array, Perms};

pub struct Region<T> {
    tracked lo: usize,
    tracked hi: usize,
    perms: SpecPerms<T>,
}

impl<T> Region<T> {
    pub closed spec fn lo(&self) -> usize {
        self.lo
    }

    pub closed spec fn hi(&self) -> usize {
        self.hi
    }
}

pub struct ArrayAbstraction<T> {
    pub array: Arc<Array<T>>,
    pub perms: Tracked<Region<T>>,
}

impl<T> ArrayAbstraction<T> {
    pub fn new(data: Vec<T>) -> (res: Self)
        where Self: std::marker::Sized,
        ensures
            data.len() == res.array.len(),
            <Array<T> as RegionArray>::wf(*res.array, res.perms@),
    {
        let (array, perms) = <Array<T> as RegionArray>::new(data);
        Self {
            array: Arc::new(array),
            perms
        }
    }

    pub fn clone_to_vec(&self) -> Vec<T>
    where
        Self: Sized,
        T: Clone,
    requires
        <Array<T> as RegionArray>::wf(*self.array, self.perms@),
        self.perms@.lo() == 0,
        self.perms@.hi() == self.array.len()
    {
        <Array<T> as RegionArray>::clone_to_vec(&self.array, Tracked(self.perms.borrow()))
    }
}

pub trait RegionArray {
    type T;
    spec fn wf(self, region: Region<Self::T>) -> bool
        where Self: std::marker::Sized;

    spec fn len(&self) -> usize where Self: Sized;

    fn split_at(&self, m: usize, Tracked(region): Tracked<Region<Self::T>>) -> (res: (Tracked<Region<Self::T>>, Tracked<Region<Self::T>>))
        where Self: std::marker::Sized,
        requires
            region.lo() <= m@ < region.hi(),
            self.wf(region),
        ensures
            self.wf(res.0@),
            res.0@.lo() == region.lo(),
            res.0@.hi() == m,
            self.wf(res.1@),
            res.1@.lo() == m,
            res.1@.hi() == region.hi();

    fn split_off(&self, m: usize, Tracked(region): Tracked<&mut Region<Self::T>>) -> (res: Tracked<Region<Self::T>>)
        where Self: Sized,
        requires
            old(region).lo() <= m@ < old(region).hi(),
            self.wf(*old(region)),
        ensures
            self.wf(*region),
            region.lo() == old(region).lo(),
            region.hi() == m,
            self.wf(res@),
            res@.lo() == m,
            res@.hi() == old(region).hi();

    fn merge(&self, Tracked(left): Tracked<&mut Region<Self::T>>, Tracked(right): Tracked<Region<Self::T>>)
        where Self: Sized,
        requires
            self.wf(*old(left)),
            self.wf(right),
            old(left).hi() == right.lo(),
        ensures
            self.wf(*left),
            left.lo() == old(left).lo(),
            left.hi() == right.hi();

    fn new(data: Vec<Self::T>) -> (res: (Self, Tracked<Region<Self::T>>))
        where Self: std::marker::Sized,
        ensures
            data.len() == res.0.len(),
            res.0.wf(res.1@),
            res.1@.lo() == 0,
            res.1@.hi() == res.0.len();

    fn replace(&self, i: usize, x: Self::T, Tracked(perms): Tracked<&mut Region<Self::T>>) -> (res: Self::T)
        where Self: Sized,
        requires
            self.wf((*old(perms))),
            old(perms).lo() <= i < old(perms).hi(),
        ensures
            self.wf(*perms),
            perms.lo() == old(perms).lo(),
            perms.hi() == old(perms).hi();

    fn read<'a>(&'a self, i: usize, Tracked(perms): Tracked<&'a Region<Self::T>>) -> (res: &'a Self::T)
        where Self: Sized,
        requires
            self.wf(*perms),
            perms.lo() <= i < perms.hi();

    fn clone_to_vec(&self, Tracked(perms): Tracked<&Region<Self::T>>) -> Vec<Self::T>
        where
            Self: Sized,
            Self::T: Clone,
        requires
            self.wf(*perms),
            perms.lo() == 0,
            perms.hi() == self.len();
}

impl<T> RegionArray for Array<T> {
    type T = T;

    closed spec fn wf(self, region: Region<T>) -> bool {
        region.lo <= region.hi <= self.len() && self.wf(region.perms) && forall |i: usize| region.lo <= i < region.hi ==> self.available(i, region.perms)
    }

    fn split_at(&self, m: usize, mut region: Tracked<Region<T>>) -> (res: (Tracked<Region<T>>, Tracked<Region<T>>)) {
        let Tracked(mut region) = region;
        let right = self.split_off(m, Tracked(&mut region));
        (Tracked(region), right)
    }

    fn split_off(&self, m: usize, Tracked(region): Tracked<&mut Region<T>>) -> (res: Tracked<Region<T>>) {
        let ghost old_perms = region.perms;
        let ghost right_keys = Set::<usize>::new(|i: usize| m <= i < region.hi());
        assert(forall |i: usize| region.lo() <= i < region.hi() ==> self.available(i, region.perms) ==> region.perms.contains_key(i));

        let tracked right_perms = region.perms.tracked_remove_keys(right_keys);
        let tracked right = Region {
            lo: m,
            hi: region.hi,
            perms: right_perms,
        };
        proof {
            region.hi = m;
        }
        assert(self.wf(region.perms)) by {
            self.submap_wf(old_perms, region.perms);
        }
        assert(self.wf(right_perms)) by {
            self.submap_wf(old_perms, right_perms);
        }
        Tracked(right)
    }

    fn merge(&self, Tracked(left): Tracked<&mut Region<Self::T>>, Tracked(right): Tracked<Region<Self::T>>) {
        assert(forall |i: usize| left.lo <= i < left.hi ==> self.available(i, left.perms) ==> left.perms.contains_key(i));
        assert(forall |i: usize| right.lo <= i < right.hi ==> self.available(i, right.perms) ==> right.perms.contains_key(i));
        proof {
            let tracked right_perms = right.perms;
            left.perms.tracked_union_prefer_right(right_perms);
        }
        proof {
            left.hi = right.hi;
        }
        assert(self.wf(left.perms)) by {
            self.union_wf(old(left).perms, right.perms);
        }
    }

    open spec fn len(&self) -> usize {
        Array::len(self)
    }

    fn new(data: Vec<T>) -> (res: (Self, Tracked<Region<T>>)) {
        let (arr, Tracked(perms)) = Array::new(data);
        assert(arr.wf(perms));
        let length = arr.length();
        let tracked region = Region {
            lo: 0usize,
            hi: length,
            perms: perms,
        };
        (arr, Tracked(region))
    }

    fn replace(&self, i: usize, x: T, Tracked(perms): Tracked<&mut Region<Self::T>>) -> (res: T) {
        <Array<T>>::replace(self, i, x, Tracked(&mut perms.perms))
    }

    fn read<'a>(&'a self, i: usize, Tracked(perms): Tracked<&'a Region<Self::T>>) -> (res: &'a Self::T) {
        <Array<T>>::read(self, i, Tracked(&perms.perms))
    }

    fn clone_to_vec(&self, Tracked(perms): Tracked<&Region<Self::T>>) -> Vec<Self::T>
        where T: Clone
    {
        <Array<T>>::clone_to_vec(self, Tracked(&perms.perms))
    }
}

pub mod mergesort {
    use super::*;
    use std::sync::Arc;

    fn merge(
        array: &Array<i32>,
        Tracked(perms): Tracked<&Region<i32>>,
        mut left_lo: usize, left_hi: usize,
        mut right_lo: usize, right_hi: usize,
        out_array: &Array<i32>,
        Tracked(out_perms): Tracked<&mut Region<i32>>,
        mut out_lo: usize,
    )
        requires
            perms.lo() <= left_lo <= left_hi <= perms.hi() <= array.len(),
            perms.lo() <= right_lo <= right_hi <= perms.hi() <= array.len(),
            left_hi - left_lo + right_hi - right_lo <= usize::MAX,
            <Array<i32> as RegionArray>::wf(*array, (*perms)),
            <Array<i32> as RegionArray>::wf(*out_array, *old(out_perms)),
            old(out_perms).lo() <= out_lo <= out_lo + (left_hi - left_lo + right_hi - right_lo) <= old(out_perms).hi() <= out_array.len(),
            out_lo + right_hi - right_lo + left_hi - left_lo <= old(out_perms).hi(),
        ensures
            // res.len() == left_hi - left_lo + right_hi - right_lo,
            <Array<i32> as RegionArray>::wf(*out_array, *out_perms),
            old(out_perms).lo() == out_perms.lo(),
            old(out_perms).hi() == out_perms.hi(),
    {
        let ghost old_out_lo = out_lo;
        let ghost old_left_lo = left_lo;
        let ghost old_right_lo = right_lo;
        while left_lo < left_hi && right_lo < right_hi
            invariant
                <Array<i32> as RegionArray>::wf(*array, (*perms)),
                perms.lo() <= left_lo <= left_hi <= perms.hi() <= array.len(),
                perms.lo() <= right_lo <= right_hi <= perms.hi() <= array.len(),
                <Array<i32> as RegionArray>::wf(*out_array, *out_perms),
                out_perms.lo() <= out_lo <= old_out_lo + (left_hi - old_left_lo + right_hi - old_right_lo) <= out_perms.hi() <= out_array.len(),
                out_lo == old_out_lo + (left_lo - old_left_lo) + (right_lo - old_right_lo),
                old_out_lo + right_hi - old_right_lo + left_hi - old_left_lo <= old(out_perms).hi(),
                old(out_perms).lo() == out_perms.lo(),
                old(out_perms).hi() == out_perms.hi(),
        {
            let element: i32;
            if <Array<i32> as RegionArray>::read(array, left_lo, Tracked(perms)) < <Array<i32> as RegionArray>::read(array, right_lo, Tracked(perms)) {
                element = *<Array<i32> as RegionArray>::read(array, left_lo, Tracked(perms));
                left_lo += 1;
            } else {
                element = *<Array<i32> as RegionArray>::read(array, right_lo, Tracked(perms));
                right_lo += 1;
            }
            <Array<i32> as RegionArray>::replace(out_array, out_lo, element, Tracked(out_perms));
            out_lo += 1;
        }

        if left_lo < left_hi {
            while left_lo < left_hi
                invariant
                    <Array<i32> as RegionArray>::wf(*array, (*perms)),
                    left_hi - left_lo + right_hi - right_lo + out_lo - old_out_lo == left_hi - old_left_lo + right_hi - old_right_lo,
                    perms.lo() <= left_lo <= left_hi <= perms.hi() <= array.len(),
                    perms.lo() <= right_lo <= right_hi,
                    <Array<i32> as RegionArray>::wf(*out_array, *out_perms),
                    out_perms.lo() <= out_lo <= out_perms.hi() <= out_array.len(),
                    out_lo == old_out_lo + (left_lo - old_left_lo) + (right_lo - old_right_lo),
                    out_lo <= old_out_lo + left_lo - old_left_lo + right_hi - old_right_lo,
                    old_out_lo + right_hi - old_right_lo + left_hi - old_left_lo <= old(out_perms).hi(),
                    old(out_perms).lo() == out_perms.lo(),
                    old(out_perms).hi() == out_perms.hi(),
            {
                let e = *<Array<i32> as RegionArray>::read(array, left_lo, Tracked(perms));
                <Array<i32> as RegionArray>::replace(out_array, out_lo, e, Tracked(out_perms));
                left_lo += 1;
                out_lo += 1;
            }
        } else if right_lo < right_hi {
            while right_lo < right_hi
                invariant
                    <Array<i32> as RegionArray>::wf(*array, (*perms)),
                    left_hi - left_lo + right_hi - right_lo + out_lo - old_out_lo == left_hi - old_left_lo + right_hi - old_right_lo,
                    perms.lo() <= right_lo <= right_hi <= perms.hi() <= array.len(),
                    perms.lo() <= left_lo <= left_hi,
                    <Array<i32> as RegionArray>::wf(*out_array, *out_perms),
                    out_perms.lo() <= out_lo <= out_perms.hi() <= out_array.len(),
                    out_lo == old_out_lo + (left_lo - old_left_lo) + (right_lo - old_right_lo),
                    out_lo <= old_out_lo + left_lo - old_left_lo + right_hi - old_right_lo,
                    old_out_lo + right_hi - old_right_lo + left_hi - old_left_lo <= old(out_perms).hi(),
                    old(out_perms).lo() == out_perms.lo(),
                    old(out_perms).hi() == out_perms.hi(),
            {
                let e = *<Array<i32> as RegionArray>::read(array, right_lo, Tracked(perms));
                <Array<i32> as RegionArray>::replace(out_array, out_lo, e, Tracked(out_perms));
                right_lo += 1;
                out_lo += 1;
            }
        }
    }

    pub fn merge_sort_abstraction(
        arr: &mut ArrayAbstraction<i32>,
        out_arr: Vec<i32>,
    )
        requires
            old(arr).perms@.lo() == 0,
            old(arr).perms@.hi() == old(arr).array.len(),
            <Array<i32> as RegionArray>::wf(*old(arr).array, (old(arr).perms@)),
            old(arr).array.len() == out_arr.len(),
        ensures
            <Array<i32> as RegionArray>::wf(*arr.array, (arr.perms@)),
            arr.perms@.lo() == old(arr).perms@.lo(),
            arr.perms@.hi() == old(arr).perms@.hi(),
    {
        let (out_arr, mut out_perms) = <Array<i32> as RegionArray>::new(out_arr);
        merge_sort(
            &arr.array,
            0,
            (&*arr.array).length(),
            Tracked(arr.perms.borrow_mut()),
            &out_arr,
            0,
            Tracked(out_perms.borrow_mut()),
        )
    }

    pub fn merge_sort(
        arr: &Array<i32>,
        mut lo: usize, hi: usize,
        Tracked(perms): Tracked<&mut Region<i32>>,
        out_arr: &Array<i32>,
        mut out_lo: usize,
        Tracked(out_perms): Tracked<&mut Region<i32>>,
    )
        requires
            old(perms).lo() <= lo <= hi <= old(perms).hi() <= arr.len(),
            <Array<i32> as RegionArray>::wf(*arr, (*old(perms))),
            old(out_perms).lo() <= out_lo <= out_lo + hi - lo <= old(out_perms).hi() <= out_arr.len(),
            <Array<i32> as RegionArray>::wf(*out_arr, (*old(out_perms))),
        ensures
            <Array<i32> as RegionArray>::wf(*arr, (*perms)),
            perms.lo() == old(perms).lo(),
            perms.hi() == old(perms).hi(),
            <Array<i32> as RegionArray>::wf(*out_arr, *out_perms),
            out_perms.lo() == old(out_perms).lo(),
            out_perms.hi() == old(out_perms).hi(),
    {
        let ghost old_out_lo = out_lo;
        let ghost old_lo = lo;
        let mid = lo + (hi - lo) / 2;
        if mid == lo {
            return;
        }

        let ghost old_perms = *perms;
        merge_sort(arr, lo, mid, Tracked(perms), out_arr, out_lo, Tracked(out_perms));
        let ghost perms1 = *perms;
        merge_sort(arr, mid, hi, Tracked(perms), out_arr, out_lo, Tracked(out_perms));

        merge(arr, Tracked(perms), lo, mid, mid, hi, out_arr, Tracked(out_perms), out_lo);
        while lo < hi
            invariant
                perms.lo() == old(perms).lo(),
                perms.hi() == old(perms).hi(),
                perms.lo() <= lo <= hi <= perms.hi() <= arr.len(),
                <Array<i32> as RegionArray>::wf(*arr, (*perms)),
                <Array<i32> as RegionArray>::wf(*out_arr, *out_perms),
                out_perms.lo() == old(out_perms).lo(),
                out_perms.hi() == old(out_perms).hi(),
                out_perms.lo() <= out_lo <= out_perms.hi() <= out_arr.len(),
                out_lo <= old_out_lo + hi - old_lo <= old(out_perms).hi(),
                out_lo - old_out_lo == lo - old_lo,
        {
            let ghost perms_prev = *perms;
            let e = *<Array<i32> as RegionArray>::read(out_arr, out_lo, Tracked(out_perms));
            <Array<i32> as RegionArray>::replace(arr, lo, e, Tracked(perms));
            out_lo += 1;
            lo += 1;
        }
    }

    pub fn merge_sort_parallel_abstraction(
        arr: &mut ArrayAbstraction<i32>,
        out_arr: Vec<i32>,
        threshold: usize,
    ) -> (ret: Result<(), &'static str>)
        requires
            old(arr).perms@.lo() == 0,
            old(arr).perms@.hi() == old(arr).array.len(),
            <Array<i32> as RegionArray>::wf(*old(arr).array, (old(arr).perms@)),
            old(arr).array.len() == out_arr.len(),
        ensures
            ret.is_ok() ==> <Array<i32> as RegionArray>::wf(*arr.array, (arr.perms@)),
            ret.is_ok() ==> arr.perms@.lo() == old(arr).perms@.lo(),
            ret.is_ok() ==> arr.perms@.hi() == old(arr).perms@.hi(),
    {
        let (out_arr, mut out_perms) = <Array<i32> as RegionArray>::new(out_arr);
        merge_sort_parallel(
            Arc::clone(&arr.array),
            0,
            (&*arr.array).length(),
            Tracked(arr.perms.borrow_mut()),
            Arc::new(out_arr),
            0,
            Tracked(out_perms.borrow_mut()),
            threshold,
        )
    }

    pub fn merge_sort_parallel(
        arr: Arc<Array<i32>>,
        mut lo: usize, hi: usize,
        Tracked(perms): Tracked<&mut Region<i32>>,
        out_arr: Arc<Array<i32>>,
        mut out_lo: usize,
        Tracked(out_perms): Tracked<&mut Region<i32>>,
        threshold: usize
    ) -> (ret: Result<(), &'static str>)
        requires
            old(perms).lo() <= lo <= hi <= old(perms).hi() <= arr.len(),
            <Array<i32> as RegionArray>::wf(*arr, (*old(perms))),
            old(out_perms).lo() <= out_lo <= out_lo + hi - lo <= old(out_perms).hi() <= out_arr.len(),
            <Array<i32> as RegionArray>::wf(*out_arr, *old(out_perms)),
        ensures
            ret.is_ok() ==> <Array<i32> as RegionArray>::wf(*arr, (*perms)),
            ret.is_ok() ==> old(perms).lo() == perms.lo() && old(perms).hi() == perms.hi(),
            ret.is_ok() ==> <Array<i32> as RegionArray>::wf(*out_arr, *out_perms),
            ret.is_ok() ==> old(out_perms).lo() == out_perms.lo() && old(out_perms).hi() == out_perms.hi(),
    {
        let ghost old_perms = *old(perms);
        let ghost old_out_perms = *old(out_perms);
        let ghost old_out_lo = out_lo;
        let ghost old_lo = lo;
        let mid = lo + (hi - lo) / 2;
        let out_mid = out_lo + (hi - lo) / 2;
        if mid == lo {
            return Ok(());
        }

        if hi - lo < threshold {
            merge_sort(&*arr, lo, hi, Tracked(perms), &*out_arr, out_lo, Tracked(out_perms));
            return Ok(());
        }

        let Tracked(right_perms) = (&*arr).split_off(mid, Tracked(perms));
        let Tracked(left_perms) = (&*arr).split_off(lo, Tracked(perms));

        let Tracked(out_right_perms) = (&*out_arr).split_off(out_mid, Tracked(out_perms));
        let Tracked(out_left_perms) = (&*out_arr).split_off(out_lo, Tracked(out_perms));

        let arr_r1 = Arc::clone(&arr);
        let arr_r2 = Arc::clone(&arr);

        let out_arr_r1 = Arc::clone(&out_arr);
        let out_arr_r2 = Arc::clone(&out_arr);

        let left_perms = vstd::thread::spawn(move || -> (ret: Result<(Tracked<Region<i32>>, Tracked<Region<i32>>), ()>)
            ensures
                ret.is_ok() ==> <Array<i32> as RegionArray>::wf(*arr, ret.unwrap().0@) && ret.unwrap().0@.lo() == lo && ret.unwrap().0@.hi() == mid,
                ret.is_ok() ==> <Array<i32> as RegionArray>::wf(*out_arr,ret.unwrap().1@) && ret.unwrap().1@.lo() == out_lo && ret.unwrap().1@.hi() == out_mid,
            {
                let tracked mut left_perms = left_perms;
                let tracked mut out_left_perms = out_left_perms;
                let ghost old_left_perms = left_perms;
                let ghost old_out_left_perms = out_left_perms;
                let t = merge_sort_parallel(arr_r1, lo, mid, Tracked(&mut left_perms), out_arr_r1, out_lo, Tracked(&mut out_left_perms), threshold);
                if t.is_err() {
                    return Err(());
                } else {
                    Ok((Tracked(left_perms), Tracked(out_left_perms)))
                }
            }
        );

        let right_perms = vstd::thread::spawn(move || -> (ret: Result<(Tracked<Region<i32>>, Tracked<Region<i32>>), ()>)
            ensures
                    ret.is_ok() ==> <Array<i32> as RegionArray>::wf(*arr, ret.unwrap().0@) && ret.unwrap().0@.lo() == mid && ret.unwrap().0@.hi() == old_perms.hi(),
                    ret.is_ok() ==> <Array<i32> as RegionArray>::wf(*out_arr, ret.unwrap().1@) && ret.unwrap().1@.lo() == out_mid && ret.unwrap().1@.hi() == old_out_perms.hi(),
            {
                let tracked mut right_perms = right_perms;
                let tracked mut out_right_perms = out_right_perms;
                match merge_sort_parallel(arr_r2, mid, hi, Tracked(&mut right_perms), out_arr_r2, out_mid, Tracked(&mut out_right_perms), threshold) {
                    Ok(()) => Ok((Tracked(right_perms), Tracked(out_right_perms))),
                    Err(_) => Err(()),
                }
            }
        );

        let left_perms = left_perms.join();
        let right_perms = right_perms.join();

        let ((Tracked(mut left_perms), Tracked(mut out_left_perms)), (Tracked(mut right_perms), Tracked(mut out_right_perms))) = match (left_perms, right_perms) {
            (Result::Ok(Ok(l)), Result::Ok(Ok(r))) => {
                (l, r)
            },
            _ => {
                return Result::Err("error while joining threads");
            }
        };
        (&*arr).merge(Tracked(&mut left_perms), Tracked(right_perms));
        (&*arr).merge(Tracked(perms), Tracked(left_perms));
        (&*out_arr).merge(Tracked(&mut out_left_perms), Tracked(out_right_perms));
        (&*out_arr).merge(Tracked(out_perms), Tracked(out_left_perms));

        merge(&arr, Tracked(perms), lo, mid, mid, hi, &out_arr, Tracked(out_perms), out_lo);
        while lo < hi
            invariant
                perms.lo() == old_perms.lo(),
                perms.hi() == old_perms.hi(),
                perms.lo() <= lo <= hi <= perms.hi() <= arr.len(),
                <Array<i32> as RegionArray>::wf(*arr, *perms),
                <Array<i32> as RegionArray>::wf(*out_arr, *out_perms),
                out_perms.lo() == old(out_perms).lo(),
                out_perms.hi() == old(out_perms).hi(),
                out_perms.lo() <= out_lo <= out_perms.hi() <= out_arr.len(),
                out_lo <= old_out_lo + hi - old_lo <= old(out_perms).hi(),
                out_lo - old_out_lo == lo - old_lo,
        {
            let e = *<Array<i32> as RegionArray>::read(&*out_arr, out_lo, Tracked(out_perms));
            <Array<i32> as RegionArray>::replace(&*arr, lo, e, Tracked(perms));
            out_lo += 1;
            lo += 1;
        }
        Ok(())
    }

    #[test]
    fn test_par_array() {
        let (arr, Tracked(mut perms)) = <Array<i32> as RegionArray>::new(vec![5, 4, 3, 2, 1]);
        let len = arr.length();
        let arr = Arc::new(arr);
        let (out_arr, Tracked(mut out_perms)) = <Array<i32> as RegionArray>::new(vec![0, 0, 0, 0, 0]);
        let out_arr = Arc::new(out_arr);
        merge_sort_parallel(Arc::clone(&arr), 0, len, Tracked(&mut perms), Arc::clone(&out_arr), 0, Tracked(&mut out_perms), 2).unwrap();
        let arr = <Array<i32> as RegionArray>::clone_to_vec(&arr, Tracked(&perms));
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
}


}
