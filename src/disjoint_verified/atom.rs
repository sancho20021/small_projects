use vstd::prelude::*;

verus!{

mod atom {
//     use vstd::prelude::*;
//     use vstd::simple_pptr;
//     use vstd::cell::*;

//     pub type UidGhost = simple_pptr::PointsTo<()>;
//     pub type UID = simple_pptr::PPtr<()>;
//     pub type BorrowId = nat;

//     pub struct Cell<T> {
//         data: PCell<T>,
//     }

//     pub tracked struct PermSet<T> {
//         perm: Tracked<PointsTo<T>>
//     }

//     pub struct DisjointGuard<'a, T> {
//         parent: &'a Cell<T>,
//         perm: &'a PermSet<T>,
//     }

//     pub struct DisjointMutGuard<'a, T> {
//         perm_id: Tracked<UID>,
//         parent: &'a Cell<T>,
//     }

//     impl PermSet {
//         pub closed spec fn id(self) -> UID {
//             self.id@.pptr()
//         }

//         pub closed spec fn well_formed(self) -> bool {
//             &&& self.immuts@.finite()
//             &&& self.muts@ ==> self.immuts@.is_empty()
//             &&& forall |x: BorrowId| self.immuts@.contains(x) ==> x < self.next_unique@
//         }

//         pub closed spec fn available_immut(self) -> bool {
//             !self.muts@
//         }

//         pub closed spec fn available_mut(self) -> bool {
//             self.immuts@.is_empty() && self.available_immut()
//         }

//         pub proof fn mut_then_immut(&self)
//             requires self.available_mut(),
//             ensures self.available_immut(),
//         {}

//         pub closed spec fn no_borrows(self) -> bool {
//             &&& self.immuts@.is_empty()
//             &&& !self.muts@
//         }

//         pub proof fn no_borrows_then_available(&self)
//             requires self.no_borrows()
//             ensures self.available_mut()
//         {}

//         pub closed spec fn nborrows(self) -> (nat, bool) {
//             (self.immuts@.len(), self.muts@)
//         }

//         pub proof fn no_borrow_implies_borrows_zero(&self)
//             requires self.no_borrows()
//             ensures self.nborrows() == (0nat, false)
//         {
//         }

//         pub proof fn borrows_zero_then_no_borrows(&self)
//             requires self.nborrows() == (0nat, false),
//             ensures self.no_borrows()
//         {
//         }

//         pub proof fn available_mut_then_borrows_zero(&self)
//             requires self.available_mut()
//             ensures self.nborrows() == (0nat, false)
//         {}

//         pub proof fn available_then_mut_false(&self)
//             requires
//                 self.available_immut(),
//                 self.well_formed(),
//             ensures self.nborrows().1 == false
//         {}
//     }

//     pub fn new_cell<T>(data: T) -> (res: (Cell<T>, Tracked<PermSet>))
//         ensures
//             correspond(res.0, res.1@),
//             res.1@.available_immut() && res.1@.available_mut(),
//             res.1@.well_formed(),
//             res.1@.no_borrows(),
//             res.1@.nborrows() == (0nat, false),
//             data == res.0@,
//     {
//         let (pptr, Tracked(perm)) = PPtr::<()>::empty();
//         let d = Cell {
//             data,
//             id: Tracked(pptr),
//         };
//         let p = Tracked(PermSet {
//             id: Tracked(perm),
//             immuts: Ghost(Set::empty()),
//             muts: Ghost(false),
//             next_unique: Ghost(0),
//         });
//         (d, p)
//     }

//     pub closed spec fn correspond<T>(d: Cell<T>, p: PermSet) -> bool {
//         d.id@ == p.id()
//     }

//     proof fn insert_remove_id<T>(s: Set<T>, x: T)
//         requires
//             s.finite(),
//             !s.contains(x),
//         ensures
//             s.insert(x).remove(x) == s,
//         decreases
//             s.len(),
//     {
//         if s.len() == 0 {
//             assert(s.insert(x).remove(x) == s);
//         } else {
//             let a = s.choose();
//             if a != x {
//                 let ss = s.remove(a);
//                 insert_remove_id(ss, x);
//                 assert(ss.insert(x).remove(x) == ss);
//                 assert(ss.insert(a).insert(x).remove(x) == ss.insert(a));
//             }
//         }
//     }

//     impl<T> Cell<T> {
//         closed spec fn corresponds(&self, guard: DisjointGuard<'_, T>) -> bool {
//             self == &guard.parent && self.id == guard.perm_id
//         }

//         closed spec fn corresponds_mut(&self, guard: DisjointMutGuard<'_, T>) -> bool {
//             // This is interesting, if I remove the perm_id check, the verification fails.
//             // Curious to find counter-example
//             self == &guard.parent && self.id == guard.perm_id
//         }

//         pub fn borrow(&self, Tracked(p): Tracked<&mut PermSet>,) -> (res: DisjointGuard<'_, T>)
//             requires
//                 old(p).well_formed(),
//                 old(p).available_immut(),
//                 correspond(*self, *old(p)),
//             ensures
//                 p.id() == old(p).id(),
//                 p.well_formed(),
//                 p.available_immut(),
//                 forall |b: DisjointGuard<'_, T>| self.borrows(old(p), b) ==> self.borrows(p, b),
//                 borrow_created(*old(p), *p, res),
//                 self.borrows(p, res),
//                 correspond(*self, *p),
//                 forall |b: DisjointGuard<'_, T>| self.borrows(old(p), b) ==> b != res,
//                 p.nborrows() == (old(p).nborrows().0 + 1, false)
//         {
//             let Ghost(mut uid) = Ghost(0);

//             proof {
//                 let Ghost(x) = p.next_unique;
//                 uid = x;

//                 assert(forall |i: BorrowId| p.immuts@.contains(i) ==> i < uid);
//                 assert(!p.immuts@.contains(uid));
//                 assert(p.immuts@.insert(uid).remove(uid) == p.immuts@) by {
//                     insert_remove_id(p.immuts@, uid);
//                 };

//                 p.immuts = Ghost(p.immuts@.insert(uid));
//                 p.next_unique = Ghost(uid + 1);
//             }

//             DisjointGuard {
//                 parent: self,
//                 uid: Ghost(uid),
//                 perm_id: self.id,
//             }
//         }

//         pub fn borrow_mut(&self, Tracked(p): Tracked<&mut PermSet>) -> (res: DisjointMutGuard<'_, T>)
//             requires
//                 old(p).well_formed(),
//                 old(p).available_mut(),
//                 correspond(*self, *old(p)),
//             ensures
//                 p.id() == old(p).id(),
//                 p.well_formed(),
//                 !p.available_mut(),
//                 !p.available_immut(),
//                 self.borrows_mut(p, res),
//                 correspond(*self, *p),
//                 p.nborrows() == (0nat, true),
//         {
//             proof {
//                 p.muts = Ghost(true);
//             }
//             DisjointMutGuard {
//                 perm_id: self.id,
//                 parent: self,
//             }
//         }

//         pub closed spec fn borrows<'a>(&'a self, p: &PermSet, borrow: DisjointGuard<'a, T>) -> bool {
//             // maybe to add correspond(a, p) here because otherwise the unique ptrs won't be checked
//             // todo: threat to soundness because borrow doesn't contain unique ptr
//             &&& correspond(*self, *p)
//             &&& self.corresponds(borrow)
//             &&& p.immuts@.contains(borrow.uid@)
//         }

//         pub closed spec fn borrows_mut<'a>(&'a self, p: &PermSet, borrow: DisjointMutGuard<'a, T>) -> bool {
//             &&& correspond(*self, *p)
//             &&& self.corresponds_mut(borrow)
//             &&& p.muts@
//         }

//         pub fn terminate_mut<'a>(&'a self, Tracked(p): Tracked<&mut PermSet>, borrow: DisjointMutGuard<'a, T>)
//             requires
//                 correspond(*self, *old(p)),
//                 old(p).well_formed(),
//                 self.borrows_mut(old(p), borrow),
//             ensures
//                 correspond(*self, *p),
//                 p.well_formed(),
//                 p.available_mut(),
//         {
//             proof {
//                 p.muts = Ghost(false);
//             }
//         }

//         pub fn terminate<'a>(&'a self, Tracked(p): Tracked<&mut PermSet>, borrow: DisjointGuard<'a, T>)
//             requires
//                 correspond(*self, *old(p)),
//                 old(p).well_formed(),
//                 self.borrows(old(p), borrow),
//             ensures
//                 correspond(*self, *p),
//                 p.well_formed(),
//                 p.available_immut(),
//                 borrow_terminated(*old(p), *p, borrow),
//                 forall |b: DisjointGuard<'_, T>| b != borrow && self.borrows(old(p), b) ==> self.borrows(p, b),
//                 p.nborrows() == ((old(p).nborrows().0 - 1) as nat, false)
//         {
//             proof {
//                 p.immuts = Ghost(p.immuts@.remove(borrow.uid@));
//             }
//         }

//         pub proof fn borrows_mut_then_unavailable(&self, p: &PermSet, b: &DisjointMutGuard<'_, T>)
//             requires self.borrows_mut(p, *b),
//             ensures !p.available_immut(),
//         {}

//         pub proof fn borrows_then_unavailable_mut(&self, p: &PermSet, b: &DisjointGuard<'_, T>)
//             requires
//                 self.borrows(p, *b),
//                 p.well_formed(),
//             ensures !p.available_mut(),
//         {}
//     }

//     impl<T> vstd::view::View for Cell<T> {
//         type V = T;

//         closed spec fn view(&self) -> Self::V {
//             self.data
//         }
//     }

//     pub closed spec fn borrow_terminated<T>(old: PermSet, new: PermSet, borrow: DisjointGuard<'_, T>) -> bool {
//         &&& old.immuts@.remove(borrow.uid@) == new.immuts@
//         &&& old.immuts@ == new.immuts@.insert(borrow.uid@)
//     }

//     pub closed spec fn borrow_created<T>(old: PermSet, new: PermSet, borrow: DisjointGuard<'_, T>) -> bool {
//         &&& old.immuts@.insert(borrow.uid@) == new.immuts@
//         &&& old.immuts@ == new.immuts@.remove(borrow.uid@)
//     }

//     pub closed spec fn borrow_untouched<T>(old: PermSet, new: PermSet) -> bool {
//         old.immuts@ == new.immuts@
//     }

//     pub proof fn borrow_equal_then_untouched<T>(old: PermSet, new: PermSet)
//         requires old == new
//         ensures borrow_untouched::<T>(old, new)
//     {}
    use std::rc::Rc;
    use vstd::prelude::*;

    fn f() {
        let x: Tracked<Rc<i32>> = Tracked(Rc::new(0));
        let ghost c = x@;
    }

}

fn main() {

}
}
