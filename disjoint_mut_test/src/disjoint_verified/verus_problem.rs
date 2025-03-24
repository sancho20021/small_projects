use vstd::prelude::*;

verus! {


pub fn g(p: Ghost<Set<nat>>)
requires p@.finite()
{
    assert(p@.insert(0).len() > 0);
}

// pub fn f()
// {
//     let s: Ghost<Set<nat>> = Ghost(Set::empty());
//     assert(s@.insert(0).len() > 0);
// }

fn main() {

}

}
