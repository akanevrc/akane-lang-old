use std::os::raw::c_int;

#[link(name = "akanectest")]
extern "C" {
    pub fn one() -> c_int;
    pub fn addOne(a: c_int) -> c_int;
    pub fn addNums(a: c_int, b: c_int) -> c_int;
    pub fn doubleAddNums(a: c_int, b: c_int) -> c_int;
    pub fn addNumsParen(a: c_int, b: c_int) -> c_int;
    pub fn evalAddOne(a: c_int) -> c_int;
    pub fn evalAddNums(a: c_int, b: c_int) -> c_int;
    pub fn mulAndAdd(a: c_int, b: c_int, c: c_int, d: c_int) -> c_int;
    pub fn mulAndAddOne(a: c_int, b: c_int) -> c_int;
}
