pub mod single_array_sort;
pub mod slices_sort;
pub mod slices_unchecked_sort;
pub mod threshold_calc;
pub mod single_array_optimized;

pub trait Sort {
    fn sort(input: &mut [i32], threshold: usize);
}
