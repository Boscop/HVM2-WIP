#![allow(dead_code)]

mod functions;
mod highlvl;
mod hvm_ffi;

use cudarc::driver::*;
use highlvl::{net_to_device, net_to_host, HlNet};

fn main() -> Result<(), DriverError> {
	let dev = CudaDevice::new(0)?;

	let mut net = HlNet::new();
	net.bags[0] = hvm_ffi::Wire {
		lft: 10,
		rgt: 20,
	};
	let device_net = net_to_device(&net, &dev)?;

	// let ptx = cudarc::nvrtc::Ptx::from_file("../cuda/target/hvm2.ptx");

	let ptx = cudarc::nvrtc::compile_ptx(include_str!("test.cu")).unwrap();
	dev.load_ptx(ptx, "module", &["test"])?;
	let f = dev.get_func("module", "test").unwrap();
	unsafe { f.launch(LaunchConfig::for_num_elems(1), (&device_net,)) }?;

	/*
	dev.load_ptx(ptx, "module", &["do_normalize"])?;
	let f = dev.get_func("module", "do_normalize").unwrap();
	unsafe { f.launch(LaunchConfig::for_num_elems(1), (&device_net,)) }?;
	*/

	let net = net_to_host(&dev, device_net)?;
	assert!(net.bags[0].lft == 10);
	assert!(net.bags[0].rgt == 20);

	Ok(())
}