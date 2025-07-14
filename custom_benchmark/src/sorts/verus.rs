use disjoint_mut_test::mergesort::ArrayForSorting;

use crate::sorts::{Element, Sort};

pub struct Verus;

impl Sort for Verus {
    type Array = ArrayForSorting<Element>;

    fn prepare_array(input: Vec<Element>) -> (Self::Array, Self::Array) {
        let buf = vec![0; input.len()];
        (ArrayForSorting::new(input), ArrayForSorting::new(buf))
    }

    fn sort(input: &mut Self::Array, buf: &mut Self::Array) {
        disjoint_mut_test::mergesort::merge_sort(input, buf);
    }

    fn sort_parallel(input: &mut Self::Array, buf: &mut Self::Array, threshold: usize) {
        disjoint_mut_test::mergesort::merge_sort_parallel(input, buf, threshold)
            .expect("Error while sorting");
    }

    fn name() -> &'static str {
        "verus"
    }
}

#[test]
fn test() {
    let a = vec![2, 3, 5, 1, 4];
    let (mut a, mut buf) = Verus::prepare_array(a);
    Verus::sort_parallel(&mut a, &mut buf, 2);
    let res = a.clone_to_vec();
    assert_eq!(res, vec![1, 2, 3, 4, 5]);
}
