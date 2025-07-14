use vstd::prelude::*;

use crate::permissions_array::SpecPerms;

verus! {

use super::permissions_array::Array;

pub tracked struct Region<T> {
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


pub closed spec fn wf<T>(aself: Array<T>, region: Region<T>) -> bool {
    region.lo <= region.hi <= aself.len() && aself.wf(region.perms) && forall |i: usize| region.lo <= i < region.hi ==> aself.available(i, region.perms)
}

pub proof fn split_off<T>(aself: &Array<T>, tracked m: usize, tracked region: &mut Region<T>) -> (tracked res: Region<T>)
where
    requires
        old(region).lo() <= m@ < old(region).hi(),
        wf(*aself,*old(region)),
    ensures
        wf(*aself,*region),
        region.lo() == old(region).lo(),
        region.hi() == m,
        wf(*aself,res),
        res.lo() == m,
        res.hi() == old(region).hi()
{
    let ghost old_perms = region.perms;
    let ghost right_keys = Set::<usize>::new(|i: usize| m <= i < region.hi());
    assert(forall |i: usize| region.lo() <= i < region.hi() ==> aself.available(i, region.perms) ==> region.perms.contains_key(i));

    let tracked right_perms = region.perms.tracked_remove_keys(right_keys);
    let tracked right = Region {
        lo: m,
        hi: region.hi,
        perms: right_perms,
    };
    region.hi = m;
    assert(aself.wf(region.perms)) by {
        aself.submap_wf(old_perms, region.perms);
    }
    assert(aself.wf(right_perms)) by {
        aself.submap_wf(old_perms, right_perms);
    }
    right
}

pub proof fn merge<T>(aself: &Array<T>, tracked left: &mut Region<T>, tracked right: Region<T>)
    where
    requires
        wf(*aself,*old(left)),
        wf(*aself,right),
        old(left).hi() == right.lo(),
    ensures
        wf(*aself,*left),
        left.lo() == old(left).lo(),
        left.hi() == right.hi()
{
    assert(forall |i: usize| left.lo <= i < left.hi ==> aself.available(i, left.perms) ==> left.perms.contains_key(i));
    assert(forall |i: usize| right.lo <= i < right.hi ==> aself.available(i, right.perms) ==> right.perms.contains_key(i));
    let tracked right_perms = right.perms;
    left.perms.tracked_union_prefer_right(right_perms);
    left.hi = right.hi;
    assert(aself.wf(left.perms)) by {
        aself.union_wf(old(left).perms, right.perms);
    }
}

pub open spec fn len<T>(aself: &Array<T>) -> usize {
    Array::len(aself)
}

pub fn new<T>(data: Vec<T>) -> (res: (Array<T>, Tracked<Region<T>>))
where
    ensures
        data.len() == res.0.len(),
        wf(res.0, res.1@),
        res.1@.lo() == 0,
        res.1@.hi() == res.0.len()
{
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

pub fn replace<T>(aself: &Array<T>, i: usize, x: T, Tracked(perms): Tracked<&mut Region<T>>) -> (res: T)
where
    requires
        wf(*aself,(*old(perms))),
        old(perms).lo() <= i < old(perms).hi(),
    ensures
        wf(*aself,*perms),
        perms.lo() == old(perms).lo(),
        perms.hi() == old(perms).hi()
{
    <Array<T>>::replace(aself, i, x, Tracked(&mut perms.perms))
}

pub fn read<'a, T>(aself: &'a Array<T>, i: usize, Tracked(perms): Tracked<&'a Region<T>>) -> (res: &'a T)
where
    requires
        wf(*aself,*perms),
        perms.lo() <= i < perms.hi()
{
    <Array<T>>::read(aself, i, Tracked(&perms.perms))
}

pub fn clone_to_vec<T>(aself: &Array<T>, Tracked(perms): Tracked<&Region<T>>) -> Vec<T>
    where T: Clone,
        T: Clone,
    requires
        wf(*aself,*perms),
        perms.lo() == 0,
        perms.hi() == self::len(aself)
{
    <Array<T>>::clone_to_vec(aself, Tracked(&perms.perms))
}

}
