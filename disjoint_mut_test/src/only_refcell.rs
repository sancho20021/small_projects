use std::cell::{Ref, RefCell, RefMut};

pub struct Disjoint<T> {
    // If we replace it with static cells (well, only could work with QCell as we can't put different type cells
    // in one collection), then we can't store tokens inside, as we have to borrow them mutably
    // we could return a list of tokens to the user, but it is quite hard to work with, as
    // use has to make sure they give back the right index permission, otherwise it will panic
    // we want the type system to check it
    data: Vec<RefCell<T>>,
}

// pub struct PermSet()

impl<T> Disjoint<T> {
    pub fn new<I: IntoIterator<Item=T>>(es: I) -> Self {
        Self { data: es.into_iter().map(RefCell::new).collect() }
    }

    pub fn index(&self, i: usize) -> Ref<'_, T> {
        self.data[i].borrow()
    }

    pub fn index_mut(&self, i: usize) -> RefMut<'_, T> {
        self.data[i].borrow_mut()
    }
}

#[cfg(test)]
mod test {
    use super::Disjoint;

    #[test]
    fn test_working() {
        let v = Disjoint::new(vec![1, 2, 3, 4]);
        {
            let mut v2 = v.index_mut(2);
            let mut v3 = v.index_mut(3);
            *v2 = 66;
            *v3 = 77;
        }
        let v2 = v.index(2);
        let v3 = v.index(3);
        assert_eq!(*v2, 66);
        assert_eq!(*v3, 77);
    }

    #[test]
    #[should_panic]
    fn test_overlapping() {
        let v = Disjoint::new([1, 2, 3, 4]);
        {
            let mut v2 = v.index_mut(2);
            let mut v3 = v.index_mut(2);
            *v2 = 66;
            *v3 = 77;
        }
    }
}
