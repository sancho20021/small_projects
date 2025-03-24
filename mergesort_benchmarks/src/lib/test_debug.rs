#[test]
fn test_exec_v2() {
    use disjoint_mut_test::disjoint_verified::exec_v2::{Array, Perms, Borrow};

    let (a, mut perms) = Array::new(vec![66, 77, 88]);

    let mut a0 = a.borrow(0, &mut perms);
    let x = a0.replace(666);
    println!("{x}");
}

#[test]
fn test_exec_pcell() {
    use disjoint_mut_test::disjoint_verified::exec_pcell::{Array, Perms};

    let (a, mut perms) = Array::new(vec![66, 77, 88]);

    let mut a0 = a.replace(0, 666, &mut perms);
    println!("{a0}");
}
