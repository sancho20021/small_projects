pub fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }

    merge_sort(&mut arr[0..mid]);
    merge_sort(&mut arr[mid..]);

    let res = merge(&arr[0..mid], &arr[mid..]);
    arr.copy_from_slice(&res[..]);
}

fn merge<T: Ord + Copy>(left: &[T], right: &[T]) -> Vec<T> {
    let mut ret: Vec<T> = Vec::with_capacity(left.len() + right.len());

    let mut left_index = 0;
    let mut right_index = 0;
    while left_index < left.len() && right_index < right.len() {
        let element: T;
        if left[left_index] <= right[right_index] {
            element = left[left_index];
            left_index += 1;
        } else {
            element = right[right_index];
            right_index += 1;
        }
        ret.push(element);
    }

    if left_index < left.len() {
        ret.extend_from_slice(&left[left_index..]);
    } else if right_index < right.len() {
        ret.extend_from_slice(&right[right_index..]);
    }
    ret
}

pub fn merge_sort_parallel<T: Ord + Send + Copy>(arr: &mut [T], threshold: usize) {
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }

    if arr.len() < threshold {
        merge_sort(arr);
        return;
    }

    let (left, right) = arr.split_at_mut(mid);

    let mut left_clone = left.to_vec();
    let mut right_clone = right.to_vec();

    std::thread::scope(|s| {
        let left_handle = s.spawn(|| {
            merge_sort_parallel(&mut left_clone, threshold);
        });
        let right_handle = s.spawn(|| {
            merge_sort_parallel(&mut right_clone, threshold);
        });
        left_handle.join().unwrap();
        right_handle.join().unwrap();
    });

    let res = merge(&left_clone, &right_clone);
    arr.copy_from_slice(&res[..]);
}

pub fn merge_sort_threadpool<T: Ord + Send + Copy>(arr: &mut [T], threshold: usize) {
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }

    if arr.len() < threshold {
        merge_sort(arr);
        return;
    }

    let (left, right) = arr.split_at_mut(mid);

    let mut left_clone = left.to_vec();
    let mut right_clone = right.to_vec();

    rayon::join(
        || merge_sort_threadpool(&mut left_clone, threshold),
        || merge_sort_threadpool(&mut right_clone, threshold),
    );

    let res = merge(&left_clone, &right_clone);
    arr.copy_from_slice(&res[..]);
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use disjoint_mut_test::disjoint_verified::{
        self,
        exec_pcell::Array,
        split_at::{ArrayAbstraction, RegionArray},
    };

    use super::{merge_sort, merge_sort_parallel, merge_sort_threadpool};

    #[test]
    fn test_seq_sort() {
        let mut array = vec![5, 4, 3, 2, 1];
        merge_sort(array.as_mut_slice());
        assert_eq!(array, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_par_sort() {
        let mut array = vec![5, 4, 3, 2, 1];
        merge_sort_parallel(array.as_mut_slice(), 2);
        assert_eq!(array, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_tp_sort() {
        let mut array = vec![5, 4, 3, 2, 1];
        merge_sort_threadpool(array.as_mut_slice(), 2);
        assert_eq!(array, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_seq_array() {
        let mut arr = ArrayAbstraction::new(vec![5, 4, 3, 2, 1]);
        disjoint_verified::split_at::mergesort::merge_sort_abstraction(&mut arr, vec![0; 5]);
        let arr = arr.clone_to_vec();
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_par_array() {
        let mut arr = ArrayAbstraction::new(vec![5, 4, 3, 2, 1]);
        disjoint_verified::split_at::mergesort::merge_sort_parallel_abstraction(
            &mut arr,
            vec![0; 5],
            2,
        )
        .unwrap();
        let arr = arr.clone_to_vec();
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
}
