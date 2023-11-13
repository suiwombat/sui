// Note: most of our tests are doctests
use atomic_float::AtomicF32;
use core::sync::atomic::Ordering::*;

#[test]
fn readme_test() {
    static A_STATIC: AtomicF32 = AtomicF32::new(800.0);

    // Should support the full std::sync::atomic::AtomicFoo API
    A_STATIC.fetch_add(30.0, Relaxed);
    A_STATIC.fetch_sub(-55.0, Relaxed);

    // But also supports things that can be implemented
    // efficiently easily, like sign-bit operations.
    A_STATIC.fetch_neg(Relaxed);

    assert_eq!(A_STATIC.load(Relaxed), -885.0);
}
