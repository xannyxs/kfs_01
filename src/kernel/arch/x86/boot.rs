// Boot Configuration (boot.rs)
//
// This module handles the Global Descriptor Table (GDT) initialization during
// the kernel's early boot process. The boot sequence follows these steps:
//
// 1. boot.asm: Initial assembly entry point
// 2. boot.rs:  GDT structure definition and initialization
// 3. gdt.asm:  CPU flush of GDT entries using LGDT instruction
//
// The GDT configuration uses a flat memory model with 4GB segments:
// - Null Descriptor: Required by CPU architecture (index 0)
// - Kernel Segments: Ring 0 code and data (indices 1-2)
// - User Segments:   Ring 3 code and data (indices 3-4)
//
// Segment Flags Overview:
// - Code (0b???1010): Execute/Read
// - Data (0b???0010): Read/Write
// - Ring 0 (0b10??): Kernel privilege
// - Ring 3 (0b11??): User privilege
// - Size bit (0b??1?): 32-bit protected mode
// - Granularity (0b???1): 4KB pages
//------------------------------------------------------------------------------

use super::{diagnostics::cpu::check_protection_status, gdt::GDTDescriptor};
use crate::arch::x86::gdt;

pub type GdtGates = [gdt::Gate; 5];

#[no_mangle]
pub static GDT_ENTRIES: GdtGates = [
	gdt::Gate(0), // [0] Null Descriptor (CPU requirement)
	#[cfg(target_arch = "x86")]
	gdt::Gate::new(0, !0, 0b10011010, 0b1100), /* [1] Kernel Code: Base 0,
	               * Limit max, Ring 0 */
	gdt::Gate::new(0, !0, 0b10010010, 0b1100), /* [2] Kernel Data: Base 0,
	                                            * Limit max, Ring 0 */
	gdt::Gate::new(0, !0, 0b11111010, 0b1100), /* [3] User Code: Base 0,
	                                            * Limit max, Ring 3 */
	gdt::Gate::new(0, !0, 0b11110010, 0b1100), /* [4] User Data: Base 0,
	                                            * Limit max, Ring 3 */
];

// Future expansion:
// - TSS (Task State Segment) entries will be needed for task switching
// gdt::Gate(0),  // TSS 1
// gdt::Gate(0),  // TSS 2

extern "C" {
	fn gdt_flush(gdt_ptr: *const GDTDescriptor);
}

#[no_mangle]
pub fn gdt_init() {
	let gdt_descriptor = GDTDescriptor {
		size: (core::mem::size_of::<GdtGates>() - 1) as u16,
		offset: &GDT_ENTRIES as *const _ as u32,
	};

	unsafe {
		gdt_flush(&gdt_descriptor as *const _);
	}

	check_protection_status();
}
