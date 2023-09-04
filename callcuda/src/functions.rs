use crate::hvm_ffi::*;

// Creates a new pointer
pub fn mkptr(tag: Tag, val: Val) -> u32 {
	((tag as u32) << 28) | (val & 0x0FFFFFFF)
}
