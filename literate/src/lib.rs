#[test]
fn test_present() {
    // i32
    assert_eq!(Some(0), bin_search(&0_i32, &[0, 1, 2, 4, 8, 16]));
    assert_eq!(Some(1), bin_search(&1_i32, &[0, 1, 2, 4, 8, 16]));
    assert_eq!(Some(2), bin_search(&2_i32, &[0, 1, 2, 4, 8, 16]));
    assert_eq!(Some(3), bin_search(&4_i32, &[0, 1, 2, 4, 8, 16]));
    assert_eq!(Some(4), bin_search(&8_i32, &[0, 1, 2, 4, 8, 16]));
    assert_eq!(Some(5), bin_search(&16_i32, &[0, 1, 2, 4, 8, 16]));

    // char
    assert_eq!(Some(0), bin_search(&'a', &['a', 'b', 'c']));
    assert_eq!(Some(1), bin_search(&'b', &['a', 'b', 'c']));
    assert_eq!(Some(2), bin_search(&'c', &['a', 'b', 'c']));

    // vector
    assert_eq!(Some(0), bin_search(&0.0, &vec![0.0, 1.0, 2.0]));
    assert_eq!(Some(1), bin_search(&1.0, &vec![0.0, 1.0, 2.0]));
    assert_eq!(Some(2), bin_search(&2.0, &vec![0.0, 1.0, 2.0]));
}

#[test]
fn test_absent() {
    assert_eq!(None, bin_search(&0, &[]));
    assert_eq!(None, bin_search(&0, &[1]));
    assert_eq!(None, bin_search(&2, &[1]));
    assert_eq!(None, bin_search(&0, &[1, 3]));
    assert_eq!(None, bin_search(&2, &[1, 3]));
    assert_eq!(None, bin_search(&5, &[1, 3]));

    assert_eq!(None, bin_search(&3.14, &[]));
    assert_eq!(None, bin_search(&3.14, &[0.0, 1.0, 3.0]));
}

pub fn bin_search<T: PartialOrd>(target: &T, collection: &[T]) -> Option<usize> {
    let mut lo: usize = 0;
    let mut hi: usize = collection.len();

    while lo < hi {
        let m: usize = (hi - lo) / 2 + lo;

        if *target == collection[m] {
            return Some(m);
        } else if *target < collection[m] {
            hi = m;
        } else {
            lo = m + 1;
        }
    }

    return None;
}
