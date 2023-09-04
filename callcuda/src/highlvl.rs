use crate::{functions::mkptr, hvm_ffi::*};
use cudarc::driver::{CudaDevice, CudaSlice, DeviceRepr, DriverError};
use std::{slice, sync::Arc};

pub struct HlNet {
	pub root: Ptr,         // root wire
	pub blen: u32,         // total bag length (redex count)
	pub bags: Box<[Wire]>, // redex bags (active pairs)
	pub node: Box<[Node]>, // memory buffer with all nodes
	pub gidx: Box<[u32]>,  // aux buffer used on scatter functions
	pub gmov: Box<[Wire]>, // aux buffer used on scatter functions
	pub pbks: u32,         // last blocks count used
	pub done: u32,         // number of completed threads
	pub rwts: u32,         // number of rewrites performed
}

impl HlNet {
	pub fn new() -> Self {
		Self {
			root: mkptr(NIL, 0),
			blen: 0,
			bags: vec![Wire::default(); BAGS_SIZE as usize].into_boxed_slice(),
			node: vec![Node::default(); NODE_SIZE as usize].into_boxed_slice(),
			gidx: vec![0; GIDX_SIZE as usize].into_boxed_slice(),
			gmov: vec![Wire::default(); GMOV_SIZE as usize].into_boxed_slice(),
			pbks: 0,
			done: 0,
			rwts: 0,
		}
	}
}

pub fn net_to_device(net: &HlNet, dev: &Arc<CudaDevice>) -> Result<CudaSlice<Net>, DriverError> {
	// TODO: Async copy?
	let device_bags = dev.htod_sync_copy(&net.bags)?;
	let device_gidx = dev.htod_sync_copy(&net.gidx)?;
	let device_gmov = dev.htod_sync_copy(&net.gmov)?;
	let device_node = dev.htod_sync_copy(&net.node)?;

	let temp_net = Net {
		root: net.root,
		blen: net.blen,
		bags: (&device_bags).as_kernel_param() as _,
		node: (&device_node).as_kernel_param() as _,
		gidx: (&device_gidx).as_kernel_param() as _,
		gmov: (&device_gmov).as_kernel_param() as _,
		pbks: net.pbks,
		done: net.done,
		rwts: net.rwts,
	};

	let device_net = dev.htod_sync_copy(slice::from_ref(&temp_net))?;

	Ok(device_net)
}
