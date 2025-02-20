use vstd::prelude::*;

verus!{

// first i try without ids

use std::marker::PhantomData;
use vstd::simple_pptr::*;

pub type UID = PPtr<()>;

pub struct Disjoint<T> {
    // This should be some collection of n Ts (Vec<RefCell<T>>, UnsafeCell<Vec<T>>, etc)
    data: PhantomData<T>,
    id: UID,
    size: usize,
}

pub tracked struct PermSet {
    id: Tracked<PointsTo<()>>,
    immuts: Ghost<Map<usize, nat>>,
    muts: Ghost<Set<usize>>,
}

pub struct DisjointGuard<'a, T> {
    parent: &'a Disjoint<T>,
    i: usize,
    uid: Tracked<PointsTo<()>>,
}

pub struct DisjointMutGuard<'a, T> {
    parent: &'a Disjoint<T>,
    i: usize,
}

impl<'a, T> DisjointGuard<'a, T> {
    pub closed spec fn corresponds(&self, i: usize) -> bool {
        self.i == i
    }

    pub closed spec fn i(&self) -> usize {
        self.i
    }
}

impl<'a, T> DisjointMutGuard<'a, T> {
    pub closed spec fn corresponds(&self, i: usize) -> bool {
        self.i == i
    }

    pub closed spec fn i(&self) -> usize {
        self.i
    }
}

impl PermSet {
    pub closed spec fn id(self) -> PPtr<()> {
        self.id@.pptr()
    }

    pub closed spec fn axm_holds(self) -> bool {
        forall |i: usize| self.muts@.contains(i) ==> self.immuts@.index(i) == 0
    }
}

pub closed spec fn available_immut(perms: PermSet, i: usize) -> bool {
    !perms.muts@.contains(i)
}

pub closed spec fn available_mut(perms: PermSet, i: usize) -> bool {
    perms.immuts@.index(i) == 0 && available_immut(perms, i)
}

pub proof fn mut_then_immut(perms: PermSet, i: usize)
    requires available_mut(perms, i),
    ensures available_immut(perms, i)
{}

pub fn new_disjoint<T>(size: usize) -> (res: (Disjoint<T>, Tracked<PermSet>))
    ensures
        correspond(res.0, res.1@),
        forall |i: usize| i < size ==> available_immut(res.1@, i) && available_mut(res.1@, i),
        res.1@.axm_holds()
{
    let (pptr, Tracked(perm)) = PPtr::empty();
    let d = Disjoint {
        data: PhantomData,
        id: pptr,
        size,
    };
    let ghost immuts = Map::<usize, nat>::new(|x: usize| x < size, |x| 0);
    let p = Tracked(PermSet {
        id: Tracked(perm),
        immuts: Ghost(immuts),
        muts: Ghost(Set::empty()),
    });
    (d, p)
}

pub closed spec fn correspond<T>(d: Disjoint<T>, p: PermSet) -> bool {
    d.id == p.id()
}

impl<T> Disjoint<T> {
    pub closed spec fn corresponds(&self, guard: DisjointGuard<'_, T>) -> bool {
        self == &guard.parent
    }

    pub closed spec fn corresponds_mut(&self, guard: DisjointMutGuard<'_, T>) -> bool {
        self == &guard.parent
    }

    pub fn borrow(&self, Tracked(p): Tracked<&mut PermSet>, i: usize) -> (res: DisjointGuard<'_, T>)
        requires
            old(p).axm_holds(),
            available_immut(*old(p), i),
        ensures
            p.id() == old(p).id(),
            p.axm_holds(),
            forall |j: usize| available_immut(*old(p), j) <==> available_immut(*p, j),
            forall |j: usize| i != j ==> (available_mut(*old(p), j) <==> available_mut(*p, j)),
            forall |b: DisjointGuard<'_, T>| self.borrows(old(p), b) ==> self.borrows(p, b),
            !available_mut(*p, i),
            self.borrows(p, res),
    {
        let ghost prev_immuts = p.immuts@;
        let ghost prev_borrows = prev_immuts.index(i);
        let ghost removed_immuts = prev_immuts.remove(i);
        let ghost updated_immuts = removed_immuts.insert(i, (prev_borrows + 1) as nat);
        proof {
            p.immuts = Ghost(updated_immuts);
        }

        let (_, Tracked(uid)) = PPtr::empty();

        DisjointGuard {
            parent: self,
            i,
            uid: Tracked(uid),
        }
    }

    pub fn borrow_mut(&self, Tracked(p): Tracked<&mut PermSet>, i: usize) -> (res: DisjointMutGuard<'_, T>)
        requires
            old(p).axm_holds(),
            available_mut(*old(p), i),
        ensures
            p.id() == old(p).id(),
            p.axm_holds(),
            forall |j: usize| i != j ==> (available_immut(*old(p), j) <==> available_immut(*p, j)),
            forall |j: usize| i != j ==> (available_mut(*old(p), j) <==> available_mut(*p, j)),
            !available_mut(*p, i),
            !available_immut(*p, i),
            self.borrows_mut(p, res),
    {
        proof {
            p.muts = Ghost(p.muts@.insert(i));
        }
        DisjointMutGuard {
            parent: self,
            i,
        }
    }

    pub closed spec fn borrows<'a>(&'a self, p: &PermSet, borrow: DisjointGuard<'a, T>) -> bool {
        &&& self.corresponds(borrow)
        &&& p.immuts@.index(borrow.i()) > 0
    }

    pub closed spec fn borrows_mut<'a>(&'a self, p: &PermSet, borrow: DisjointMutGuard<'a, T>) -> bool {
        &&& self.corresponds_mut(borrow)
        &&& p.muts@.contains(borrow.i())
    }

    pub fn terminate_mut<'a>(&'a self, Tracked(p): Tracked<&mut PermSet>, borrow: DisjointMutGuard<'a, T>)
        requires
            old(p).axm_holds(),
            self.borrows_mut(old(p), borrow),
        ensures
            p.axm_holds(),
            available_mut(*p, borrow.i())
    {
        proof {
            p.muts = Ghost(p.muts@.remove(borrow.i()));
        }
    }

    pub fn terminate<'a>(&'a self, Tracked(p): Tracked<&mut PermSet>, borrow: DisjointGuard<'a, T>)
        requires
            old(p).axm_holds(),
            self.borrows(old(p), borrow),
        ensures
            p.axm_holds(),
            available_immut(*p, borrow.i()),
            forall |b: DisjointGuard<'_, T>| b != borrow && #[trigger] self.borrows(old(p), b) ==> #[trigger] self.borrows(p, b),
    {
        let ghost prev_immuts = p.immuts@;
        let ghost prev_borrows = prev_immuts.index(borrow.i());
        let ghost updated_immuts = prev_immuts.insert(borrow.i(), (prev_borrows - 1) as nat);
        proof {
            p.immuts = Ghost(updated_immuts);
        }
    }
}

fn main() {
    // test examples from ghostcell
    // 1) simple case with no owner id checking
    let (d, Tracked(mut p)) = new_disjoint::<usize>(3);

    let i0a = d.borrow(Tracked(&mut p), 0);
    let i0b = d.borrow(Tracked(&mut p), 0);

    assert(i0a.uid != i0b.uid);

    assert(d.borrows(&p, i0b));
    d.terminate(Tracked(&mut p), i0a);
    assert(d.borrows(&p, i0b));
    d.terminate(Tracked(&mut p), i0b);

    // let m0 = d.borrow_mut(Tracked(&mut p), 0);
}

}
