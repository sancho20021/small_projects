use vstd::prelude::*;

verus!{

// first i try without ids

use vstd::simple_pptr::*;

pub type UidGhost = PointsTo<()>;
pub type UID = PPtr<()>;
pub type BorrowId = nat;

pub struct Cell<T> {
    data: T,
    id: Tracked<UID>,
}

pub tracked struct PermSet {
    id: Tracked<UidGhost>,
    next_unique: Ghost<nat>,
    immuts: Ghost<Set<BorrowId>>,
    muts: Ghost<bool>,
}

pub struct DisjointGuard<'a, T> {
    parent: &'a Cell<T>,
    uid: Ghost<BorrowId>,
}

pub struct DisjointMutGuard<'a, T> {
    parent: &'a Cell<T>,
}

impl PermSet {
    pub closed spec fn id(self) -> UID {
        self.id@.pptr()
    }

    pub closed spec fn well_formed(self) -> bool {
        &&& self.immuts@.finite()
        &&& self.muts@ ==> self.immuts@.is_empty()
        &&& forall |x: BorrowId| self.immuts@.contains(x) ==> x < self.next_unique@
    }

    pub closed spec fn available_immut(self) -> bool {
        !self.muts@
    }

    pub closed spec fn available_mut(self) -> bool {
        self.immuts@.is_empty() && self.available_immut()
    }

    pub proof fn mut_then_immut(&self)
        requires self.available_mut(),
        ensures self.available_immut(),
    {}

    pub closed spec fn no_borrows(self) -> bool {
        &&& self.immuts@.is_empty()
        &&& !self.muts@
    }
}

pub fn new_disjoint<T>(data: T) -> (res: (Cell<T>, Tracked<PermSet>))
    ensures
        correspond(res.0, res.1@),
        res.1@.available_immut() && res.1@.available_mut(),
        res.1@.well_formed(),
        res.1@.no_borrows()
{
    let (pptr, Tracked(perm)) = PPtr::<()>::empty();
    let d = Cell {
        data,
        id: Tracked(pptr),
    };
    let p = Tracked(PermSet {
        id: Tracked(perm),
        immuts: Ghost(Set::empty()),
        muts: Ghost(false),
        next_unique: Ghost(0),
    });
    (d, p)
}

pub closed spec fn correspond<T>(d: Cell<T>, p: PermSet) -> bool {
    d.id@ == p.id()
}

proof fn insert_remove_id<T>(s: Set<T>, x: T)
    requires
        s.finite(),
        !s.contains(x),
    ensures
        s.insert(x).remove(x) == s,
    decreases
        s.len(),
{
    if s.len() == 0 {
        assert(s.insert(x).remove(x) == s);
    } else {
        let a = s.choose();
        if a != x {
            let ss = s.remove(a);
            insert_remove_id(ss, x);
            assert(ss.insert(x).remove(x) == ss);
            assert(ss.insert(a).insert(x).remove(x) == ss.insert(a));
        }
    }
}

impl<T> Cell<T> {
    closed spec fn corresponds(&self, guard: DisjointGuard<'_, T>) -> bool {
        self == &guard.parent
    }

    closed spec fn corresponds_mut(&self, guard: DisjointMutGuard<'_, T>) -> bool {
        self == &guard.parent
    }

    pub fn borrow(&self, Tracked(p): Tracked<&mut PermSet>,) -> (res: DisjointGuard<'_, T>)
        requires
            old(p).well_formed(),
            old(p).available_immut(),
            correspond(*self, *old(p)),
        ensures
            p.id() == old(p).id(),
            p.well_formed(),
            p.available_immut(),
            forall |b: DisjointGuard<'_, T>| self.borrows(old(p), b) ==> self.borrows(p, b),
            borrow_created(*old(p), *p, res),
            self.borrows(p, res),
    {
        let Ghost(mut uid) = Ghost(0);

        proof {
            let Ghost(x) = p.next_unique;
            uid = x;

            assert(forall |i: BorrowId| p.immuts@.contains(i) ==> i < uid);
            assert(!p.immuts@.contains(uid));
            assert(p.immuts@.insert(uid).remove(uid) == p.immuts@) by {
                insert_remove_id(p.immuts@, uid);
            };

            p.immuts = Ghost(p.immuts@.insert(uid));
            p.next_unique = Ghost(uid + 1);
        }

        DisjointGuard {
            parent: self,
            uid: Ghost(uid),
        }
    }

    pub fn borrow_mut(&self, Tracked(p): Tracked<&mut PermSet>) -> (res: DisjointMutGuard<'_, T>)
        requires
            old(p).well_formed(),
            old(p).available_mut(),
            correspond(*self, *old(p)),
        ensures
            p.id() == old(p).id(),
            p.well_formed(),
            !p.available_mut(),
            !p.available_immut(),
            self.borrows_mut(p, res),
    {
        proof {
            p.muts = Ghost(true);
        }
        DisjointMutGuard {
            parent: self,
        }
    }

    pub closed spec fn borrows<'a>(&'a self, p: &PermSet, borrow: DisjointGuard<'a, T>) -> bool {
        &&& correspond(*self, *p)
        &&& self.corresponds(borrow)
        &&& p.immuts@.contains(borrow.uid@)
    }

    pub closed spec fn borrows_mut<'a>(&'a self, p: &PermSet, borrow: DisjointMutGuard<'a, T>) -> bool {
        &&& correspond(*self, *p)
        &&& self.corresponds_mut(borrow)
        &&& p.muts@
    }

    pub fn terminate_mut<'a>(&'a self, Tracked(p): Tracked<&mut PermSet>, borrow: DisjointMutGuard<'a, T>)
        requires
            correspond(*self, *old(p)),
            old(p).well_formed(),
            self.borrows_mut(old(p), borrow),
        ensures
            correspond(*self, *p),
            p.well_formed(),
            p.available_mut(),
    {
        proof {
            p.muts = Ghost(false);
        }
    }

    pub fn terminate<'a>(&'a self, Tracked(p): Tracked<&mut PermSet>, borrow: DisjointGuard<'a, T>)
        requires
            correspond(*self, *old(p)),
            old(p).well_formed(),
            self.borrows(old(p), borrow),
        ensures
            correspond(*self, *p),
            p.well_formed(),
            p.available_immut(),
            borrow_terminated(*old(p), *p, borrow),
    {
        proof {
            p.immuts = Ghost(p.immuts@.remove(borrow.uid@));
        }
    }
}

pub closed spec fn borrow_terminated<T>(old: PermSet, new: PermSet, borrow: DisjointGuard<'_, T>) -> bool {
    &&& old.immuts@.remove(borrow.uid@) == new.immuts@
    &&& old.immuts@ == new.immuts@.insert(borrow.uid@)
}

pub closed spec fn borrow_created<T>(old: PermSet, new: PermSet, borrow: DisjointGuard<'_, T>) -> bool {
    &&& old.immuts@.insert(borrow.uid@) == new.immuts@
    &&& old.immuts@ == new.immuts@.remove(borrow.uid@)
}

fn main() {
    // // test examples from ghostcell
    // // 1) simple case with no owner id checking
    {
        let (c, Tracked(mut p)) = new_disjoint::<usize>(0);

        let i0a = c.borrow(Tracked(&mut p));
        let i0b = c.borrow(Tracked(&mut p));


        c.terminate(Tracked(&mut p), i0a);
        c.terminate(Tracked(&mut p), i0b);

        let m0 = c.borrow_mut(Tracked(&mut p));
        c.terminate_mut(Tracked(&mut p), m0);

        let m1 = c.borrow_mut(Tracked(&mut p));
        // let m2 = c.borrow_mut(Tracked(&mut p));  // is not accepted
        // let i2 = c.borrow(Tracked(&mut p));  // is not accepted
    }

    {
        // Testing owner check
        let (c1, Tracked(mut p1)) = new_disjoint::<usize>(0);
        let (c2, Tracked(mut p2)) = new_disjoint::<usize>(0);

        let i0a = c1.borrow(Tracked(&mut p1));

        let i1a = c2.borrow(Tracked(&mut p2));

        c1.terminate(Tracked(&mut p1), i1a);

        // let i0b = c1.borrow(Tracked(&mut p2));  // not accepted
        // let m0b = c1.borrow_mut(Tracked(&mut p2));  // not accepted
    }

}

}
