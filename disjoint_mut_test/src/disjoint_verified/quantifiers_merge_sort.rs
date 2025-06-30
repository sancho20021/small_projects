use vstd::prelude::*;

verus!{

use crate::disjoint_verified::exec_pcell::*;
use std::sync::Arc;


fn merge(array: &Array<i32>, perms: &Perms<i32>, mut left_lo: usize, left_hi: usize, mut right_lo: usize, right_hi: usize) -> (res: Vec<i32>)
    requires
        left_lo <= left_hi <= array.len(),
        right_lo <= right_hi <= array.len(),
        left_hi - left_lo + right_hi - right_lo <= usize::MAX,
        array.wf((*perms)@),
        forall |i: usize| (left_lo <= i < left_hi) || (right_lo <= i < right_hi) ==> array.available(i, (*perms)@),
    ensures
        res.len() == left_hi - left_lo + right_hi - right_lo,
{
    let ghost old_left_lo = left_lo;
    let ghost old_right_lo = right_lo;
    let mut ret: Vec<i32> = Vec::with_capacity(left_hi - left_lo + (right_hi - right_lo));
    while left_lo < left_hi && right_lo < right_hi
        invariant
            array.wf((*perms)@),
            forall |i: usize| (left_lo <= i < left_hi) || (right_lo <= i < right_hi) ==> array.available(i, (*perms)@),
            left_lo <= left_hi <= array.len(),
            right_lo <= right_hi <= array.len(),
            left_hi - left_lo + right_hi - right_lo + ret.len() == left_hi - old_left_lo + right_hi - old_right_lo,

    {
        let element: i32;
        if array.read(left_lo, perms) < array.read(right_lo, perms) {
            element = *array.read(left_lo, perms);
            left_lo += 1;
        } else {
            element = *array.read(right_lo, perms);
            right_lo += 1;
        }
        ret.push(element);
    }

    if left_lo < left_hi {
        while left_lo < left_hi
            invariant
                array.wf((*perms)@),
                forall |i: usize| left_lo <= i < left_hi ==> array.available(i, (*perms)@),
                left_hi - left_lo + right_hi - right_lo + ret.len() == left_hi - old_left_lo + right_hi - old_right_lo,
                left_lo <= left_hi <= array.len(),
        {
            ret.push(*array.read(left_lo, perms));
            left_lo += 1;
        }
    } else if right_lo < right_hi {
        while right_lo < right_hi
            invariant
                array.wf((*perms)@),
                forall |i: usize| right_lo <= i < right_hi ==> array.available(i, (*perms)@),
                left_hi - left_lo + right_hi - right_lo + ret.len() == left_hi - old_left_lo + right_hi - old_right_lo,
                right_lo <= right_hi <= array.len(),
        {
            ret.push(*array.read(right_lo, perms));
            right_lo += 1;
        }
    }
    ret
}

pub fn merge_sort(arr: &Array<i32>, lo: usize, hi: usize, perms: &mut Perms<i32>)
    requires
        lo <= hi <= arr.len(),
        arr.wf((*old(perms))@),
        forall |i: usize| lo <= i < hi ==> arr.available(i, (*old(perms))@),
    ensures
        arr.wf((*perms)@),
        forall |i: usize| lo <= i < hi ==> arr.available(i, (*perms)@),
        arr.availability_unchanged((*old(perms))@, (*perms)@),
{
    let mid = lo + (hi - lo) / 2;
    if mid == lo {
        return;
    }

    let ghost old_perms = *perms;
    merge_sort(arr, lo, mid, perms);
    let ghost perms1 = *perms;
    merge_sort(arr, mid, hi, perms);

    let res = merge(arr, perms, lo, mid, mid, hi);
    let mut i: usize = 0;
    while i < res.len()
        invariant
            lo + i <= hi <= arr.len(),
            hi - lo == res.len(),
            arr.wf((*perms)@),
            forall |i: usize| lo <= i < hi ==> arr.available(i, (*perms)@),
            arr.availability_unchanged(old_perms@, (*perms)@),
    {
        let ghost perms_prev = *perms;
        arr.replace(lo + i, res[i], perms);
        i += 1;
    }
}

pub fn merge_sort_parallel(arr: Arc<Array<i32>>, lo: usize, hi: usize, perms: &mut Perms<i32>, threshold: usize) -> (ret: Result<(), &'static str>)
    requires
        lo <= hi <= arr.len(),
        (*arr).wf((*old(perms))@),
        forall |i: usize| lo <= i < hi ==> (*arr).available(i, (*old(perms))@),
    ensures
        ret.is_ok() ==> (*arr).wf((*perms)@),
        ret.is_ok() ==> forall |i: usize| lo <= i < hi ==> arr.available(i, (*perms)@),
        ret.is_ok() ==> (*arr).availability_unchanged((*old(perms))@, (*perms)@),
{
    let ghost old_perms = (*old(perms))@;
    let mid = lo + (hi - lo) / 2;
    if mid == lo {
        return Ok(());
    }

    if hi - lo < threshold {
        merge_sort(&*arr, lo, hi, perms);
        return Ok(());
    }

    let ghost right_keys = Set::<usize>::new(|i: usize| mid <= i < hi);
    let ghost left_keys = Set::<usize>::new(|i: usize| lo <= i < mid);
    assert(forall |i: usize| lo <= i < hi ==> arr.available(i, (*perms)@) ==> perms@.contains_key(i));

    let left_perms = Tracked(perms.borrow_mut().tracked_remove_keys(left_keys));
    assert(arr.wf(left_perms@)) by {
        arr.submap_wf((*old(perms))@, left_perms@);
    }

    let right_perms = Tracked(perms.borrow_mut().tracked_remove_keys(right_keys));
    assert(left_perms@.dom().disjoint(right_perms@.dom()));
    assert(arr.wf((*perms)@)) by {
        assert(perms@.submap_of((*old(perms))@));
        arr.submap_wf((*old(perms))@, (*perms)@);
    }
    assert(right_perms@.submap_of(old(perms)@));
    assert(arr.wf(right_perms@)) by {
        arr.submap_wf((*old(perms))@, right_perms@);
    }

    let arr_r1 = Arc::clone(&arr);
    let arr_r2 = Arc::clone(&arr);

    let left_perms = vstd::thread::spawn(move || -> (ret: Result<Perms<i32>, ()>)
        ensures
            ret.is_ok() ==> arr.wf(ret.unwrap()@),
            ret.is_ok() ==> forall |i: usize| lo <= i < mid <==> arr.available(i, ret.unwrap()@),
        {
            let mut left_perms = left_perms;
            let ghost old_left_perms = left_perms@;
            let t = merge_sort_parallel(arr_r1, lo, mid, &mut left_perms, threshold);
            if t.is_err() {
                return Err(());
            } else {
                Ok(left_perms)
            }
        }
    );

    let right_perms = vstd::thread::spawn(move || -> (ret: Result<Perms<i32>, ()>)
        ensures
                ret.is_ok() ==> arr.wf(ret.unwrap()@),
                ret.is_ok() ==> forall |i: usize| mid <= i < hi <==> arr.available(i, ret.unwrap()@),
        {
            let mut right_perms = right_perms;
            match merge_sort_parallel(arr_r2, mid, hi, &mut right_perms, threshold) {
                Ok(()) => Ok(right_perms),
                Err(_) => Err(()),
            }
        }
    );

    let left_perms = left_perms.join();
    let right_perms = right_perms.join();

    let (mut left_perms, mut right_perms) = match (left_perms, right_perms) {
        (Result::Ok(Ok(l)), Result::Ok(Ok(r))) => {
            (l, r)
        },
        _ => {
            return Result::Err("error while joining threads");
        }
    };

    proof {
        let ghost perms0 = (*perms)@;
        perms.borrow_mut().tracked_union_prefer_right(left_perms@);
        assert(arr.wf((*perms)@)) by {
            arr.union_wf(perms0, left_perms@);
        }
        let ghost perms1 = (*perms)@;

        perms.borrow_mut().tracked_union_prefer_right(right_perms@);
        let ghost perms2 = (*perms)@;
        assert(arr.wf(perms2)) by {
            arr.union_wf(perms1, right_perms@);
        }
        assert forall |i: usize| (i < lo || i >= hi) && #[trigger] arr.available(i, perms2) implies arr.available(i, perms0) by {
            assert(!arr.available(i, right_perms@));
            assert(!arr.available(i, left_perms@));
        }

        assert forall |i: usize| lo <= i < hi implies arr.available(i, perms2) by {
            if i < mid {
                assert(arr.available(i, left_perms@));
            } else {
                assert(arr.available(i, right_perms@));
            }
        }
    }
    let res = merge(&arr, perms, lo, mid, mid, hi);
    let mut i = 0;
    while i < res.len()
        invariant
            res.len() == hi - lo,
            lo + res.len() <= arr.len(),
            arr.wf(perms@),
            forall |i: usize| lo <= i < hi ==> arr.available(i, perms@),
            (*arr).availability_unchanged(old_perms, perms@),
    {
        (*arr).replace(lo + i, res[i], perms);
        i += 1;
    }
    Ok(())
}

#[test]
fn test_par_array() {
    let (arr, mut perms) = Array::new(vec![5, 4, 3, 2, 1]);
    let len = arr.length();
    let arr = Arc::new(arr);
    merge_sort_parallel(Arc::clone(&arr), 0, len, &mut perms, 2).unwrap();
    let arr = arr.clone_to_vec(&perms);
    assert_eq!(arr, vec![1, 2, 3, 4, 5]);
}

}
