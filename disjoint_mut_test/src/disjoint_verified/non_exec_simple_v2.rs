use vstd::prelude::*;

verus!{

mod vec_extra {
    use vstd::prelude::*;

    pub fn rev<T>(mut v: Vec<T>) -> (res: Vec<T>)
        ensures
            res@ == v@.reverse()
    {
        let ghost v_snap = v@;

        let mut res: Vec<T> = vec![];
        assert(res@ == Seq::<T>::empty());
        assert(v_snap.reverse().take(0) == Seq::<T>::empty());
        assert(v@ == v_snap.take(v.len() as int));
        while v.len() > 0
            invariant
                v@ == v_snap.take(v.len() as int),
                res@ == v_snap.reverse().take(res.len() as int),
                v.len() <= v_snap.len(),
                v_snap.len() - res.len() == v.len(),
        {
            let x = v.pop().unwrap();
            assert(v@ == v_snap.take(v.len() as int));
            res.push(x);
            assert(res@ == v_snap.reverse().take(res.len() as int));
        }
        assert(v_snap.reverse().take(res.len() as int) == v_snap.reverse());
        res
    }
}

mod atom {
    use vstd::prelude::*;
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
        perm_id: Tracked<UID>,
        uid: Ghost<BorrowId>,
    }

    pub struct DisjointMutGuard<'a, T> {
        perm_id: Tracked<UID>,
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

        pub proof fn no_borrows_then_available(&self)
            requires self.no_borrows()
            ensures self.available_mut()
        {}

        pub closed spec fn nborrows(self) -> (nat, bool) {
            (self.immuts@.len(), self.muts@)
        }

        pub proof fn no_borrow_implies_borrows_zero(&self)
            requires self.no_borrows()
            ensures self.nborrows() == (0nat, false)
        {
        }

        pub proof fn borrows_zero_then_no_borrows(&self)
            requires self.nborrows() == (0nat, false),
            ensures self.no_borrows()
        {
        }

        pub proof fn available_mut_then_borrows_zero(&self)
            requires self.available_mut()
            ensures self.nborrows() == (0nat, false)
        {}

        pub proof fn available_then_mut_false(&self)
            requires
                self.available_immut(),
                self.well_formed(),
            ensures self.nborrows().1 == false
        {}
    }

    pub fn new_cell<T>(data: T) -> (res: (Cell<T>, Tracked<PermSet>))
        ensures
            correspond(res.0, res.1@),
            res.1@.available_immut() && res.1@.available_mut(),
            res.1@.well_formed(),
            res.1@.no_borrows(),
            res.1@.nborrows() == (0nat, false),
            data == res.0@,
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
            self == &guard.parent && self.id == guard.perm_id
        }

        closed spec fn corresponds_mut(&self, guard: DisjointMutGuard<'_, T>) -> bool {
            // This is interesting, if I remove the perm_id check, the verification fails.
            // Curious to find counter-example
            self == &guard.parent && self.id == guard.perm_id
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
                correspond(*self, *p),
                forall |b: DisjointGuard<'_, T>| self.borrows(old(p), b) ==> b != res,
                p.nborrows() == (old(p).nborrows().0 + 1, false)
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
                perm_id: self.id,
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
                correspond(*self, *p),
                p.nborrows() == (0nat, true),
        {
            proof {
                p.muts = Ghost(true);
            }
            DisjointMutGuard {
                perm_id: self.id,
                parent: self,
            }
        }

        pub closed spec fn borrows<'a>(&'a self, p: &PermSet, borrow: DisjointGuard<'a, T>) -> bool {
            // maybe to add correspond(a, p) here because otherwise the unique ptrs won't be checked
            // todo: threat to soundness because borrow doesn't contain unique ptr
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
                forall |b: DisjointGuard<'_, T>| b != borrow && self.borrows(old(p), b) ==> self.borrows(p, b),
                p.nborrows() == ((old(p).nborrows().0 - 1) as nat, false)
        {
            proof {
                p.immuts = Ghost(p.immuts@.remove(borrow.uid@));
            }
        }

        pub proof fn borrows_mut_then_unavailable(&self, p: &PermSet, b: &DisjointMutGuard<'_, T>)
            requires self.borrows_mut(p, *b),
            ensures !p.available_immut(),
        {}

        pub proof fn borrows_then_unavailable_mut(&self, p: &PermSet, b: &DisjointGuard<'_, T>)
            requires
                self.borrows(p, *b),
                p.well_formed(),
            ensures !p.available_mut(),
        {}
    }

    impl<T> vstd::view::View for Cell<T> {
        type V = T;

        closed spec fn view(&self) -> Self::V {
            self.data
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

    pub closed spec fn borrow_untouched<T>(old: PermSet, new: PermSet) -> bool {
        old.immuts@ == new.immuts@
    }

    pub proof fn borrow_equal_then_untouched<T>(old: PermSet, new: PermSet)
        requires old == new
        ensures borrow_untouched::<T>(old, new)
    {}

}

mod array {
    use super::atom::*;
    use super::atom;
    use vstd::prelude::*;
    use super::vec_extra;

    pub struct Array<T> {
        cells: Vec<Cell<T>>,
        size: usize,
    }

    pub struct ArrayBorrow<'a, T> {
        borrow: DisjointGuard<'a, T>,
        i: usize,
    }

    pub struct ArrayMutBorrow<'a, T> {
        borrow: DisjointMutGuard<'a, T>,
        i: usize,
    }

    impl<'a, T> ArrayBorrow<'a, T> {
        pub closed spec fn i(&self) -> usize {
            self.i
        }
    }

    impl<'a, T> ArrayMutBorrow<'a, T> {
        pub closed spec fn i(&self) -> usize {
            self.i
        }
    }

    pub tracked struct APermSet(Map<usize, PermSet>);

    pub proof fn test_size<T>(a: Array<T>, p: APermSet)
        requires well_formed(a, p),
        ensures a@.len() == a.size(),
    {
    }

    pub closed spec fn well_formed<T>(a: Array<T>, p: APermSet) -> bool {
        &&& correspond(a, p)
        &&& a@.len() == a.size()
        &&& forall |i: usize| i < a.size() ==> p.0.contains_key(i)
        &&& forall |i: usize| i < a.size() ==> #[trigger] p.0.index(i).well_formed()
    }

    closed spec fn correspond<T>(a: Array<T>, p: APermSet) -> bool {
        forall |i: usize| i < a.size ==> #[trigger] atom::correspond(a.cells[i as int], p.0.index(i))
    }

    pub closed spec fn borrow_untouched<T>(old: APermSet, new: APermSet, i: usize) -> bool {
        atom::borrow_untouched::<T>(old.0.index(i), new.0.index(i))
    }

    impl APermSet {
        pub closed spec fn available_immut(&self, i: usize) -> bool {
            self.0.index(i).available_immut()
        }

        pub closed spec fn available_mut(&self, i: usize) -> bool {
            self.0.index(i).available_mut()
        }

        pub closed spec fn no_borrows(&self, i: usize) -> bool {
            self.0.index(i).no_borrows() && self.available_immut(i) && self.available_mut(i)
        }

        pub proof fn no_borrow_implies_borrows_zero(&self, i: usize)
            requires self.no_borrows(i)
            ensures self.nborrows(i) == (0nat, false)
        {
            assert(self.0.index(i).no_borrows());
            self.0.index(i).no_borrow_implies_borrows_zero();
        }

        pub proof fn borrows_zero_then_no_borrows(&self, i: usize)
            requires self.nborrows(i) == (0nat, false),
            ensures self.no_borrows(i)
        {
            self.0.index(i).borrows_zero_then_no_borrows();
            self.0.index(i).no_borrows_then_available();
            self.0.index(i).mut_then_immut();
        }

        pub proof fn no_borrows_then_available(&self, i: usize)
            requires self.no_borrows(i)
            ensures self.available_immut(i) && self.available_mut(i)
        {}

        proof fn available_propagate<T>(&self, i: usize)
            requires Array::<T>::available_both(self.0.index(i)),
            ensures self.available_mut(i) && self.available_immut(i),
        {}

        /// experimental, not sure this should be exposed
        pub closed spec fn nborrows(&self, i: usize) -> (nat, bool) {
            self.0.index(i).nborrows()
        }

        pub broadcast proof fn borrows_zero_then_available(&self, i: usize)
            requires #[trigger] self.nborrows(i) == (0nat, false)
            ensures self.no_borrows(i) && self.available_immut(i) && self.available_mut(i)
        {
            self.0.index(i).borrows_zero_then_no_borrows();
            self.0.index(i).no_borrows_then_available();
            self.0.index(i).mut_then_immut();
        }

        pub proof fn available_mut_then_borrows_zero(&self, i: usize)
            requires self.available_mut(i)
            ensures self.nborrows(i) == (0nat, false)
        {
            self.0.index(i).available_mut_then_borrows_zero();
        }
    }

    pub closed spec fn borrow_created<T>(old: APermSet, new: APermSet, borrow: ArrayBorrow<'_, T>) -> bool {
        let index = borrow.i;
        let old = old.0.index(index);
        let new = new.0.index(index);
        atom::borrow_created(old, new, borrow.borrow)
    }

    pub closed spec fn borrow_terminated<T>(old: APermSet, new: APermSet, borrow: ArrayBorrow<'_, T>) -> bool {
        let index = borrow.i;
        let old = old.0.index(index);
        let new = new.0.index(index);
        atom::borrow_terminated(old, new, borrow.borrow)
    }

    impl<T> Array<T> {
        pub open spec fn no_borrows(&self, p: APermSet) -> bool {
            forall |i: usize| i < self.size() ==> #[trigger] p.no_borrows(i)
        }

        pub closed spec fn size(&self) -> usize {
            self.size
        }

        pub proof fn no_borrows_then_available(&self, p: &APermSet, i: usize)
            requires
                i < self.size(),
                well_formed(*self, *p),
                p.no_borrows(i),
            ensures
                p.available_mut(i),
                p.available_immut(i),
        {
            p.0.index(i).no_borrows_then_available();
            self.available_mut_then_immut(p, i);
        }

        pub proof fn available_mut_then_immut(&self, p: &APermSet, i: usize)
            requires
                i < self.size(),
                well_formed(*self, *p),
                p.available_mut(i)
            ensures
                p.available_immut(i)
        {
            assert(p.0.index(i).available_mut());
            p.0.index(i).mut_then_immut();
        }

        pub closed spec fn borrows(&self, p: APermSet, borrow: ArrayBorrow<'_, T>) -> bool {
            // Hmm, looks like we need index, and we need to store it in borrow. todo: Maybe just ghost, maybe not
            let index = borrow.i;
            let cell = self.cells[index as int];
            let perm = p.0.index(index);
            borrow.i < self.size() && cell.borrows(&perm, borrow.borrow)
        }

        pub closed spec fn borrows_mut(&self, p: APermSet, borrow: ArrayMutBorrow<'_, T>) -> bool {
            let index = borrow.i;
            let cell = self.cells[index as int];
            let perm = p.0.index(index);
            borrow.i < self.size() && cell.borrows_mut(&perm, borrow.borrow)
        }

        spec fn available(perms: Map<usize, PermSet>, i: usize) -> bool {
            perms.index(i).available_immut() && perms.index(i).available_mut()
        }

        spec fn available_both(p: PermSet) -> bool {
            p.available_immut() && p.available_mut()
        }

        proof fn perms_unrelated_insert(old: Map<usize, PermSet>, new: Map<usize, PermSet>, n: usize, i: usize, p: PermSet)
            requires
                forall |j: usize| j < n ==> old.contains_key(j),
                forall |j: usize| j < n ==> #[trigger] Self::available_both(old.index(j)),
                !old.contains_key(i),
                new == old.insert(i, p),
            ensures
                forall |j: usize| j < n ==> #[trigger] Self::available_both(new.index(j)),
        {}

        fn new_rev(mut elements: Vec<T>) -> (res: (Vec<Cell<T>>, Tracked<Map<usize, PermSet>>))
            ensures
                forall |i: usize| i < res.0.len() ==> res.1@.contains_key(i),
                forall |i: usize| i < res.0.len() ==> #[trigger] res.1@.index(i).well_formed(),
                forall |i: usize| i < res.0.len() ==> #[trigger] atom::correspond(res.0[i as int], res.1@.index(i)),
                forall |i: usize| i < res.0.len() ==> #[trigger] Self::available_both(res.1@.index(i)),
                forall |i: usize| i < res.0.len() ==> #[trigger] res.1@.index(i).no_borrows(),
                elements@.reverse() == res.0@.map_values(|x: Cell<T>| x@),
        {
            let mut cells: Vec<Cell<T>> = Vec::new();
            let tracked mut perms: Map<usize, PermSet> = Map::tracked_empty();

            assert(elements@.take(0) == cells@.map_values(|x: Cell<T>| x@));

            let ghost elements_snap = elements@;

            assert(elements_snap.reverse().take(cells.len() as int) == Seq::<T>::empty());
            assert(cells@.map_values(|x: Cell<T>| x@) == Seq::<T>::empty());
            assert(elements@ == elements_snap.take(elements.len() as int));

            while elements.len() > 0
                invariant
                    forall |j: usize| j < cells.len() ==> perms.contains_key(j),
                    forall |j: usize| j < cells.len() ==> #[trigger] perms.index(j).well_formed(),
                    forall |j: usize| j < cells.len() ==> #[trigger] atom::correspond(cells[j as int], perms.index(j)),
                    elements_snap.reverse().take(cells.len() as int) == cells@.map_values(|x: Cell<T>| x@),
                    elements@ == elements_snap.take(elements.len() as int),
                    elements_snap.len() - cells.len() == elements.len(),
                    cells.len() + elements.len() == elements_snap.reverse().len(),
                    forall |j: usize| j < cells.len() ==> #[trigger] Self::available_both(perms.index(j)),
                    forall |i: usize| i < cells.len() ==> #[trigger] perms.index(i).no_borrows(),
            {
                assert(elements_snap.reverse().len() - elements.len() == cells.len());

                let x = elements.pop().unwrap();
                assert(elements@ == elements_snap.take(elements.len() as int));

                assert(x == elements_snap.reverse()[cells.len() as int]);


                let (c, Tracked(p)) = new_cell(x);
                assert(p.available_immut() && p.available_mut());
                assert(c@ == x);
                assert(c@ == elements_snap.reverse()[cells.len() as int]);


                cells.push(c);

                assert(c@ == elements_snap.reverse()[cells.len() - 1 as int]);
                assert(cells@[cells.len() - 1]@ == elements_snap.reverse()[cells.len() - 1 as int]);

                let ghost mcells = cells@.map_values(|x: Cell<T>| x@);
                let ghost esrev = elements_snap.reverse();
                let ghost n = cells.len() - 1;

                assert(mcells.take(n) == esrev.take(n));
                assert(mcells.take(n).push(mcells[n]) == mcells);
                assert(mcells == esrev.take(n+1));


                proof {
                    perms.tracked_insert((cells.len() - 1) as usize, p);
                    assert(Self::available(perms, (cells.len() - 1) as usize));
                }
            }
            assert(elements_snap.reverse().take(cells.len() as int) == elements_snap.reverse());

            (cells, Tracked(perms))
        }

        pub fn new(elements: Vec<T>) -> (res: (Array<T>, Tracked<APermSet>))
            ensures
                well_formed(res.0, res.1@),
                res.0.no_borrows(res.1@),
                forall |i: usize| i < res.0.size() ==> res.1@.available_immut(i) && res.1@.available_mut(i),
                forall |i: usize| i < res.0.size() ==> res.1@.nborrows(i) == (0nat, false),
                elements@ == res.0@,
                res.0.size() == elements@.len(),
        {
            let ghost elements_snap = elements@;

            let size = elements.len();

            let elements_rev = vec_extra::rev(elements);
            assert(elements_rev@.reverse() == elements_snap);

            let (cells, Tracked(perms)) = Self::new_rev(elements_rev);

            let array = Array {
                cells,
                size,
            };
            let tracked perm = APermSet(perms);

            assert(forall |j: usize| j < cells.len() ==> #[trigger] Self::available_both(perm.0.index(j)));
            assert forall |j: usize| j < cells.len() implies perm.available_immut(j) && perm.available_mut(j) by {
                perm.available_propagate::<T>(j);
            }
            assert forall |i: usize| i < cells.len() implies perm.nborrows(i) == (0nat, false) by {
                assert(perm.no_borrows(i));
                perm.no_borrow_implies_borrows_zero(i);
            }
            assert(array.no_borrows(perm));

            (array, Tracked(perm))
        }

        pub fn borrow(&self, Tracked(p): Tracked<&mut APermSet>, i: usize) -> (res: ArrayBorrow<'_, T>)
            requires
                well_formed(*self, *old(p)),
                old(p).available_immut(i),
                i < self.size(),
            ensures
                well_formed(*self, *p),
                forall |j: usize| j != i && j < self.size() ==> borrow_untouched::<T>(*old(p), *p, j),
                forall |j: usize| i != j && j < self.size() ==> (old(p).available_mut(j) ==> p.available_mut(j)),
                forall |j: usize| i != j && j < self.size() ==> (old(p).available_immut(j) ==> p.available_immut(j)),
                forall |j: usize| i != j && j < self.size() ==> (old(p).nborrows(j) == p.nborrows(j)),
                forall |b: ArrayBorrow<'_, T>| self.borrows(*old(p), b) ==> self.borrows(*p, b),
                forall |b: ArrayBorrow<'_, T>| self.borrows(*old(p), b) ==> b != res,
                forall |b: ArrayMutBorrow<'_, T>| self.borrows_mut(*old(p), b) ==> self.borrows_mut(*p, b),
                self.borrows(*p, res),
                res.i() == i,
                borrow_created(*old(p), *p, res),
                p.available_immut(i),
                p.nborrows(i) == (old(p).nborrows(i).0 + 1, false)
        {
            let tracked permission = p.0.tracked_remove(i);

            let ghost old_permission = permission;

            let borrow = self.cells[i].borrow(Tracked(&mut permission));
            proof {
                p.0.tracked_insert(i, permission);
            }
            proof {
                self.one_borrow_changed(*old(p), *p, i);
            }
            assert forall |b: ArrayMutBorrow<'_, T> | self.borrows_mut(*old(p), b) implies self.borrows_mut(*p, b) by {
                if b.i() == i {
                    assert(!old_permission.available_immut()) by {
                        assert(self.cells[i as int].borrows_mut(&old_permission, b.borrow));
                        self.cells[i as int].borrows_mut_then_unavailable(&old_permission, &b.borrow);
                    }
                }
            }
            ArrayBorrow {
                i,
                borrow,
            }
        }

        pub fn borrow_mut(&self, Tracked(p): Tracked<&mut APermSet>, i: usize) -> (res: ArrayMutBorrow<'_, T>)
            requires
                well_formed(*self, *old(p)),
                old(p).available_mut(i),
                i < self.size(),
            ensures
                well_formed(*self, *p),
                forall |j: usize| i != j && j < self.size() ==> (old(p).available_immut(j) ==> p.available_immut(j)),
                forall |j: usize| i != j && j < self.size() ==> (old(p).available_mut(j) ==> p.available_mut(j)),
                forall |j: usize| i != j && j < self.size() ==> (old(p).nborrows(j) == p.nborrows(j)),
                forall |j: usize| j != i && j < self.size() ==> borrow_untouched::<T>(*old(p), *p, j),
                forall |b: ArrayBorrow<'_, T>| self.borrows(*old(p), b) ==> self.borrows(*p, b),
                forall |b: ArrayMutBorrow<'_, T>| self.borrows_mut(*old(p), b) ==> self.borrows_mut(*p, b),
                p.nborrows(i) == (0nat, true),
                self.borrows_mut(*p, res),
                res.i() == i,
        {
            assert(p.nborrows(i) == (0nat, false)) by {
                p.available_mut_then_borrows_zero(i);
            }
            let tracked permission = p.0.tracked_remove(i);
            let ghost old_permission = permission;

            assert(permission.nborrows() == (0nat, false));
            let borrow = self.cells[i].borrow_mut(Tracked(&mut permission));
            assert(permission.nborrows() == (0nat, true));
            proof {
                p.0.tracked_insert(i, permission);
            }
            proof {
                self.one_borrow_changed(*old(p), *p, i);
            }
            assert forall |b: ArrayBorrow<'_, T> | self.borrows(*old(p), b) implies self.borrows(*p, b) by {
                if b.i() == i {
                    assert(!old_permission.available_mut()) by {
                        self.cells[i as int].borrows_then_unavailable_mut(&old_permission, &b.borrow);
                    }
                }
            }
            assert forall |b: ArrayMutBorrow<'_, T> | self.borrows_mut(*old(p), b) implies self.borrows_mut(*p, b) by {
                if b.i() == i {
                    assert(!old_permission.available_immut()) by {
                        self.cells[i as int].borrows_mut_then_unavailable(&old_permission, &b.borrow);
                    }
                    assert(old_permission.available_mut());
                    old_permission.mut_then_immut();
                    assert(old_permission.available_immut());
                }
            }

            ArrayMutBorrow {
                i,
                borrow,
            }
        }

        pub fn terminate(&self, Tracked(p): Tracked<&mut APermSet>, borrow: ArrayBorrow<'_, T>)
            requires
                well_formed(*self, *old(p)),
                self.borrows(*old(p), borrow),
            ensures
                well_formed(*self, *p),
                p.available_immut(borrow.i()),
                forall |b: ArrayBorrow<'_, T>| b != borrow && self.borrows(*old(p), b) ==> self.borrows(*p, b),
                forall |b: ArrayMutBorrow<'_, T>| b.i() != borrow.i() && #[trigger] self.borrows_mut(*old(p), b) ==> self.borrows_mut(*p, b),
                forall |j: usize| borrow.i() != j && j < self.size() ==> (old(p).available_mut(j) ==> p.available_mut(j)),
                forall |j: usize| borrow.i() != j && j < self.size() ==> (old(p).available_immut(j) ==> p.available_immut(j)),
                forall |j: usize| borrow.i() != j && j < self.size() ==> (old(p).nborrows(j) == p.nborrows(j)),
                borrow_terminated(*old(p), *p, borrow),
                p.nborrows(borrow.i()) == ((old(p).nborrows(borrow.i()).0 - 1) as nat, false)
        {
            let index = borrow.i;
            let tracked permission = p.0.tracked_remove(index);
            self.cells[index].terminate(Tracked(&mut permission), borrow.borrow);
            proof {
                p.0.tracked_insert(index, permission);
            }
        }

        pub fn terminate_mut(&self, Tracked(p): Tracked<&mut APermSet>, borrow: ArrayMutBorrow<'_, T>)
            requires
                well_formed(*self, *old(p)),
                self.borrows_mut(*old(p), borrow),
            ensures
                well_formed(*self, *p),
                forall |j: usize| j < self.size() && j != borrow.i() ==> borrow_untouched::<T>(*old(p), *p, j),
                forall |j: usize| borrow.i() != j && j < self.size() ==> (old(p).available_mut(j) ==> p.available_mut(j)),
                forall |j: usize| borrow.i() != j && j < self.size() ==> (old(p).available_immut(j) ==> p.available_immut(j)),
                p.available_mut(borrow.i()),
                p.nborrows(borrow.i()) == (0nat, false),
                forall |b: ArrayMutBorrow<'_, T>| b.i() != borrow.i() && #[trigger] self.borrows_mut(*old(p), b) ==> self.borrows_mut(*p, b),
        {
            let index = borrow.i;
            let tracked permission = p.0.tracked_remove(index);
            let ghost old_permission = permission;
            self.cells[index].terminate_mut(Tracked(&mut permission), borrow.borrow);
            proof {
                p.0.tracked_insert(index, permission);
                self.one_borrow_changed(*old(p), *p, borrow.i());
                p.available_mut_then_borrows_zero(borrow.i());
            }
        }

        proof fn one_borrow_changed_prefix(&self, old: APermSet, new: APermSet, i: usize, n: usize)
            requires
                well_formed(*self, old),
                well_formed(*self, new),
                n <= self.size(),
                forall |j: usize| j != i && j < self.size() ==> old.0.index(j) == new.0.index(j),
            ensures
                forall |j: usize| j != i && j < n ==> atom::borrow_untouched::<T>(#[trigger] old.0.index(j), new.0.index(j)),
            decreases n,
        {
            if n != 0 {
                self.one_borrow_changed_prefix(old, new, i, (n - 1) as usize);
                if (n - 1) as usize != i {
                    assert(old.0.index((n - 1) as usize) == new.0.index((n - 1) as usize));
                    assert(old.0.index((n - 1) as usize) == new.0.index((n - 1) as usize));
                    atom::borrow_equal_then_untouched::<T>(old.0.index((n - 1) as usize), new.0.index((n - 1) as usize));
                    assert(atom::borrow_untouched::<T>(old.0.index((n - 1) as usize), new.0.index((n - 1) as usize)));
                }
            }
        }

        proof fn one_borrow_changed(&self, old: APermSet, new: APermSet, i: usize)
            requires
                well_formed(*self, old),
                well_formed(*self, new),
                forall |j: usize| j != i && j < self.size() ==> old.0.index(j) == new.0.index(j),
            ensures
                forall |j: usize| j != i && j < self.size() ==> borrow_untouched::<T>(old, new, j),
        {
            self.one_borrow_changed_prefix(old, new, i, self.size());
        }
    }

    impl<T> vstd::view::View for Array<T> {
        type V = Seq<T>;

        closed spec fn view(&self) -> Seq<T> {
            self.cells@.map_values(|x: Cell<T>| x@)
        }
    }
}

use array::*;

fn main() {
    broadcast use array::APermSet::borrows_zero_then_available;

    let elements = vec![0, 1, 2];
    let (d, Tracked(mut p)) = Array::<usize>::new(elements);


    let i0a = d.borrow(Tracked(&mut p), 0);
    let m2a = d.borrow_mut(Tracked(&mut p), 2);
    let i0b = d.borrow(Tracked(&mut p), 1);


    d.terminate(Tracked(&mut p), i0a);
    d.terminate(Tracked(&mut p), i0b);

    d.terminate_mut(Tracked(&mut p), m2a);


    let m0a = d.borrow_mut(Tracked(&mut p), 0);
    let m1a = d.borrow_mut(Tracked(&mut p), 1);
    let m2a = d.borrow_mut(Tracked(&mut p), 2);

    d.terminate_mut(Tracked(&mut p), m0a);
    d.terminate_mut(Tracked(&mut p), m1a);
    d.terminate_mut(Tracked(&mut p), m2a);
}
}
