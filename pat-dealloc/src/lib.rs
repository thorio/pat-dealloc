use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

const DEVICE: &str = "/dev/pat_dealloc";

pub struct PatDealloc {
	file: File,
}

impl PatDealloc {
	pub fn new() -> io::Result<Self> {
		Self::new_from(DEVICE)
	}

	pub fn new_from(device: impl AsRef<Path>) -> io::Result<Self> {
		let file = OpenOptions::new().write(true).open(device)?;

		Ok(Self { file })
	}

	pub fn free_memtype(&mut self, start: u64, end: u64) -> io::Result<()> {
		let range = format_address_range(start, end);

		// ignoring bytes written is ok, chardev will always eat whatever we feed it
		self.file.write(range.as_bytes()).map(|_| ())
	}
}

fn format_address_range(start: u64, end: u64) -> String {
	format!("{}-{}", format_address(start), format_address(end))
}

fn format_address(address: u64) -> String {
	format!("{address:#018x}")
}
