use vstd::prelude::*;

verus!{

use vstd::set_lib;

use crate::permissions_array::{Array, SpecPerms};


pub proof fn split_perms<T>(
    arr: &Array<T>,
    tracked perms: &mut SpecPerms<T>,
    a: Set<usize>,
    b: Set<usize>,
) -> (tracked res: (SpecPerms<T>, SpecPerms<T>))
requires
    arr.wf(*old(perms)),
    a.subset_of(perms.dom()),
    b.subset_of(perms.dom()),
    a.disjoint(b),
ensures
    arr.wf(*perms),
    res.0.dom() == a,
    arr.wf(res.0),
    res.1.dom() == b,
    arr.wf(res.1),
    perms.dom() == old(perms).dom().difference(a.union(b))
{

}

}
