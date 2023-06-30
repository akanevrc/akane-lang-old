use std::os::raw::c_int;

#[link(name = "akanectest")]
extern "C" {
    pub fn one() -> c_int;
    pub fn add_one(a: c_int) -> c_int;
    pub fn add_nums(a: c_int, b: c_int) -> c_int;
    pub fn double_add_nums(a: c_int, b: c_int) -> c_int;
    pub fn add_nums_paren(a: c_int, b: c_int) -> c_int;
    pub fn eval_add_one(a: c_int) -> c_int;
    pub fn eval_add_nums(a: c_int, b: c_int) -> c_int;
    pub fn mul_and_add(a: c_int, b: c_int, c: c_int, d: c_int) -> c_int;
    pub fn mul_and_add_one(a: c_int, b: c_int) -> c_int;
}
