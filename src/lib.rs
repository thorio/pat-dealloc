use std::fs::File;
use std::io::{self, Write};

pub fn free_memtype(start: u64, end: u64) -> io::Result<()> {
	let range = format_address_range(start, end);
	let mut file = File::create("/dev/pat_dealloc")?;

	// ignoring bytes written is ok, chardev will always eat whatever we give it
	file.write(range.as_bytes()).map(|_| ())
}

fn format_address_range(start: u64, end: u64) -> String {
	format!("{}-{}", format_address(start), format_address(end))
}

fn format_address(address: u64) -> String {
	format!("{address:#018x}")
}
