use std::ffi::CStr;

#[repr(C)]
struct Auxv {
    a_type: u64,
    a_val: u64,
}

extern "C" {
    // The environment variables.
    static __environ: *mut *mut i8;
}

fn listauxval() {
    let mut environ = unsafe { __environ };

    println!("env address {:#x}", environ as u64);

    unsafe {
        // Skip past all the environment variables.
        while !(*environ).is_null() {
            println!("{}", CStr::from_ptr(*environ).to_str().unwrap());
            environ = environ.offset(1);
        }

        // Start of the auxiliary vector.
        let mut auxv = environ.offset(1) as *const Auxv;

        // Traverse the auxiliary vector until we find the desired type or reach the end.
        while (*auxv).a_type != 0 {
            println!("{:#x} = {:#x}", (*auxv).a_type, (*auxv).a_val);
            auxv = auxv.offset(1);
        }
    }
}

fn main() {
    println!("Hello, world!");

    listauxval();
}
