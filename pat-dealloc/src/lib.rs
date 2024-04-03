use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::num;
use std::path::{Path, PathBuf};
use thiserror::Error;

mod pat;
mod pci;

const DEVICE: &str = "/dev/pat_dealloc";

pub type Address = u64;

#[derive(Debug)]
pub(crate) struct AddressRange {
	pub start: Address,
	pub end: Address,
}

impl AddressRange {
	pub fn contains(&self, other: &Self) -> bool {
		self.start <= other.start && other.end <= self.end
	}
}

pub type PatResult<T> = Result<T, PatError>;

#[derive(Error, Debug)]
pub enum PatError {
	#[error("{0}")]
	Io(io::Error),
	#[error("{0}")]
	ParseInt(num::ParseIntError),
	#[error("unable to parse {0}")]
	Parsing(String),
	#[error("'{0}' is not available")]
	ChardevError(PathBuf),
}

impl From<io::Error> for PatError {
	fn from(value: io::Error) -> Self {
		Self::Io(value)
	}
}

impl From<num::ParseIntError> for PatError {
	fn from(value: num::ParseIntError) -> Self {
		Self::ParseInt(value)
	}
}

pub struct PatDealloc {
	file: File,
}

impl PatDealloc {
	pub fn new() -> PatResult<Self> {
		Self::new_from(DEVICE)
	}

	pub fn new_from(device: impl AsRef<Path>) -> PatResult<Self> {
		let file = OpenOptions::new()
			.write(true)
			.open(device.as_ref())
			.map_err(|_| PatError::ChardevError(device.as_ref().to_owned()))?;

		Ok(Self { file })
	}

	pub fn free_memtype(&mut self, start: Address, end: Address) -> PatResult<()> {
		let range = format_address_range(AddressRange { start, end });

		log::debug!("freeing {range}");

		// ignoring bytes written is ok, chardev will always eat whatever we feed it
		self.file.write(range.as_bytes()).map(|_| ())?;

		Ok(())
	}

	pub fn free_memtypes_for_pci(&mut self, address: &str) -> PatResult<()> {
		let resources = pci::get_relevant_pci_resources(address)?;
		log::debug!("resources: {resources:?}");
		let memtypes = pat::get_relevant_pat_memtypes(&resources.iter().map(|r| &r.range).collect::<Vec<_>>())?;
		log::debug!("memtypes: {memtypes:?}");

		for memtype in memtypes {
			self.free_memtype(memtype.range.start, memtype.range.end)?;
		}

		Ok(())
	}
}

fn format_address_range(range: AddressRange) -> String {
	format!("{}-{}", format_address(range.start), format_address(range.end))
}

fn format_address(address: Address) -> String {
	format!("{address:#018x}")
}
