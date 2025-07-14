pub mod slices;
pub mod slices_unchecked;
pub mod verus;

pub type Element = i32;

pub trait Sort {
    type Array;
    fn prepare_array(input: Vec<Element>) -> (Self::Array, Self::Array);
    fn sort(input: &mut Self::Array, buf: &mut Self::Array);
    fn sort_parallel(input: &mut Self::Array, buf: &mut Self::Array, threshold: usize);
    fn name() -> &'static str;
}
