use std::os::raw::c_int;

#[link(name = "akanectest")]
extern "C" {
    pub fn add_one(a: c_int) -> c_int;
    pub fn add(a: c_int, b: c_int) -> c_int;
}
