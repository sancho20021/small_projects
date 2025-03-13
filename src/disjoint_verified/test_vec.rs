use vstd::prelude::*;

verus!{

use std::collections::HashMap;

struct Refs<'a, T> {
    refs: HashMap<usize, &'a mut T>,
}

impl<'a, T> Refs<'a, T> {
    pub fn new(refs: HashMap<usize, &'a mut T>) -> Self {
        Self {
            refs
        }
    }

    #[verifier::external_body]
    pub fn from_vec(vec: &'a mut Vec<T>) -> Self {
        let mut res = HashMap::new();
        for (i, x) in (0..vec.len()).into_iter().zip(vec.iter_mut()) {
            res.insert(i, x);
        }
        Self::new(res)
    }

    pub fn borrow_mut<'b>(&'b mut self, i: usize) -> &'a mut T {
        self.refs.remove(&i).unwrap()
    }

    pub fn terminate<'b>(&'b mut self, borrow: &'a mut T) {
        // todo: we have to check that this borrow is exactly of ith element and of this vector
        // self.refs.insert(k, v)
    }
}

fn main() {

}

}
