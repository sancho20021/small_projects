pub const DEFAULT_THRESHOLD: usize = 2024;

// With this threshold I will spawn at lest cores number of threads
// for big enough arrays
// but not more than 2 * cores
pub fn get_threshold_with_cores(array_len: usize, cores: usize) -> usize {
    let mut tl = DEFAULT_THRESHOLD;
    let mut tr = std::cmp::max(DEFAULT_THRESHOLD + 1, array_len);
    while tr - tl > 1 {
        let tm = (tl + tr) / 2;
        let threads_est = get_est_threads_spawned(array_len, tm);
        if threads_est >= cores {
            tl = tm;
        } else {
            tr = tm;
        }
    }
    tl
}

pub fn get_threshold(array_len: usize) -> usize {
    get_threshold_with_cores(
        array_len,
        std::thread::available_parallelism().unwrap().into(),
    )
}

fn get_est_threads_spawned(array_len: usize, threshold: usize) -> usize {
    if array_len <= threshold {
        return 1;
    };
    let nt = (array_len as f64) / (threshold as f64);
    let log_nt = nt.log2().ceil() as usize;
    1 << log_nt
}

#[test]
fn expect_default() {
    let array_lens = [1, 2, 4, 10, 100, DEFAULT_THRESHOLD];
    for x in array_lens {
        assert_eq!(get_threshold(x), DEFAULT_THRESHOLD);
    }
}

#[test]
fn expect_grow_with_array_len() {
    let n1 = 10_000;
    let n2 = 100_000;
    assert!(get_threshold(n1) < get_threshold(n2));
}

#[test]
fn expect_decrease_with_cores() {
    let n = 100_000;
    let c1 = 10;
    let c2 = 20;
    assert!(get_threshold_with_cores(n, c1) > get_threshold_with_cores(n, c2));
}

#[test]
fn big_array() {
    let n = 100_000_000;
    let c = 8;
    let t = get_threshold_with_cores(n, c);
    let c_est = get_est_threads_spawned(n, t);
    println!("c_est = {c_est}, t = {t}");
    assert_eq!(c_est, c);
}

#[test]
fn test_est() {
    let n = 100_000_000;
    let t = n - 1;
    let c_est = get_est_threads_spawned(n, t);
    assert_eq!(c_est, 2);
}

#[test]
fn test_cpu24() {
    let n = 100_000_000;
    let c = 24;
    let t = get_threshold_with_cores(n, c);
    let c_est = get_est_threads_spawned(n, t);
    assert!(c_est >= c);
}
