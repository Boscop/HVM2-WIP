use std::ffi::c_int;

#[link(name = "hvm2", kind = "static")]
extern "C" {
	fn lib_main() -> c_int;
}

fn main() {
	let ret = unsafe { lib_main() };
	std::process::exit(ret);
}
