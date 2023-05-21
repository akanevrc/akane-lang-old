use std::{
    env,
    process::Command,
    path::Path,
};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    if Path::new("./src/tests/akane/test.akane").exists() {
        akanec::compiler::compile(
            "./src/tests/akane/test.akane",
            &format!("{}/test.ll", out_dir)
        ).unwrap();
        Command::new("llc")
            .args([
                "--filetype",
                "obj",
                "-o",
                &format!("{}/test.o", out_dir),
                &format!("{}/test.ll", out_dir),
            ])
            .status()
            .unwrap();
        Command::new("ar")
            .args([
                "r",
                &format!("{}/libakanectest.a", out_dir),
                &format!("{}/test.o", out_dir),
            ])
            .status()
            .unwrap();
    }

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=akanectest");
    println!("cargo:rerun-if-changed={}/libakanectest.a", out_dir);
}
