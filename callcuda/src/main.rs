#![allow(dead_code)]

mod functions;
mod highlvl;
mod hvm_ffi;

use cudarc::{driver::*, nvrtc::CompileOptions};
use highlvl::{net_to_device, HlNet};

fn main() -> Result<(), DriverError> {
	let dev = CudaDevice::new(0)?;

	let net = HlNet::new();
	let device_net = net_to_device(&net, &dev)?;

	// let ptx = cudarc::nvrtc::Ptx::from_file("../cuda/target/hvm2.ptx");

	let ptx = cudarc::nvrtc::compile_ptx_with_opts(include_str!("test.cu"), CompileOptions {
		include_paths: vec![],
		..Default::default()
	})
	.unwrap();
	dev.load_ptx(ptx, "module", &["test"])?;
	let f = dev.get_func("module", "test").unwrap();
	unsafe { f.launch(LaunchConfig::for_num_elems(1), (&device_net,)) }?;

	/*
	dev.load_ptx(ptx, "module", &["do_normalize"])?;
	let f = dev.get_func("module", "do_normalize").unwrap();
	unsafe { f.launch(LaunchConfig::for_num_elems(1), (&device_net,)) }?;
	 */
	Ok(())
}
