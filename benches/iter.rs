#![feature(test)]

extern crate test;

use test::Bencher;

#[bench]
fn iterator(b: &mut Bencher) {
    let vec = vec![0; 10_000];
    b.iter(|| {
        assert!(vec.iter().all(|x| *x == 0));
    })
}

#[bench]
fn iterator_cloned(b: &mut Bencher) {
    let vec = vec![0; 10_000];
    b.iter(|| {
        assert!(vec.iter().cloned().all(|x| x == 0));
    })
}

#[bench]
fn slice(b: &mut Bencher) {
    let vec = vec![0; 10_000];
    b.iter(|| {
        let mut all_are_zeros = true;
        for v in vec.as_slice() {
            if *v != 0 {
                all_are_zeros = false;
                break;
            }
        }
        assert!(all_are_zeros);
    })
}
