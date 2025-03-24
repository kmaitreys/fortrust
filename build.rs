use std::path::Path;
use std::process::Command;

fn main() {
    let lib_dir = "lib";
    let fortran_source = format!("{}/math.f90", lib_dir);
    let output_lib = format!("{}/libmath.dylib", lib_dir);

    println!("cargo:rerun-if-changed={}", fortran_source);
    // println!("cargo:rustc-link-arg=-Wl,-no_warning_for_no_symbols");

    // Compile Fortran if source file has changed
    if !Path::new(&output_lib).exists()
        || Path::new(&fortran_source)
            .metadata()
            .unwrap()
            .modified()
            .unwrap()
            > Path::new(&output_lib)
                .metadata()
                .unwrap()
                .modified()
                .unwrap()
    {
        println!("Recompiling Fortran code...");

        let status = Command::new("gfortran")
            .args(["-shared", "-fPIC", "-o", &output_lib, &fortran_source])
            .status()
            .expect("Failed to compile Fortran");

        if !status.success() {
            panic!("Fortran compilation failed!");
        }
    }

    // Set library paths for linking
    println!("cargo:rustc-link-search=native={}", lib_dir);
    println!("cargo:rustc-link-lib=dylib=math");
}
