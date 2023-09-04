use cudarc::driver::{DeviceRepr, ValidAsZeroBits, sys::CUdeviceptr};

// This code is initially optimized for nVidia RTX 4090
pub const BLOCK_LOG2: u32 = 8; // log2 of block size
pub const BLOCK_SIZE: u32 = 1 << BLOCK_LOG2; // threads per block
pub const UNIT_SIZE: u32 = 4; // threads per rewrite unit
pub const NODE_SIZE: u32 = 1 << 28; // max total nodes (2GB addressable)
pub const BAGS_SIZE: u32 = BLOCK_SIZE * BLOCK_SIZE * BLOCK_SIZE; // size of global redex bag
pub const GROUP_SIZE: u32 = BLOCK_SIZE * BLOCK_SIZE; // size os a group of bags
pub const GIDX_SIZE: u32 = BAGS_SIZE + GROUP_SIZE + BLOCK_SIZE; // aux object to hold scatter indices
pub const GMOV_SIZE: u32 = BAGS_SIZE; // aux object to hold scatter indices

// Types
// -----

// Pointer value (28-bit)
pub type Val = u32;

// Pointer tags (4-bit)
pub type Tag = u8;
pub const NIL: Tag = 0x0; // empty node
pub const REF: Tag = 0x1; // reference to a definition (closed net)
pub const NUM: Tag = 0x2; // unboxed number
pub const ERA: Tag = 0x3; // unboxed eraser
pub const VRR: Tag = 0x4; // variable pointing to root
pub const VR1: Tag = 0x5; // variable pointing to aux1 port of node
pub const VR2: Tag = 0x6; // variable pointing to aux2 port of node
pub const RDR: Tag = 0x7; // redirection to root
pub const RD1: Tag = 0x8; // redirection to aux1 port of node
pub const RD2: Tag = 0x9; // redirection to aux2 port of node
pub const CON: Tag = 0xA; // points to main port of con node
pub const DUP: Tag = 0xB; // points to main port of dup node
pub const TRI: Tag = 0xC; // points to main port of tri node
pub const QUA: Tag = 0xD; // points to main port of qua node
pub const QUI: Tag = 0xE; // points to main port of qui node
pub const SEX: Tag = 0xF; // points to main port of sex node
pub const NEO: u32 = 0xFFFFFFFD; // recently allocated value
pub const GON: u32 = 0xFFFFFFFE; // node has been moved to redex bag
pub const BSY: u32 = 0xFFFFFFFF; // value taken by another thread, will be replaced soon

// Rewrite fractions
pub const A1: u32 = 0;
pub const A2: u32 = 1;
pub const B1: u32 = 2;
pub const B2: u32 = 3;

// Ports (P1 or P2)
pub type Port = u8;
pub const P1: u32 = 0;
pub const P2: u32 = 1;

// Pointers = 4-bit tag + 28-bit val
pub type Ptr = u32;

// Nodes are pairs of pointers
#[repr(C, align(8))]
#[derive(Default, Copy, Clone)]
pub struct Node {
	pub ports: [Ptr; 2],
}

unsafe impl DeviceRepr for Node {}
unsafe impl ValidAsZeroBits for Node {}

// Wires are pairs of pointers
#[repr(C, align(8))]
#[derive(Default, Copy, Clone)]
pub struct Wire {
	pub lft: Ptr,
	pub rgt: Ptr,
}

unsafe impl DeviceRepr for Wire {}
unsafe impl ValidAsZeroBits for Wire {}

// An interaction net
#[repr(C)]
#[derive(Debug)]
pub struct Net {
	pub root: Ptr, // root wire
	pub blen: u32, // total bag length (redex count)

	// pub bags: *mut Wire, // redex bags (active pairs)
	// pub node: *mut Node, // memory buffer with all nodes
	// pub gidx: *mut u32,  // aux buffer used on scatter functions
	// pub gmov: *mut Wire, // aux buffer used on scatter functions

	pub bags: CUdeviceptr, // redex bags (active pairs)
	pub node: CUdeviceptr, // memory buffer with all nodes
	pub gidx: CUdeviceptr, // aux buffer used on scatter functions
	pub gmov: CUdeviceptr, // aux buffer used on scatter functions

	pub pbks: u32, // last blocks count used
	pub done: u32, // number of completed threads
	pub rwts: u32, // number of rewrites performed
}

unsafe impl DeviceRepr for Net {}
unsafe impl ValidAsZeroBits for Net {}

#[repr(C)]
pub struct Term {
	pub root: Ptr,
	pub alen: u32,
	pub acts: *mut Wire,
	pub nlen: u32,
	pub node: *mut Node,
	pub locs: *mut u32,
}

// A book
#[repr(C)]
pub struct Book {
	pub defs: *mut *mut Term,
}
