use vstd::prelude::*;

verus! {

use vstd::simple_pptr::*;

pub struct Array<T> {
    ptrs: Vec<PPtr<T>>,
}

pub type Perms<T> = Tracked<Map<usize, PointsTo<T>>>;

pub struct Borrow<T> {
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

    pub closed spec fn wf(self, perms: Perms<T>) -> bool {
        forall |i: usize| i < self.len()
            ==> #[trigger] perms@.contains_key(i)
            ==> self.ptrs[i as int] == perms@.index(i).pptr() && perms@.index(i).is_init()
    }

    pub open spec fn all_available(self, perms: Perms<T>) -> bool {
        forall |i: usize| i < self.len() ==> self.available(i, perms)
    }

    pub closed spec fn ptrs(self) -> Seq<PPtr<T>> {
        self.ptrs@
    }

    pub open spec fn availability_unchanged(&self, old: Perms<T>, new: Perms<T>, i: usize) -> bool
        recommends
            i < self.len(),
    {
        forall |j: usize| j < self.len() && i != j ==> (self.available(j, old) ==> self.available(j, new))
    }

    pub open spec fn available(self, i: usize, perms: Perms<T>) -> bool {
        i < self.len() && perms@.contains_key(i)
    }

    pub closed spec fn corresponds(self, borrow: Borrow<T>) -> bool {
        borrow.index() < self.len() && borrow.ptr() == self.ptrs()[borrow.index() as int]
    }

    pub fn new(mut data: Vec<T>) -> (res: (Self, Perms<T>))
        ensures
            data.len() == res.0.len(),
            res.0.wf(res.1),
            res.0.all_available(res.1),
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
        (
            Self {
            ptrs
            },
            Tracked(perms)
        )
    }

    pub fn borrow(&self, i: usize, perms: &mut Perms<T>) -> (res: Borrow<T>)
        requires
            self.wf(*old(perms)),
            self.available(i, *old(perms)),
        ensures
            res.index() == i,
            self.corresponds(res),
            self.availability_unchanged(*old(perms), *perms, i),
            self.wf(*perms),
            res.wf(),
    {
        let tracked perm = perms.borrow_mut().tracked_remove(i);
        let ptr = self.ptrs[i];
        Borrow {
            index: i,
            ptr,
            perm: Tracked(perm),
        }
    }

    pub fn terminate(&self, b: Borrow<T>, perms: &mut Perms<T>)
        requires
            self.wf(*old(perms)),
            self.corresponds(b),
            b.wf(),
        ensures
            self.availability_unchanged(*old(perms), *perms, b.index()),
            self.wf(*perms),
            self.available(b.index(), *perms),
    {
        let Tracked(perm) = b.perm;
        proof {
            perms.borrow_mut().tracked_insert(b.index(), perm);
        }
    }
}

impl<T> Clone for Array<T> {
    /// Shallow clone of pointers
    /// Can be useful for sharing array across threads in Verus
    /// because Verus doesn't support borrowing in threads
    fn clone(&self) -> (res: Self)
        ensures
            forall |p: Perms<T>| self.wf(p) ==> res.wf(p),
            forall |p: Perms<T>, i: usize| self.available(i, p) ==> res.available(i, p),
            forall |b: Borrow<T>| self.corresponds(b) ==> res.corresponds(b),
    {
        Self {
            ptrs: self.ptrs.clone(),
        }
    }
}

mod tests {
    /// notes: This array is kinda useless in concurrent setting
    /// because it requires mutable reference to do any action
    /// but potentially it can be split into array (vec of pointers) and permission map
    /// this way array is always shared, but permissions can be split and passed to threads

    use super::{Array, Perms, Borrow};
    use vstd::prelude::*;
    use vstd::simple_pptr::PointsTo;

    fn read(x: &i32) {}

    #[verifier::external_body]
    fn print<T: std::fmt::Debug>(a: &Array<T>, perms: &mut Perms<T>)
        requires a.all_available(*old(perms))
    {
        for i in 0..a.length() {
            let b = a.borrow(i, perms);
            print!("{:?} ", b.read());
            a.terminate(b, perms);
        }
        println!("");
    }

    pub fn f() {
        let (array, mut perms) = Array::new(vec![0, 1, 2]);

        let mut b0 = array.borrow(0, &mut perms);
        // let mut b0_ = array.borrow(0);  // doesn't compile

        let mut b1 = array.borrow(1, &mut perms);

        let b0_i: &i32 = b0.read();
        let b0_ii = b0.read();

        b0.replace(66);

        b1.replace(77);
        array.terminate(b1, &mut perms);

        // read(b0_i);  // won't compile as we use b0 mutably

        array.terminate(b0, &mut perms);

        print(&array, &mut perms);
    }

    pub fn concurrent() {
        let (array, mut perms) = Array::new(vec![0, 1]);

        let mut b0 = array.borrow(0, &mut perms);
        let mut b1 = array.borrow(1, &mut perms);

        let res0 = vstd::thread::spawn(move || -> (ret: Borrow<i32>)
            ensures array.corresponds(ret), ret.wf(), ret.index() == 0
            {
                let mut b0 = b0;
                b0.replace(66);
                b0
            }
        );
        let res1 = vstd::thread::spawn(move || -> (ret: Borrow<i32>)
            ensures array.corresponds(ret), ret.wf(), ret.index() == 1
            {
                let mut b1 = b1;
                b1.replace(77);
                b1
            }
        );

        let res0 = res0.join();
        let res1 = res1.join();

        let (b0, b1) = match (res0, res1) {
            (Result::Ok(b0), Result::Ok(b1)) => {
                (b0, b1)
            },
            _ => {
                // panic!("error");
                return;
            }
        };

        array.terminate(b0, &mut perms);
        array.terminate(b1, &mut perms);
        print(&array, &mut perms);
    }

    pub fn concurrent2() {
        // This is about splitting permission set

        let (array, mut perms) = Array::new(vec![0, 1]);
        let array_r1 = array.clone();
        let array_r2 = array.clone();

        let mut perms1 = Tracked(Map::<usize, PointsTo<i32>>::tracked_empty());
        proof {
            assert(array.available(1, perms));
            assert(array.available(0, perms));
            let tracked p1 = perms.borrow_mut().tracked_remove(1);
            perms1.borrow_mut().tracked_insert(1, p1);
        }

        let perms0 = vstd::thread::spawn(move || -> (ret: Perms<i32>)
            ensures array.available(0, ret)
            {
                let mut perms0 = perms;
                let mut b0 = array_r1.borrow(0, &mut perms0);
                b0.replace(66);
                array_r1.terminate(b0, &mut perms0);
                perms0
            }
        );
        let perms1 = vstd::thread::spawn(move || -> (ret: Perms<i32>)
            ensures array.available(1, ret)
            {
                let mut perms1 = perms1;
                let mut b1 = array_r2.borrow(1, &mut perms1);
                b1.replace(77);
                array_r2.terminate(b1, &mut perms1);
                perms1
            }
        );

        let res0 = perms0.join();
        let res1 = perms1.join();

        let (mut perms0, mut perms1) = match (res0, res1) {
            (Result::Ok(x0), Result::Ok(x1)) => {
                (x0, x1)
            },
            _ => {
                // panic!("error");
                return;
            }
        };

        proof {
            let tracked p1 = perms1.borrow_mut().tracked_remove(1);
            perms0.borrow_mut().tracked_insert(1, p1);
        }

        print(&array, &mut perms0);
    }
}

fn main() {
    // tests::f();
    // tests::concurrent();
    tests::concurrent2();
}

}
