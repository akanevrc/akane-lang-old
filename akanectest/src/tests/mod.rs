mod ffi;

#[test]
fn add_one() {
    unsafe {
        assert_eq!(ffi::add_one(1), 2);
        assert_eq!(ffi::add_one(4), 5);
    }
}

#[test]
fn add() {
    unsafe {
        assert_eq!(ffi::add(1, 1), 2);
        assert_eq!(ffi::add(3, 5), 8);
    }
}

#[test]
fn call_add() {
    unsafe {
        assert_eq!(ffi::call_add(1, 1, 1, 1), 4);
        assert_eq!(ffi::call_add(3, 5, 7, 9), 24);
    }
}
