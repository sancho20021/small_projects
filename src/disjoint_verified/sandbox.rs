use vstd::prelude::*;

verus!{

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
    // let uid1 = uid();
    // let uid2 = uid();

    // assert(uid1 != uid2);

    {
        proof {
            let s3 = set_of_n(3);
            assert(s3.contains(2));
        }
    }

    {
        let (_, Tracked(uid)) = PPtr::<()>::empty();
        let mut set: Ghost<Set<PointsTo<()>>> = Ghost(Set::empty());
        assert(set@.insert(uid).len() > 0);
    }
}

}
