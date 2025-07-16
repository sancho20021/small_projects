use std::sync::Arc;

use disjoint_mut_test::mergesort::ArrayForSorting;

#[cfg(test)]
use enum_iterator::{Sequence, all};
use rayon::slice::ParallelSliceMut;

use crate::sorts;

pub mod naked_verus;
pub mod naked_verus_arc;
pub mod naked_verus_arc_clone;
pub mod naked_verus_arc_clone_no_scope;
pub mod naked_verus_arc_no_scope;
pub mod naked_verus_no_scope;
pub mod naked_verus_rayon;
pub mod slices;
pub mod slices_blackbox;
pub mod slices_unchecked;
pub mod slices_unchecked_vspawn;
pub mod verus_no_g_less_arcs;
pub mod verus_no_g_no_arc;
pub mod verus_no_ghost;
pub mod verus_no_ghost_muninit;

pub type Element = i32;

pub enum InputArray {
    Vec(Vec<Element>),
    Verus(ArrayForSorting<Element>),
    VerusNoGhost(verus_no_ghost::ArrayForSorting<Element>),
    VerusNoGhostNoArc(verus_no_g_no_arc::ArrayForSorting<Element>),
    VerusNoGhostLessArcs(verus_no_g_less_arcs::ArrayForSorting<Element>),
    VerusNoGhostMuninit(verus_no_ghost_muninit::ArrayForSorting<Element>),
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

    fn unwrap_as_verus_ng(&mut self) -> &mut verus_no_ghost::ArrayForSorting<Element> {
        match self {
            InputArray::VerusNoGhost(vec) => vec,
            _ => panic!("wrong input type"),
        }
    }

    fn unwrap_as_verus_ng_na(&mut self) -> &mut verus_no_g_no_arc::ArrayForSorting<Element> {
        match self {
            InputArray::VerusNoGhostNoArc(vec) => vec,
            _ => panic!("wrong input type"),
        }
    }

    fn unwrap_as_verus_ng_less_arcs(
        &mut self,
    ) -> &mut verus_no_g_less_arcs::ArrayForSorting<Element> {
        match self {
            InputArray::VerusNoGhostLessArcs(vec) => vec,
            _ => panic!("wrong input type"),
        }
    }

    fn unwrap_as_verus_ng_muninit(
        &mut self,
    ) -> &mut verus_no_ghost_muninit::ArrayForSorting<Element> {
        match self {
            InputArray::VerusNoGhostMuninit(vec) => vec,
            _ => panic!("wrong input type"),
        }
    }

    fn clone_to_vec(&self) -> Vec<Element> {
        match self {
            InputArray::Vec(items) => items.clone(),
            InputArray::Verus(items) => items.clone_to_vec(),
            InputArray::VerusNoGhost(items) => items.clone_to_vec(),
            InputArray::VerusNoGhostNoArc(items) => items.clone_to_vec(),
            InputArray::VerusNoGhostLessArcs(items) => items.clone_to_vec(),
            InputArray::VerusNoGhostMuninit(items) => items.clone_to_vec(),
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
    SlicesBlackbox,
    SlicesUnchecked,
    Verus,
    VerusNoGhost,
    VerusNoGhostNoArc,
    VerusNoGhostLessArcs,
    VerusNoGhostMuninit,
    ImposterSlices,
    VerusLessArcs,
    NakedVerus,
    NakedVerusNoScope,
    NakedVerusArcClone,
    NakedVerusArcCloneNoScope,
    NakedVerusArc,
    NakedVerusArcNoScope,
    NakedVerusRayon,
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
            Self::ImposterSlices => "imposter slices",
            Self::SlicesBlackbox => "slices blackbox",
            Self::SlicesUnchecked => "slices unchecked",
            Self::Verus => "verus",
            Self::VerusNoGhost => "verus no ghost",
            Self::VerusNoGhostNoArc => "verus no ghost no arc",
            Self::VerusNoGhostLessArcs => "verus no ghost less arcs",
            Self::VerusNoGhostMuninit => "verus no ghost maybe uninit",
            Self::VerusLessArcs => "verus less arcs",
            Self::NakedVerus => "naked verus",
            Self::NakedVerusNoScope => "naked verus no scope",
            Self::NakedVerusArcClone => "naked verus arc clone",
            Self::NakedVerusArcCloneNoScope => "naked verus arc clone no scope",
            Self::NakedVerusArc => "naked verus arc",
            Self::NakedVerusArcNoScope => "naked verus arc no scope",
            Self::SlicesUncheckedVspawn => "vspawn slices unchecked",
            Self::Rayon => "rayon",
            Self::NakedVerusRayon => "naked verus rayon",
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
            ParSort::ImposterSlices => {
                normal_sort_par(slices::_merge_sort_parallel, input, buf, threshold)
            }
            ParSort::SlicesBlackbox => {
                normal_sort_par(slices_blackbox::_merge_sort_parallel, input, buf, threshold)
            }
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
            ParSort::VerusNoGhost => sorts::verus_no_ghost::merge_sort_parallel(
                input.unwrap_as_verus_ng(),
                buf.unwrap_as_verus_ng(),
                threshold,
            ),
            ParSort::VerusNoGhostNoArc => sorts::verus_no_g_no_arc::merge_sort_parallel(
                input.unwrap_as_verus_ng_na(),
                buf.unwrap_as_verus_ng_na(),
                threshold,
            ),
            ParSort::VerusNoGhostLessArcs => sorts::verus_no_g_less_arcs::merge_sort_parallel(
                input.unwrap_as_verus_ng_less_arcs(),
                buf.unwrap_as_verus_ng_less_arcs(),
                threshold,
            ),
            ParSort::VerusNoGhostMuninit => sorts::verus_no_ghost_muninit::merge_sort_parallel(
                input.unwrap_as_verus_ng_muninit(),
                buf.unwrap_as_verus_ng_muninit(),
                threshold,
            ),
            ParSort::VerusLessArcs => disjoint_mut_test::mergesort_less_arcs::merge_sort_parallel(
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
            ParSort::NakedVerusNoScope => {
                let (input, buf) = (input.unwrap_as_vec(), buf.unwrap_as_vec());
                let input_a = naked_verus_no_scope::Array(input.as_ptr() as *mut i32);
                let buf_a = naked_verus_no_scope::Array(buf.as_ptr() as *mut i32);
                naked_verus_no_scope::_merge_sort_parallel(
                    input_a,
                    0,
                    input.len(),
                    buf_a,
                    threshold,
                )
            }
            ParSort::NakedVerusArcClone => {
                let (input, buf) = (input.unwrap_as_vec(), buf.unwrap_as_vec());
                let input_a = Arc::new(naked_verus_arc_clone::Array(input.as_ptr() as *mut i32));
                let buf_a = Arc::new(naked_verus_arc_clone::Array(buf.as_ptr() as *mut i32));
                naked_verus_arc_clone::_merge_sort_parallel(
                    input_a,
                    0,
                    input.len(),
                    buf_a,
                    threshold,
                )
            }
            ParSort::NakedVerusArcCloneNoScope => {
                let (input, buf) = (input.unwrap_as_vec(), buf.unwrap_as_vec());
                let input_a = Arc::new(naked_verus_arc_clone_no_scope::Array(
                    input.as_ptr() as *mut i32
                ));
                let buf_a = Arc::new(naked_verus_arc_clone_no_scope::Array(
                    buf.as_ptr() as *mut i32
                ));
                naked_verus_arc_clone_no_scope::_merge_sort_parallel(
                    input_a,
                    0,
                    input.len(),
                    buf_a,
                    threshold,
                )
            }
            ParSort::NakedVerusArc => {
                let (input, buf) = (input.unwrap_as_vec(), buf.unwrap_as_vec());
                let input_a = Arc::new(naked_verus_arc::Array(input.as_ptr() as *mut i32));
                let buf_a = Arc::new(naked_verus_arc::Array(buf.as_ptr() as *mut i32));
                naked_verus_arc::_merge_sort_parallel(input_a, 0, input.len(), buf_a, threshold)
            }
            ParSort::NakedVerusArcNoScope => {
                let (input, buf) = (input.unwrap_as_vec(), buf.unwrap_as_vec());
                let input_a = Arc::new(naked_verus_arc_no_scope::Array(input.as_ptr() as *mut i32));
                let buf_a = Arc::new(naked_verus_arc_no_scope::Array(buf.as_ptr() as *mut i32));
                naked_verus_arc_no_scope::_merge_sort_parallel(
                    input_a,
                    0,
                    input.len(),
                    buf_a,
                    threshold,
                )
            }
            ParSort::NakedVerusRayon => {
                let (input, buf) = (input.unwrap_as_vec(), buf.unwrap_as_vec());
                let input_a = naked_verus::Array(input.as_ptr() as *mut i32);
                let buf_a = naked_verus::Array(buf.as_ptr() as *mut i32);
                naked_verus_rayon::_merge_sort_parallel(input_a, 0, input.len(), buf_a)
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
            Sort::Seq(SeqSort::Verus)
            | Sort::Par(ParSort::Verus)
            | Sort::Par(ParSort::VerusLessArcs) => InputArray::Verus(ArrayForSorting::new(input)),
            Sort::Par(ParSort::VerusNoGhostNoArc) => {
                InputArray::VerusNoGhostNoArc(verus_no_g_no_arc::ArrayForSorting::new(input))
            }
            Sort::Par(ParSort::VerusNoGhost) => {
                InputArray::VerusNoGhost(verus_no_ghost::ArrayForSorting::new(input))
            }
            Sort::Par(ParSort::VerusNoGhostLessArcs) => {
                InputArray::VerusNoGhostLessArcs(verus_no_g_less_arcs::ArrayForSorting::new(input))
            }
            Sort::Par(ParSort::VerusNoGhostMuninit) => {
                InputArray::VerusNoGhostMuninit(verus_no_ghost_muninit::ArrayForSorting::new(input))
            }
            _ => InputArray::Vec(input),
        };
        (mapper(input), mapper(buf))
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
