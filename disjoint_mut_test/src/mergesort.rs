use vstd::prelude::*;

use std::sync::Arc;

verus! {

use crate::{
    permissions_array::Array,
    region_array::{self, Region},
};

pub struct ArrayForSorting<T> {
    pub array: Arc<Array<T>>,
    pub perms: Tracked<Region<T>>,
}

impl<T> ArrayForSorting<T> {
    pub fn new(data: Vec<T>) -> (res: Self)
        where Self: std::marker::Sized,
        ensures
            data.len() == res.array.len(),
            region_array::wf(*res.array, res.perms@),
    {
        let (array, perms) = region_array::new(data);
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
        region_array::wf(*self.array, self.perms@),
        self.perms@.lo() == 0,
        self.perms@.hi() == self.array.len()
    {
        region_array::clone_to_vec(&self.array, Tracked(self.perms.borrow()))
    }
}

pub fn merge_sort(
    arr: &mut ArrayForSorting<i32>,
    out_arr: &mut ArrayForSorting<i32>,
)
    requires
        old(arr).perms@.lo() == 0,
        old(arr).perms@.hi() == old(arr).array.len(),
        region_array::wf(*old(arr).array, (old(arr).perms@)),
        old(out_arr).perms@.lo() == 0,
        old(out_arr).perms@.hi() == old(out_arr).array.len(),
        region_array::wf(*old(out_arr).array, (old(out_arr).perms@)),
        old(arr).array.len() == old(out_arr).array.len(),
    ensures
        region_array::wf(*arr.array, (arr.perms@)),
        arr.perms@.lo() == old(arr).perms@.lo(),
        arr.perms@.hi() == old(arr).perms@.hi(),
{
    _merge_sort(
        &arr.array,
        0,
        (&*arr.array).length(),
        Tracked(arr.perms.borrow_mut()),
        &out_arr.array,
        0,
        Tracked(out_arr.perms.borrow_mut()),
    )
}

pub fn merge_sort_parallel(
    arr: &mut ArrayForSorting<i32>,
    out_arr: &mut ArrayForSorting<i32>,
    threshold: usize,
) -> (ret: Result<(), ()>)
    requires
        old(arr).perms@.lo() == 0,
        old(arr).perms@.hi() == old(arr).array.len(),
        region_array::wf(*old(arr).array, (old(arr).perms@)),
        old(out_arr).perms@.lo() == 0,
        old(out_arr).perms@.hi() == old(out_arr).array.len(),
        region_array::wf(*old(out_arr).array, (old(out_arr).perms@)),
        old(arr).array.len() == old(out_arr).array.len(),
    ensures
        ret.is_ok() ==> region_array::wf(*arr.array, (arr.perms@)),
        ret.is_ok() ==> arr.perms@.lo() == old(arr).perms@.lo(),
        ret.is_ok() ==> arr.perms@.hi() == old(arr).perms@.hi(),
{
    _merge_sort_parallel(
        Arc::clone(&arr.array),
        0,
        (&*arr.array).length(),
        Tracked(arr.perms.borrow_mut()),
        Arc::clone(&out_arr.array),
        0,
        Tracked(out_arr.perms.borrow_mut()),
        threshold,
    )
}


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
        region_array::wf(*array, (*perms)),
        region_array::wf(*out_array, *old(out_perms)),
        old(out_perms).lo() <= out_lo <= out_lo + (left_hi - left_lo + right_hi - right_lo) <= old(out_perms).hi() <= out_array.len(),
        out_lo + right_hi - right_lo + left_hi - left_lo <= old(out_perms).hi(),
    ensures
        region_array::wf(*out_array, *out_perms),
        old(out_perms).lo() == out_perms.lo(),
        old(out_perms).hi() == out_perms.hi(),
{
    let ghost old_out_lo = out_lo;
    let ghost old_left_lo = left_lo;
    let ghost old_right_lo = right_lo;
    while left_lo < left_hi && right_lo < right_hi
        invariant
            region_array::wf(*array, (*perms)),
            perms.lo() <= left_lo <= left_hi <= perms.hi() <= array.len(),
            perms.lo() <= right_lo <= right_hi <= perms.hi() <= array.len(),
            region_array::wf(*out_array, *out_perms),
            out_perms.lo() <= out_lo <= old_out_lo + (left_hi - old_left_lo + right_hi - old_right_lo) <= out_perms.hi() <= out_array.len(),
            out_lo == old_out_lo + (left_lo - old_left_lo) + (right_lo - old_right_lo),
            old(out_perms).lo() == out_perms.lo(),
            old(out_perms).hi() == out_perms.hi(),
    {
        let element: i32;
        if region_array::read(array, left_lo, Tracked(perms)) < region_array::read(array, right_lo, Tracked(perms)) {
            element = *region_array::read(array, left_lo, Tracked(perms));
            left_lo += 1;
        } else {
            element = *region_array::read(array, right_lo, Tracked(perms));
            right_lo += 1;
        }
        region_array::replace(out_array, out_lo, element, Tracked(out_perms));
        out_lo += 1;
    }

    if left_lo < left_hi {
        while left_lo < left_hi
            invariant
                region_array::wf(*array, (*perms)),
                left_hi - left_lo + right_hi - right_lo + out_lo - old_out_lo == left_hi - old_left_lo + right_hi - old_right_lo,
                perms.lo() <= left_lo <= left_hi <= perms.hi() <= array.len(),
                perms.lo() <= right_lo <= right_hi,
                region_array::wf(*out_array, *out_perms),
                out_perms.lo() <= out_lo <= out_perms.hi() <= out_array.len(),
                old_out_lo + right_hi - old_right_lo + left_hi - old_left_lo <= old(out_perms).hi(),
                old(out_perms).lo() == out_perms.lo(),
                old(out_perms).hi() == out_perms.hi(),
        {
            let e = *region_array::read(array, left_lo, Tracked(perms));
            region_array::replace(out_array, out_lo, e, Tracked(out_perms));
            left_lo += 1;
            out_lo += 1;
        }
    } else if right_lo < right_hi {
        while right_lo < right_hi
            invariant
                region_array::wf(*array, (*perms)),
                left_hi - left_lo + right_hi - right_lo + out_lo - old_out_lo == left_hi - old_left_lo + right_hi - old_right_lo,
                perms.lo() <= right_lo <= right_hi <= perms.hi() <= array.len(),
                perms.lo() <= left_lo <= left_hi,
                region_array::wf(*out_array, *out_perms),
                out_perms.lo() <= out_lo <= out_perms.hi() <= out_array.len(),
                old_out_lo + right_hi - old_right_lo + left_hi - old_left_lo <= old(out_perms).hi(),
                old(out_perms).lo() == out_perms.lo(),
                old(out_perms).hi() == out_perms.hi(),
        {
            let e = *region_array::read(array, right_lo, Tracked(perms));
            region_array::replace(out_array, out_lo, e, Tracked(out_perms));
            right_lo += 1;
            out_lo += 1;
        }
    }
}

fn _merge_sort(
    arr: &Array<i32>,
    mut lo: usize, hi: usize,
    Tracked(perms): Tracked<&mut Region<i32>>,
    out_arr: &Array<i32>,
    mut out_lo: usize,
    Tracked(out_perms): Tracked<&mut Region<i32>>,
)
    requires
        old(perms).lo() <= lo <= hi <= old(perms).hi() <= arr.len(),
        region_array::wf(*arr, (*old(perms))),
        old(out_perms).lo() <= out_lo <= out_lo + hi - lo <= old(out_perms).hi() <= out_arr.len(),
        region_array::wf(*out_arr, (*old(out_perms))),
    ensures
        region_array::wf(*arr, (*perms)),
        perms.lo() == old(perms).lo(),
        perms.hi() == old(perms).hi(),
        region_array::wf(*out_arr, *out_perms),
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
    _merge_sort(arr, lo, mid, Tracked(perms), out_arr, out_lo, Tracked(out_perms));
    let ghost perms1 = *perms;
    _merge_sort(arr, mid, hi, Tracked(perms), out_arr, out_lo, Tracked(out_perms));

    merge(arr, Tracked(perms), lo, mid, mid, hi, out_arr, Tracked(out_perms), out_lo);
    while lo < hi
        invariant
            perms.lo() == old(perms).lo(),
            perms.hi() == old(perms).hi(),
            perms.lo() <= lo <= hi <= perms.hi() <= arr.len(),
            region_array::wf(*arr, (*perms)),
            region_array::wf(*out_arr, *out_perms),
            out_perms.lo() == old(out_perms).lo(),
            out_perms.hi() == old(out_perms).hi(),
            out_perms.lo() <= out_lo <= out_perms.hi() <= out_arr.len(),
            out_lo <= old_out_lo + hi - old_lo <= old(out_perms).hi(),
            out_lo - old_out_lo == lo - old_lo,
    {
        let ghost perms_prev = *perms;
        let e = *region_array::read(out_arr, out_lo, Tracked(out_perms));
        region_array::replace(arr, lo, e, Tracked(perms));
        out_lo += 1;
        lo += 1;
    }
}

fn _merge_sort_parallel(
    arr: Arc<Array<i32>>,
    mut lo: usize, hi: usize,
    Tracked(perms): Tracked<&mut Region<i32>>,
    out_arr: Arc<Array<i32>>,
    mut out_lo: usize,
    Tracked(out_perms): Tracked<&mut Region<i32>>,
    threshold: usize
) -> (ret: Result<(), ()>)
    requires
        old(perms).lo() <= lo <= hi <= old(perms).hi() <= arr.len(),
        region_array::wf(*arr, (*old(perms))),
        old(out_perms).lo() <= out_lo <= out_lo + hi - lo <= old(out_perms).hi() <= out_arr.len(),
        region_array::wf(*out_arr, *old(out_perms)),
    ensures
        ret.is_ok() ==> region_array::wf(*arr, (*perms)),
        ret.is_ok() ==> old(perms).lo() == perms.lo() && old(perms).hi() == perms.hi(),
        ret.is_ok() ==> region_array::wf(*out_arr, *out_perms),
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

    if hi - lo <= threshold {
        _merge_sort(&*arr, lo, hi, Tracked(perms), &*out_arr, out_lo, Tracked(out_perms));
        return Ok(());
    }

    let tracked right_perms = region_array::split_off(&*arr, mid, perms);
    let tracked left_perms = region_array::split_off(&*arr, lo, perms);

    let tracked out_right_perms = region_array::split_off(&*out_arr, out_mid, out_perms);
    let tracked out_left_perms = region_array::split_off(&*out_arr, out_lo, out_perms);

    let arr_r1 = Arc::clone(&arr);
    let arr_r2 = Arc::clone(&arr);

    let out_arr_r1 = Arc::clone(&out_arr);
    let out_arr_r2 = Arc::clone(&out_arr);

    let left_perms = vstd::thread::spawn(move || -> (ret: Result<(Tracked<Region<i32>>, Tracked<Region<i32>>), ()>)
        ensures
            ret.is_ok() ==> region_array::wf(*arr, ret.unwrap().0@) && ret.unwrap().0@.lo() == lo && ret.unwrap().0@.hi() == mid,
            ret.is_ok() ==> region_array::wf(*out_arr,ret.unwrap().1@) && ret.unwrap().1@.lo() == out_lo && ret.unwrap().1@.hi() == out_mid,
        {
            let tracked mut left_perms = left_perms;
            let tracked mut out_left_perms = out_left_perms;
            let ghost old_left_perms = left_perms;
            let ghost old_out_left_perms = out_left_perms;
            let t = _merge_sort_parallel(arr_r1, lo, mid, Tracked(&mut left_perms), out_arr_r1, out_lo, Tracked(&mut out_left_perms), threshold);
            if t.is_err() {
                Err(())
            } else {
                Ok((Tracked(left_perms), Tracked(out_left_perms)))
            }
        }
    );

    match _merge_sort_parallel(arr_r2, mid, hi, Tracked(&mut right_perms), out_arr_r2, out_mid, Tracked(&mut out_right_perms), threshold) {
        Ok(()) => {},
        Err(_) => {return Err(());},
    };

    let left_perms = left_perms.join();

    let (Tracked(mut left_perms), Tracked(mut out_left_perms)) = match left_perms {
        Result::Ok(Ok(l)) => {
            l
        },
        _ => {
            return Result::Err(());
        }
    };

    proof {
        region_array::merge(&*arr, &mut left_perms, right_perms);
        region_array::merge(&*arr, perms, left_perms);
        region_array::merge(&*out_arr, &mut out_left_perms, out_right_perms);
        region_array::merge(&*out_arr, out_perms, out_left_perms);
    }

    merge(&arr, Tracked(perms), lo, mid, mid, hi, &out_arr, Tracked(out_perms), out_lo);
    while lo < hi
        invariant
            perms.lo() == old_perms.lo(),
            perms.hi() == old_perms.hi(),
            perms.lo() <= lo <= hi <= perms.hi() <= arr.len(),
            region_array::wf(*arr, *perms),
            region_array::wf(*out_arr, *out_perms),
            out_perms.lo() == old(out_perms).lo(),
            out_perms.hi() == old(out_perms).hi(),
            out_perms.lo() <= out_lo <= out_perms.hi() <= out_arr.len(),
            out_lo <= old_out_lo + hi - old_lo <= old(out_perms).hi(),
            out_lo - old_out_lo == lo - old_lo,
    {
        let e = *region_array::read(&*out_arr, out_lo, Tracked(out_perms));
        region_array::replace(&*arr, lo, e, Tracked(perms));
        out_lo += 1;
        lo += 1;
    }
    Ok(())
}

#[test]
fn test_par_array() {
    let (arr, Tracked(mut perms)) = region_array::new(vec![5, 4, 3, 2, 1]);
    let len = arr.length();
    let arr = Arc::new(arr);
    let (out_arr, Tracked(mut out_perms)) = region_array::new(vec![0, 0, 0, 0, 0]);
    let out_arr = Arc::new(out_arr);
    _merge_sort_parallel(Arc::clone(&arr), 0, len, Tracked(&mut perms), Arc::clone(&out_arr), 0, Tracked(&mut out_perms), 2).unwrap();
    let arr = region_array::clone_to_vec(&arr, Tracked(&perms));
    assert_eq!(arr, vec![1, 2, 3, 4, 5]);
}

}
