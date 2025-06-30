use vstd::prelude::*;

verus! {

use vstd::cell::*;
use std::sync::Arc;

pub struct Array<T> {
    ptrs: Vec<PCell<T>>,
}

pub type Perms<T> = Tracked<Map<usize, PointsTo<T>>>;
pub type SpecPerms<T> = Map<usize, PointsTo<T>>;

proof fn unfold_submap<V>(m1: Tracked<Map<usize, V>>, m2: Tracked<Map<usize, V>>)
    requires
        m1@.submap_of(m2@),
    ensures
        forall |i: usize| #[trigger] m1@.dom().contains(i) ==> m2@.dom().contains(i) && m1@[i] == m2@[i]
{

}

impl<T> Array<T> {
    pub closed spec fn len(&self) -> usize {
        self.ptrs.len()
    }

    pub fn length(&self) -> (res: usize)
        ensures res == self.len()
    {
        self.ptrs.len()
    }

    pub closed spec fn wf(self, perms: SpecPerms<T>) -> bool {
        forall |i: usize| i < self.len()
            ==> #[trigger] perms.contains_key(i)
            ==> self.ptrs[i as int].id() == perms.index(i).id() && perms.index(i).is_init()
    }

    spec fn wfi(self, perms: SpecPerms<T>, i: usize) -> bool {
        self.ptrs[i as int].id() == perms.index(i).id() && perms.index(i).is_init()
    }

    pub proof fn submap_wf(self, perms: SpecPerms<T>, sub_perms: SpecPerms<T>)
        requires
            sub_perms.submap_of(perms),
            self.wf(perms),
        ensures
            self.wf(sub_perms),
    {
        assert(forall |i: usize| #[trigger] sub_perms.contains_key(i) ==> perms.contains_key(i));
    }

    pub proof fn union_wf(self, perms1: SpecPerms<T>, perms2: SpecPerms<T>)
        requires
            self.wf(perms1),
            self.wf(perms2),
        ensures
            self.wf(perms1.union_prefer_right(perms2)),
    {}

    pub open spec fn all_available(self, perms: SpecPerms<T>) -> bool {
        forall |i: usize| i < self.len() ==> self.available(i, perms)
    }

    pub open spec fn availability_unchanged(&self, old: SpecPerms<T>, new: SpecPerms<T>) -> bool
    {
        forall |j: usize| self.available(j, old) <==> self.available(j, new)
    }

    pub open spec fn available(self, i: usize, perms: SpecPerms<T>) -> bool {
        perms.contains_key(i)
    }

    pub fn new(mut data: Vec<T>) -> (res: (Self, Perms<T>))
        ensures
            data.len() == res.0.len(),
            res.0.wf(res.1@),
            res.0.all_available(res.1@),
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
        let mut ptrs = Vec::<PCell<T>>::new();
        let tracked mut perms = Map::<usize, PointsTo<T>>::tracked_empty();
        let mut i: usize = 0;
        while data_rev.len() > 0
            invariant
                data_rev.len() + i == n,
                ptrs.len() == i,
                forall |j: usize| j < i
                    ==> #[trigger] perms.contains_key(j)
                    ==> ptrs[j as int].id() == perms.index(j).id() && perms.index(j).is_init(),
                forall |j: usize| j < i
                    ==> perms.contains_key(j),
        {
            let x = data_rev.pop().unwrap();
            let (ptr, Tracked(perm)) = PCell::new(x);
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

    #[verifier::external_body]
    fn vec_replace(v: &Vec<PCell<T>>, i: usize, e: T, Tracked(perm): Tracked<&mut PointsTo<T>>) -> (res: T)
        requires
            i < v@.len(),
            old(perm).is_init(),
            v@[i as int].id() == old(perm)@.pcell,
        ensures
            perm.is_init(),
            perm@.value.unwrap() == e,
            v@[i as int].id() == perm@.pcell,
            res == old(perm)@.value.unwrap(),
    {
        // if i >= v.len() {
        //     panic!("Lol");
        // }
        let vi = unsafe { v.get_unchecked(i) };
        // panic!();
        vi.replace(Tracked(perm), e)
    }

    #[verifier::external_body]
    fn vec_borrow<'a>(v: &'a Vec<PCell<T>>, i: usize, Tracked(perm): Tracked<&'a PointsTo<T>>) -> (res: &'a T)
        requires
            i < v@.len(),
            perm.is_init(),
            v@[i as int].id() == perm@.pcell,
        ensures
            res == perm@.value.unwrap(),
    {
        // if i >= v.len() {
        //     panic!("Lol");
        // }
        let vi = unsafe { v.get_unchecked(i) };
        vi.borrow(Tracked(perm))
    }

    pub fn replace(&self, i: usize, x: T, perms: &mut Perms<T>) -> (res: T)
        requires
            i < self.len(),
            self.wf((*old(perms))@),
            self.available(i, (*old(perms))@),
        ensures
            self.availability_unchanged((*old(perms))@, (*perms)@),
            self.wf((*perms)@)
    {
        let tracked mut perm = perms.borrow_mut().tracked_remove(i);
        let ghost old_perm = perm;
        let res = Self::vec_replace(&self.ptrs, i, x, Tracked(&mut perm));
        // let res = self.ptrs[i].replace(Tracked(&mut perm), x);
        proof {
            perms.borrow_mut().tracked_insert(i, perm);
        }
        res
    }

    pub fn read<'a>(&'a self, i: usize, perms: &'a Perms<T>) -> (res: &'a T)
        requires
            i < self.len(),
            self.wf((*perms)@),
            self.available(i, (*perms)@),
    {
        let tracked perm = perms.borrow().tracked_borrow(i);
        Self::vec_borrow(&self.ptrs, i, Tracked(perm))
        // self.ptrs[i].borrow(Tracked(perm))
    }

    /// probably it should be implemented not as clone,
    /// but as consuming into Vec<T>,
    /// but to make it faster I have to cast?? TODO
    pub fn clone_to_vec<'a>(&'a self, perms: &'a Perms<T>) -> (res: Vec<T>)
        where T: Clone,
        requires
            self.wf(perms@),
            self.all_available(perms@),
    {
        let mut res: Vec<T> = Vec::with_capacity(self.length());
        let mut i: usize = 0;
        while i < self.length()
            invariant
                i <= self.len(),
                self.wf((*perms)@),
                self.all_available(perms@),
        {
            res.push(self.read(i, perms).clone());
            i += 1;
        }
        res
    }
}

fn main() {
    let (array, mut perms) = Array::new(vec![0, 1]);
    let array = Arc::new(array);
    let array_r1 = Arc::clone(&array);
    let array_r2 = Arc::clone(&array);

    let mut perms1 = Tracked(Map::<usize, PointsTo<i32>>::tracked_empty());
    proof {
        assert(array.available(1, perms@));
        assert(array.available(0, perms@));
        let tracked p1 = perms.borrow_mut().tracked_remove(1);
        perms1.borrow_mut().tracked_insert(1, p1);
    }

    let perms0 = vstd::thread::spawn(move || -> (ret: Perms<i32>)
        ensures array.available(0, ret@)
        {
            let mut perms0 = perms;
            (*array_r1).replace(0, 66, &mut perms0);
            perms0
        }
    );
    let perms1 = vstd::thread::spawn(move || -> (ret: Perms<i32>)
        ensures array.available(1, ret@)
        {
            let mut perms1 = perms1;
            (*array_r2).replace(1, 77, &mut perms1);
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

#[verifier::external_body]
fn print<T: std::fmt::Debug>(a: &Array<T>, perms: &mut Perms<T>)
    requires a.all_available((*old(perms))@)
{
    for i in 0..a.length() {
        let b = a.read(i, perms);
        print!("{:?} ", b);
    }
    println!("");
}

}
