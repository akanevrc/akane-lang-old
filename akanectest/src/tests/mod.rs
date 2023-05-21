mod ffi;

#[test]
fn simple_add_one() {
    unsafe {
        assert_eq!(ffi::simple_add_one(1), 2);
        assert_eq!(ffi::simple_add_one(4), 5);
    }
}

#[test]
fn simple_add() {
    unsafe {
        assert_eq!(ffi::simple_add(1, 1), 2);
        assert_eq!(ffi::simple_add(3, 5), 8);
    }
}
