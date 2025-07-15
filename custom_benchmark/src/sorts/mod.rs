use disjoint_mut_test::mergesort::ArrayForSorting;

#[cfg(test)]
use enum_iterator::{Sequence, all};
use rayon::slice::ParallelSliceMut;

pub mod naked_verus;
pub mod slices;
pub mod slices_unchecked;
pub mod slices_unchecked_vspawn;

pub type Element = i32;

pub enum InputArray {
    Vec(Vec<Element>),
    Verus(ArrayForSorting<Element>),
}

impl InputArray {
    fn unwrap_as_vec(&mut self) -> &mut Vec<Element> {
        match self {
            InputArray::Vec(vec) => vec,
            _ => panic!("wrong input type"),
        }
    }

    fn unwrap_as_verus(&mut self) -> &mut ArrayForSorting<Element> {
        match self {
            InputArray::Verus(vec) => vec,
            _ => panic!("wrong input type"),
        }
    }

    fn clone_to_vec(&self) -> Vec<Element> {
        match self {
            InputArray::Vec(items) => items.clone(),
            InputArray::Verus(items) => items.clone_to_vec(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[cfg_attr(test, derive(Sequence))]
pub enum SeqSort {
    Slices,
    SlicesUnchecked,
    Verus,
    NakedVerus,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[cfg_attr(test, derive(Sequence))]
pub enum ParSort {
    Slices,
    SlicesUnchecked,
    Verus,
    NakedVerus,
    SlicesUncheckedVspawn,
    Rayon,
}

pub trait HasName {
    fn name(&self) -> &'static str;
}

impl HasName for SeqSort {
    fn name(&self) -> &'static str {
        match self {
            Self::Slices => "slices",
            Self::SlicesUnchecked => "slices unchecked",
            Self::Verus => "verus",
            Self::NakedVerus => "naked verus",
        }
    }
}

impl HasName for ParSort {
    fn name(&self) -> &'static str {
        match self {
            Self::Slices => "slices",
            Self::SlicesUnchecked => "slices unchecked",
            Self::Verus => "verus",
            Self::NakedVerus => "naked verus",
            Self::SlicesUncheckedVspawn => "vspawn slices unchecked",
            Self::Rayon => "rayon",
        }
    }
}

#[derive(Clone, Copy)]
pub enum Sort {
    Seq(SeqSort),
    Par(ParSort),
}

fn normal_sort(
    sort: impl Fn(&mut [Element], &mut [Element]),
    input: &mut InputArray,
    buf: &mut InputArray,
) {
    sort(input.unwrap_as_vec(), buf.unwrap_as_vec());
}

fn normal_sort_par(
    sort: impl Fn(&mut [Element], &mut [Element], usize) -> Result<(), ()>,
    input: &mut InputArray,
    buf: &mut InputArray,
    threshold: usize,
) -> Result<(), ()> {
    sort(input.unwrap_as_vec(), buf.unwrap_as_vec(), threshold)
}

impl SeqSort {
    pub fn sort(&self, input: &mut InputArray, buf: &mut InputArray) {
        match self {
            SeqSort::Slices => normal_sort(slices::merge_sort, input, buf),
            SeqSort::SlicesUnchecked => normal_sort(slices_unchecked::merge_sort, input, buf),
            SeqSort::Verus => disjoint_mut_test::mergesort::merge_sort(
                input.unwrap_as_verus(),
                buf.unwrap_as_verus(),
            ),
            SeqSort::NakedVerus => {
                let (input, buf) = (input.unwrap_as_vec(), buf.unwrap_as_vec());
                let input_a = naked_verus::Array(input.as_ptr() as *mut i32);
                let buf_a = naked_verus::Array(buf.as_ptr() as *mut i32);
                naked_verus::merge_sort(input_a, 0, input.len(), buf_a)
            }
        }
    }
}

impl ParSort {
    pub fn sort_parallel(&self, input: &mut InputArray, buf: &mut InputArray, threshold: usize) {
        match self {
            ParSort::Slices => normal_sort_par(slices::_merge_sort_parallel, input, buf, threshold),
            ParSort::SlicesUnchecked => normal_sort_par(
                slices_unchecked::_merge_sort_parallel,
                input,
                buf,
                threshold,
            ),
            ParSort::Verus => disjoint_mut_test::mergesort::merge_sort_parallel(
                input.unwrap_as_verus(),
                buf.unwrap_as_verus(),
                threshold,
            ),
            ParSort::NakedVerus => {
                let (input, buf) = (input.unwrap_as_vec(), buf.unwrap_as_vec());
                let input_a = naked_verus::Array(input.as_ptr() as *mut i32);
                let buf_a = naked_verus::Array(buf.as_ptr() as *mut i32);
                naked_verus::_merge_sort_parallel(input_a, 0, input.len(), buf_a, threshold)
            }
            ParSort::SlicesUncheckedVspawn => normal_sort_par(
                slices_unchecked_vspawn::_merge_sort_parallel,
                input,
                buf,
                threshold,
            ),
            ParSort::Rayon => {
                normal_sort_par(|input, _, _| Ok(input.par_sort()), input, buf, threshold)
            }
        }
        .unwrap();
    }
}

impl Sort {
    pub fn prepare_array(&self, input: Vec<Element>) -> (InputArray, InputArray) {
        let buf = vec![0; input.len()];
        let mapper = |input| match self {
            Sort::Seq(SeqSort::Verus) | Sort::Par(ParSort::Verus) => {
                InputArray::Verus(ArrayForSorting::new(input))
            }
            _ => InputArray::Vec(input),
        };
        (mapper(input), mapper(buf))
    }
}

impl std::fmt::Debug for SeqSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Slices => "slices",
            Self::SlicesUnchecked => "slices unchecked",
            Self::Verus => "verus",
            Self::NakedVerus => "naked verus",
        };
        write!(f, "\"{s}\"")
    }
}

impl std::fmt::Debug for ParSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Slices => "slices",
            Self::SlicesUnchecked => "slices unchecked",
            Self::Verus => "verus",
            Self::NakedVerus => "naked verus",
            Self::SlicesUncheckedVspawn => "vspawn slices unchecked",
            Self::Rayon => "rayon",
        };
        write!(f, "\"{s}\"")
    }
}

#[test]
fn test_all_seq_sorts() {
    for sort in all::<SeqSort>() {
        let input = vec![2, 3, 5, 1, 4];
        let (mut input, mut buf) = Sort::Seq(sort).prepare_array(input);
        sort.sort(&mut input, &mut buf);
        let input = input.clone_to_vec();
        assert_eq!(input, vec![1, 2, 3, 4, 5]);
    }
}

#[test]
fn test_all_par_sorts() {
    for sort in all::<ParSort>() {
        let input = vec![2, 3, 5, 1, 4];
        let (mut input, mut buf) = Sort::Par(sort).prepare_array(input);
        sort.sort_parallel(&mut input, &mut buf, 2);
        let input = input.clone_to_vec();
        assert_eq!(input, vec![1, 2, 3, 4, 5]);
    }
}
