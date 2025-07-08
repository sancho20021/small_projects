use std::{collections::HashSet, time::Duration};

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::{
    seq::{IndexedRandom, SliceRandom},
    RngCore,
};

pub fn get_input_array(size: usize) -> Vec<i32> {
    let mut rng = rand::rng();

    let mut v = vec![];
    for i in 0..size {
        v.push(rng.next_u32() as i32);
    }
    v.reverse();
    v
}

fn generate_merge_task(size: usize) -> (Vec<i32>, Vec<i32>) {
    let mut rng = rand::rng();

    let mut array = get_input_array(size * 2);
    array.sort();

    let all_indices = (0..array.len()).collect::<Vec<_>>();
    let mut indices_right = all_indices
        .choose_multiple(&mut rng, size)
        .map(|x| *x)
        .collect::<Vec<_>>();
    let mut indices_left = {
        let mut indices = all_indices.clone().into_iter().collect::<HashSet<_>>();
        let indices_right = indices_right.clone().into_iter().collect::<HashSet<_>>();
        indices.retain(|x| !indices_right.contains(x));
        indices.into_iter().collect::<Vec<_>>()
    };
    assert_eq!(indices_left.len(), indices_right.len());
    assert_eq!(indices_left.len(), size);
    indices_left.sort();
    indices_right.sort();

    let left = indices_left
        .into_iter()
        .map(|i| array[i])
        .collect::<Vec<_>>();
    let right = indices_right
        .into_iter()
        .map(|i| array[i])
        .collect::<Vec<_>>();
    assert!(left.is_sorted());
    assert!(right.is_sorted());
    (left, right)
}

#[test]
fn test_generate_merge_task() {
    let (left, right) = generate_merge_task(3);
    println!("{left:?}");
    println!("{right:?}");
}


fn standard_merge(c: &mut Criterion) {
    for size in ARRAY_SIZES {
        let (left, right) = generate_merge_task(size);
        let mut out = vec![0; size * 2];

        c.bench_with_input(BenchmarkId::new("standard merge", size), &size, |b, _| {
            b.iter(|| {
                mergesort_benchmarks::merge_sorts::merge(
                    black_box(&left),
                    black_box(&right),
                    black_box(&mut out),
                );
            });
        });
    }
}

fn verus_merge(c: &mut Criterion) {
    for size in ARRAY_SIZES {
        let (mut left, mut right) = generate_merge_task(size);
        left.append(&mut right);
        let all = left;

        let out = vec![0; size * 2];

        let arr =
            mergesort_benchmarks::merge_sorts::verus_without_ghost::exec_pcell::Array::new(all);
        let out =
            mergesort_benchmarks::merge_sorts::verus_without_ghost::exec_pcell::Array::new(out);

        c.bench_with_input(BenchmarkId::new("verus merge", size), &size, |b, _| {
            b.iter(|| {
                mergesort_benchmarks::merge_sorts::verus_without_ghost::mergesort::merge(
                    black_box(&arr),
                    black_box(0),
                    black_box(size),
                    black_box(size),
                    black_box(2 * size),
                    black_box(&out),
                    black_box(0),
                );
            });
        });
    }
}

static ARRAY_SIZES: [usize; 8] = [
    100_000,
    1_000_000,
    2_000_000,
    4_000_000,
    8_000_000,
    20_000_000,
    50_000_000,
    100_000_000,
];

fn small_config() -> Criterion {
    Criterion::default().sample_size(10)
    .measurement_time(Duration::from_secs(20))
}

criterion_group! {
    name = merge;
    config = small_config();
    targets = standard_merge, verus_merge
}
criterion_main!(merge);
