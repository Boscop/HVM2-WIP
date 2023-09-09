use crate::{functions::mkptr, hvm_ffi::*};
use cudarc::driver::{CudaDevice, CudaSlice, DeviceRepr, DriverError, DevicePtr};
use std::{slice, sync::Arc, mem};

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
	// TODO: Async copy? (Requires passing owned Vec)
	let device_bags = dev.htod_sync_copy(&net.bags)?;
	let device_node = dev.htod_sync_copy(&net.node)?;
	let device_gidx = dev.htod_sync_copy(&net.gidx)?;
	let device_gmov = dev.htod_sync_copy(&net.gmov)?;

	let temp_net = Net {
		root: net.root,
		blen: net.blen,
		bags: *(&device_bags).device_ptr() /* as _ */,
		node: *(&device_node).device_ptr() /* as _ */,
		gidx: *(&device_gidx).device_ptr() /* as _ */,
		gmov: *(&device_gmov).device_ptr() /* as _ */,
		pbks: net.pbks,
		done: net.done,
		rwts: net.rwts,
	};

	let device_net = dev.htod_sync_copy(slice::from_ref(&temp_net))?;

    // TODO: Keep these alive in the returned value
    mem::forget(device_bags);
    mem::forget(device_node);
    mem::forget(device_gidx);
    mem::forget(device_gmov);

	Ok(device_net)
}

pub fn net_to_host(dev: &Arc<CudaDevice>, device_net: CudaSlice<Net>) -> Result<HlNet, DriverError> {
	use cudarc::driver::{result, sys::CUdeviceptr};

	pub fn dtoh_sync_copy_into<T: DeviceRepr>(
		dev: &Arc<CudaDevice>,
		src: CUdeviceptr,
		src_len: usize,
		dst: &mut [T],
	) -> Result<(), DriverError> {
		assert_eq!(src_len, dst.len());
		dev.bind_to_thread()?;
		unsafe { result::memcpy_dtoh_sync(dst, src) }?;
		dev.synchronize()
	}

	pub fn dtoh_sync_copy<T: DeviceRepr>(
		dev: &Arc<CudaDevice>,
		src: CUdeviceptr,
		src_len: usize,
	) -> Result<Vec<T>, DriverError> {
		let mut dst = Vec::with_capacity(src_len);
		unsafe { dst.set_len(src_len) };
		dtoh_sync_copy_into(dev, src, src_len, &mut dst)?;
		Ok(dst)
	}

	// let mut net_vec: Vec<Net> = dev.sync_reclaim(device_net)?;
	let mut net_vec = dev.dtoh_sync_copy(&device_net)?;
	let net = net_vec.remove(0);

	// let bags = vec![Wire::default(); BAGS_SIZE as usize];
	// let node = vec![Node::default(); NODE_SIZE as usize];
	// let gidx = vec![0; GIDX_SIZE as usize];
	// let gmov = vec![Wire::default(); GMOV_SIZE as usize];

	let bags = dtoh_sync_copy(dev, net.bags /* as CUdeviceptr */, BAGS_SIZE as usize)?;
	let node = dtoh_sync_copy(dev, net.node /* as CUdeviceptr */, NODE_SIZE as usize)?;
	let gidx = dtoh_sync_copy(dev, net.gidx /* as CUdeviceptr */, GIDX_SIZE as usize)?;
	let gmov = dtoh_sync_copy(dev, net.gmov /* as CUdeviceptr */, GMOV_SIZE as usize)?;

	let net = HlNet {
		root: net.root,
		blen: net.blen,
		bags: bags.into_boxed_slice(),
		node: node.into_boxed_slice(),
		gidx: gidx.into_boxed_slice(),
		gmov: gmov.into_boxed_slice(),
		pbks: net.pbks,
		done: net.done,
		rwts: net.rwts,
	};
	Ok(net)
}
