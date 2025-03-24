use vstd::prelude::*;

verus!{

use vstd::simple_pptr::*;

struct Array<T> {
    ptrs: Vec<PPtr<T>>,
    perms: Tracked<Map<usize, PointsTo<T>>>,
}

struct Borrow<T> {
    index: usize,
    ptr: PPtr<T>,
    perm: Tracked<PointsTo<T>>,
}

impl<T> Borrow<T> {
    pub closed spec fn wf(self) -> bool {
        self.ptr == self.perm@.pptr() && self.perm@.is_init()
    }

    pub closed spec fn value(self) -> T {
        self.perm@.value()
    }

    pub closed spec fn index(self) -> usize {
        self.index
    }

    pub closed spec fn ptr(&self) -> PPtr<T> {
        self.ptr
    }

    pub fn read(&self) -> (res: &T)
        requires self.wf()
        ensures *res == self.value()
    {
        self.ptr.borrow(Tracked(self.perm.borrow()))
    }

    pub fn replace(&mut self, x: T) -> (res: T)
        requires old(self).wf()
        ensures
            self.wf(),
            self.ptr() == old(self).ptr(),
            self.index() == old(self).index(),
            self.value() == x,
            res == old(self).value(),

    {
        self.ptr.replace(Tracked(self.perm.borrow_mut()), x)
    }
}

impl<T> Array<T> {
    pub closed spec fn len(&self) -> usize {
        self.ptrs.len()
    }

    pub fn length(&self) -> usize {
        self.ptrs.len()
    }

    pub closed spec fn wf(self) -> bool {
        forall |i: usize| i < self.len()
            ==> #[trigger] self.perms@.contains_key(i)
            ==> self.ptrs[i as int] == self.perms@.index(i).pptr() && self.perms@.index(i).is_init()
    }

    pub open spec fn all_available(self) -> bool {
        forall |i: usize| i < self.len() ==> self.available(i)
    }

    pub closed spec fn ptrs(self) -> Seq<PPtr<T>> {
        self.ptrs@
    }

    pub open spec fn availability_unchanged(old: Array<T>, new: Array<T>, i: usize) -> bool
        recommends
            old.len() == new.len(),
            i < old.len(),
    {
        forall |j: usize| j < old.len() && i != j ==> (old.available(j) ==> new.available(j))
    }

    pub open spec fn borrows_unchanged(old: Array<T>, new: Array<T>) -> bool
        recommends
            old.len() == new.len(),
    {
        forall |b: Borrow<T>| old.corresponds(b) ==> new.corresponds(b)
    }

    pub closed spec fn available(self, i: usize) -> bool {
        i < self.len() && self.perms@.contains_key(i)
    }

    pub closed spec fn corresponds(self, borrow: Borrow<T>) -> bool {
        borrow.index() < self.len() && borrow.ptr() == self.ptrs()[borrow.index() as int]
    }

    pub fn new(mut data: Vec<T>) -> (res: Self)
        ensures
            data.len() == res.len(),
            res.wf(),
            res.all_available(),
    {
        let ghost n = data.len();

        let mut data_rev = Vec::<T>::new();
        while data.len() > 0
            invariant
                data_rev.len() + data.len() == n,
        {
            data_rev.push(data.pop().unwrap());
        }
        assert(data_rev.len() == n);
        let mut ptrs = Vec::<PPtr<T>>::new();
        let tracked mut perms = Map::<usize, PointsTo<T>>::tracked_empty();
        let mut i: usize = 0;
        while data_rev.len() > 0
            invariant
                data_rev.len() + i == n,
                ptrs.len() == i,
                forall |j: usize| j < i
                    ==> #[trigger] perms.contains_key(j)
                    ==> ptrs[j as int] == perms.index(j).pptr() && perms.index(j).is_init(),
                forall |j: usize| j < i
                    ==> perms.contains_key(j),
        {
            let x = data_rev.pop().unwrap();
            let (ptr, Tracked(perm)) = PPtr::new(x);
            ptrs.push(ptr);
            proof {
                perms.tracked_insert(i, perm);
            }
            i += 1;
        }
        Self {
            ptrs,
            perms: Tracked(perms),
        }
    }

    pub fn borrow(&mut self, i: usize) -> (res: Borrow<T>)
        requires
            old(self).wf(),
            old(self).available(i),
        ensures
            res.index() == i,
            self.corresponds(res),
            Self::availability_unchanged(*old(self), *self, i),
            Self::borrows_unchanged(*old(self), *self),
            old(self).ptrs() == self.ptrs(),
            self.wf(),
            res.wf(),
    {
        let tracked perm = self.perms.borrow_mut().tracked_remove(i);
        let ptr = self.ptrs[i];
        Borrow {
            index: i,
            ptr,
            perm: Tracked(perm),
        }
    }

    pub fn terminate(&mut self, b: Borrow<T>)
        requires
            old(self).wf(),
            old(self).corresponds(b),
            b.wf(),
        ensures
            Self::availability_unchanged(*old(self), *self, b.index()),
            Self::borrows_unchanged(*old(self), *self),
            old(self).ptrs() == self.ptrs(),
            self.wf(),
            self.available(b.index()),
    {
        let Tracked(perm) = b.perm;
        proof {
            self.perms.borrow_mut().tracked_insert(b.index(), perm);
        }
    }
}

mod tests {
    /// notes: This array is kinda useless in concurrent setting
    /// because it requires mutable reference to do any action
    /// but potentially it can be split into array (vec of pointers) and permission map
    /// this way array is always shared, but permissions can be split and passed to threads

    use super::Array;
    use vstd::prelude::*;

    fn read(x: &i32) {}

    #[verifier::external_body]
    fn print<T: std::fmt::Debug>(a: &mut Array<T>)
        requires old(a).all_available()
    {
        for i in 0..a.length() {
            let b = a.borrow(i);
            print!("{:?} ", b.read());
            a.terminate(b);
        }
        println!("");
    }

    pub fn f() {
        let mut array = Array::new(vec![0, 1, 2]);

        let mut b0 = array.borrow(0);
        // let mut b0_ = array.borrow(0);  // doesn't compile

        let mut b1 = array.borrow(1);

        let b0_i: &i32 = b0.read();
        let b0_ii = b0.read();

        b0.replace(66);

        b1.replace(77);
        array.terminate(b1);

        // read(b0_i);  // won't compile as we use b0 mutably

        array.terminate(b0);

        print(&mut array);
    }
}

fn main() {
    tests::f();
}

}
