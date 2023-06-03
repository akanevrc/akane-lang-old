mod ffi;

#[test]
fn one() {
    unsafe {
        assert_eq!(ffi::one(), 1);
    }
}

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
fn eval_add() {
    unsafe {
        assert_eq!(ffi::eval_add(1, 1), 2);
        assert_eq!(ffi::eval_add(3, 5), 8);
    }
}

#[test]
fn double() {
    unsafe {
        assert_eq!(ffi::double(1, 1), 4);
        assert_eq!(ffi::double(3, 5), 16);
    }
}

#[test]
fn add_paren() {
    unsafe {
        assert_eq!(ffi::eval_add(1, 1), 2);
        assert_eq!(ffi::eval_add(3, 5), 8);
    }
}
