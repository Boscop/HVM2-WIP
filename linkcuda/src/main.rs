use std::ffi::c_int;
// use libc::c_int;

#[link(name = "hvm2", kind = "static")]
extern "C" {
	fn lib_main() -> c_int;
}

fn main() {
	println!("{}", unsafe { lib_main() });
}
