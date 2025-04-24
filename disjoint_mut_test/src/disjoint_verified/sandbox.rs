use vstd::prelude::*;

verus! {

use vstd::simple_pptr::PPtr;
use vstd::simple_pptr::PointsTo;

type UID = PPtr<()>;

pub fn uid() -> UID {
    let (p, perm) = PPtr::empty();
    p
}

proof fn set_of_n(size: usize) -> (res: Set<usize>)
    ensures
        forall |x: usize| x < size ==> res.contains(x)
{
    Set::new(|x: usize| x < size)
}


fn main() {

}

}
