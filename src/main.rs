extern crate libc;

use libc::c_int;

// Declare the Fortran function
#[link(name = "gfortran")]
unsafe extern "C" {
    fn add(a: *const c_int, b: *const c_int, result: *mut c_int);
}

fn main() {
    let a = 10;
    let b = 20;
    let mut result = 0;
    let j = [5.0; 10];

    unsafe {
        add(&a, &b, &mut result);
    }
    result *= j.iter().sum::<f64>() as i32;
    println!("{}", result);
}
