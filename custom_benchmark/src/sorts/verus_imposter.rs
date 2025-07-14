use disjoint_mut_test::mergesort::ArrayForSorting;

use crate::sorts::{Element, Sort};

pub struct VerusImposter;

impl Sort for VerusImposter {
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
        "verus imposter"
    }
}
