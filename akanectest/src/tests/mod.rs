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
fn add_nums() {
    unsafe {
        assert_eq!(ffi::add_nums(1, 1), 2);
        assert_eq!(ffi::add_nums(3, 5), 8);
    }
}

#[test]
fn double_add_nums() {
    unsafe {
        assert_eq!(ffi::double_add_nums(1, 1), 4);
        assert_eq!(ffi::double_add_nums(3, 5), 16);
    }
}

#[test]
fn add_nums_paren() {
    unsafe {
        assert_eq!(ffi::add_nums_paren(1, 1), 2);
        assert_eq!(ffi::add_nums_paren(3, 5), 8);
    }
}

#[test]
fn eval_add_one() {
    unsafe {
        assert_eq!(ffi::eval_add_one(1), 2);
        assert_eq!(ffi::eval_add_one(4), 5);
    }
}

#[test]
fn eval_add_nums() {
    unsafe {
        assert_eq!(ffi::eval_add_nums(1, 1), 2);
        assert_eq!(ffi::eval_add_nums(3, 5), 8);
    }
}

#[test]
fn mul_and_add() {
    unsafe {
        assert_eq!(ffi::mul_and_add(1, 1, 1, 1), 2);
        assert_eq!(ffi::mul_and_add(3, 5, 7, 9), 78);
    }
}

#[test]
fn mul_and_add_one() {
    unsafe {
        assert_eq!(ffi::mul_and_add_one(1, 1), 2);
        assert_eq!(ffi::mul_and_add_one(3, 5), 16);
    }
}
