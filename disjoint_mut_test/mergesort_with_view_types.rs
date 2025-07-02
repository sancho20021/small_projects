use super::*;
use std::sync::Arc;

fn merge(
    array: &Array<i32>,
    Tracked(perms): Tracked<&Region<i32>>,
    mut left_lo: usize, left_hi: usize,
    mut right_lo: usize, right_hi: usize,
    out_array: &Array<i32>,
    Tracked(out_perms): Tracked<&mut {&lo, &hi, &mut perms} Region<i32>>,
    mut out_lo: usize,
)
    requires
        perms.lo() <= left_lo <= left_hi <= perms.hi() <= array.len(),
        perms.lo() <= right_lo <= right_hi <= perms.hi() <= array.len(),
        left_hi - left_lo + right_hi - right_lo <= usize::MAX,
        region_array::wf(*array, (*perms)),
        region_array::wf(*out_array, *old(out_perms)),
        out_perms.lo() <= out_lo <= out_lo + (left_hi - left_lo + right_hi - right_lo) <= out_perms.hi() <= out_array.len(),
        out_lo + right_hi - right_lo + left_hi - left_lo <= out_perms.hi(),
    ensures
        region_array::wf(*out_array, *out_perms),
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
                old_out_lo + right_hi - old_right_lo + left_hi - old_left_lo <= out_perms.hi(),
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
                old_out_lo + right_hi - old_right_lo + left_hi - old_left_lo <= out_perms.hi(),
        {
            let e = *region_array::read(array, right_lo, Tracked(perms));
            region_array::replace(out_array, out_lo, e, Tracked(out_perms));
            right_lo += 1;
            out_lo += 1;
        }
    }
}

pub fn merge_sort(
    arr: &Array<i32>,
    mut lo: usize, hi: usize,
    Tracked(perms): Tracked<&mut {&lo, &hi, &mut perms} Region<i32>>,
    out_arr: &Array<i32>,
    mut out_lo: usize,
    Tracked(out_perms): Tracked<&mut {&lo, &hi, &mut perms} Region<i32>>,
)
    requires
        perms.lo() <= lo <= hi <= perms.hi() <= arr.len(),
        region_array::wf(*arr, (*perms)),
        out_perms.lo() <= out_lo <= out_lo + hi - lo <= out_perms.hi() <= out_arr.len(),
        region_array::wf(*out_arr, (*old(out_perms))),
    ensures
        region_array::wf(*arr, (*perms)),
        region_array::wf(*out_arr, *out_perms),
{
    let ghost old_out_lo = out_lo;
    let ghost old_lo = lo;
    let mid = lo + (hi - lo) / 2;
    if mid == lo {
        return;
    }

    merge_sort(arr, lo, mid, Tracked(perms), out_arr, out_lo, Tracked(out_perms));
    let ghost perms1 = *perms;
    merge_sort(arr, mid, hi, Tracked(perms), out_arr, out_lo, Tracked(out_perms));

    merge(arr, Tracked(perms), lo, mid, mid, hi, out_arr, Tracked(out_perms), out_lo);
    while lo < hi
        invariant
            perms.lo() <= lo <= hi <= perms.hi() <= arr.len(),
            region_array::wf(*arr, (*perms)),
            region_array::wf(*out_arr, *out_perms),
            out_perms.lo() <= out_lo <= out_perms.hi() <= out_arr.len(),
            out_lo <= old_out_lo + hi - old_lo <= out_perms.hi(),
            out_lo - old_out_lo == lo - old_lo,
    {
        let ghost perms_prev = *perms;
        let e = *region_array::read(out_arr, out_lo, Tracked(out_perms));
        region_array::replace(arr, lo, e, Tracked(perms));
        out_lo += 1;
        lo += 1;
    }
}

pub fn merge_sort_parallel(
    arr: Arc<Array<i32>>,
    mut lo: usize, hi: usize,
    Tracked(perms): Tracked<&mut {&lo, &hi, &mut perms} Region<i32>>,
    out_arr: Arc<Array<i32>>,
    mut out_lo: usize,
    Tracked(out_perms): Tracked<&mut {&lo, &hi, &mut perms} Region<i32>>,
    threshold: usize
) -> (ret: Result<(), &'static str>)
    requires
        perms.lo() <= lo <= hi <= perms.hi() <= arr.len(),
        region_array::wf(*arr, (*old(perms))),
        out_perms.lo() <= out_lo <= out_lo + hi - lo <= out_perms.hi() <= out_arr.len(),
        region_array::wf(*out_arr, *old(out_perms)),
    ensures
        ret.is_ok() ==> region_array::wf(*arr, (*perms)),
        ret.is_ok() ==> region_array::wf(*out_arr, *out_perms),
{
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
                ret.is_ok() ==> region_array::wf(*arr, ret.unwrap().0@) && ret.unwrap().0@.lo() == mid && ret.unwrap().0@.hi() == perms.hi(),
                ret.is_ok() ==> region_array::wf(*out_arr, ret.unwrap().1@) && ret.unwrap().1@.lo() == out_mid && ret.unwrap().1@.hi() == out_perms.hi(),
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

    proof {
        region_array::merge(&*arr, &mut left_perms, right_perms);
        region_array::merge(&*arr, perms, left_perms);
        region_array::merge(&*out_arr, &mut out_left_perms, out_right_perms);
        region_array::merge(&*out_arr, out_perms, out_left_perms);
    }

    merge(&arr, Tracked(perms), lo, mid, mid, hi, &out_arr, Tracked(out_perms), out_lo);
    while lo < hi
        invariant
            perms.lo() <= lo <= hi <= perms.hi() <= arr.len(),
            region_array::wf(*arr, *perms),
            region_array::wf(*out_arr, *out_perms),
            out_perms.lo() <= out_lo <= out_perms.hi() <= out_arr.len(),
            out_lo <= old_out_lo + hi - old_lo <= out_perms.hi(),
            out_lo - old_out_lo == lo - old_lo,
    {
        let e = *region_array::read(&*out_arr, out_lo, Tracked(out_perms));
        region_array::replace(&*arr, lo, e, Tracked(perms));
        out_lo += 1;
        lo += 1;
    }
    Ok(())
}
