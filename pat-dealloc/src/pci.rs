use crate::{Address, AddressRange, PatError};
use std::fs;

const HEX_RADIX: u32 = 16;
const IORESOURCE_MEM_64: u64 = 0x00100000;

#[derive(Debug)]
pub struct PciResource {
	pub range: AddressRange,
	pub flags: u64,
}

pub fn get_relevant_pci_resources(address: &str) -> Result<Vec<PciResource>, PatError> {
	let resources = get_pci_resources(address)?
		.into_iter()
		.filter(|r| r.range.start != 0 && r.flags & IORESOURCE_MEM_64 > 0)
		.collect();

	Ok(resources)
}

pub fn get_pci_resources(address: &str) -> Result<Vec<PciResource>, PatError> {
	fs::read_to_string(format!("/sys/bus/pci/devices/{address}/resource"))?
		.lines()
		.map(parse_pci_resource)
		.collect::<Result<Vec<_>, _>>()
}

fn parse_pci_resource(line: &str) -> Result<PciResource, PatError> {
	let [start, end, flags] = line.split_whitespace().collect::<Vec<_>>()[..] else {
		return Err(PatError::Parsing(line.to_owned()));
	};

	let range = AddressRange {
		start: Address::from_str_radix(&start[2..], HEX_RADIX)?,
		end: Address::from_str_radix(&end[2..], HEX_RADIX)? + 1,
	};

	let resource = PciResource {
		range,
		flags: u64::from_str_radix(&flags[2..], HEX_RADIX)?,
	};

	Ok(resource)
}
