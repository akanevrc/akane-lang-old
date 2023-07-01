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
        assert_eq!(ffi::addOne(1), 2);
        assert_eq!(ffi::addOne(4), 5);
    }
}

#[test]
fn add_nums() {
    unsafe {
        assert_eq!(ffi::addNums(1, 1), 2);
        assert_eq!(ffi::addNums(3, 5), 8);
    }
}

#[test]
fn double_add_nums() {
    unsafe {
        assert_eq!(ffi::doubleAddNums(1, 1), 4);
        assert_eq!(ffi::doubleAddNums(3, 5), 16);
    }
}

#[test]
fn add_nums_paren() {
    unsafe {
        assert_eq!(ffi::addNumsParen(1, 1), 2);
        assert_eq!(ffi::addNumsParen(3, 5), 8);
    }
}

#[test]
fn eval_add_one() {
    unsafe {
        assert_eq!(ffi::evalAddOne(1), 2);
        assert_eq!(ffi::evalAddOne(4), 5);
    }
}

#[test]
fn eval_add_nums() {
    unsafe {
        assert_eq!(ffi::evalAddNums(1, 1), 2);
        assert_eq!(ffi::evalAddNums(3, 5), 8);
    }
}

#[test]
fn mul_and_add() {
    unsafe {
        assert_eq!(ffi::mulAndAdd(1, 1, 1, 1), 2);
        assert_eq!(ffi::mulAndAdd(3, 5, 7, 9), 78);
    }
}

#[test]
fn mul_and_add_one() {
    unsafe {
        assert_eq!(ffi::mulAndAddOne(1, 1), 2);
        assert_eq!(ffi::mulAndAddOne(3, 5), 16);
    }
}
