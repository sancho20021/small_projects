pub mod merge_sorts;
mod test_debug;

pub const DEFAULT_THRESHOLD: usize = 2024;

pub fn get_threshold_with_cores(array_len: usize, cores: usize) -> usize {
    if cores == 1 {
        return array_len;
    };
    let cf = cores as f64;
    let t = ((array_len as f64) * (cf.ln() - cf.ln().ln()) / cf) as usize;
    return std::cmp::max(t, DEFAULT_THRESHOLD);
}

pub fn get_threshold(array_len: usize) -> usize {
    get_threshold_with_cores(
        array_len,
        std::thread::available_parallelism().unwrap().into(),
    )
}

mod threshold_tests {
    use crate::{get_threshold, get_threshold_with_cores, DEFAULT_THRESHOLD};

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
}
