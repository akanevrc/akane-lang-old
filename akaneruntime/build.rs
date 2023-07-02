use std::{
    env,
    process::Command,
    path::Path,
};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    if Path::new(&format!("{}/src/c/libakaneruntime.c", cargo_manifest_dir)).exists() {
        Command::new("clang-15")
        .args([
            "-I",
            &format!("{}/../lib/bdwgc/include", cargo_manifest_dir),
            "-c",
            "-o",
            &format!("{}/libakaneruntime.o", out_dir),
            &format!("{}/src/c/libakaneruntime.c", cargo_manifest_dir),
            &format!("-L{}/../lib/bdwgc/out", cargo_manifest_dir),
            "-lgc",
        ])
        .status()
        .or_else(|_|
            Command::new("clang")
            .args([
                "-I",
                &format!("{}/../lib/bdwgc/include", cargo_manifest_dir),
                "-c",
                "-o",
                &format!("{}/libakaneruntime.o", out_dir),
                &format!("{}/src/c/libakaneruntime.c", cargo_manifest_dir),
                &format!("-L{}/../lib/bdwgc/out", cargo_manifest_dir),
                "-lgc",
            ])
            .status()
        ).unwrap();
    }
    else {
        panic!("libakaneruntime.c not exists.");
    }
    if Path::new(&format!("{}/libakaneruntime.o", out_dir),).exists() {
        Command::new("ar")
        .args([
            "r",
            &format!("{}/../lib/libakaneruntime.a", cargo_manifest_dir),
            &format!("{}/libakaneruntime.o", out_dir),
        ])
        .status()
        .unwrap();
    }
    else {
        panic!("libakaneruntime.o not exists.");
    }

    println!("cargo:rerun-if-changed={}/src/c/libakaneruntime.c", cargo_manifest_dir);
}
