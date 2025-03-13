#![feature(allocator_api)]

use vstd::prelude::*;
use vstd::cell::*;

verus!{

use std::rc::Rc;

// drop
// strong_count
// into_inner

pub closed spec fn ghost_strong_count<T, A>(rc: &Rc<T, A>) -> usize;

pub assume_specification<T, A>[Rc::<T, A>::strong_count](rc: &Rc<T, A>) -> (res: usize)
    where
        T: ?std::marker::Sized,
        A: std::alloc::Allocator,
    ensures
        res == ghost_strong_count(rc)
;



fn f() {
    let x: Rc<i32> = Rc::new(0);
    // let y = Rc::strong_count(&x);
    // drop(x);
}

}
