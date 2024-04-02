use crate::{Address, AddressRange, PatError};
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

const HEX_RADIX: u32 = 16;

lazy_static! {
	static ref PAT_REGEX: Regex = Regex::new(r"^PAT: \[mem 0x([\da-f]{16})-0x([\da-f]{16})\] ([\w-]+)$").unwrap();
}

#[derive(Debug)]
pub struct PatMemtype {
	pub range: AddressRange,
	pub cache_type: PatCacheType,
}

#[derive(Debug)]
pub enum PatCacheType {
	WriteBack,
	Uncached,
	WriteCombined,
	WriteThrough,
	UncachedMinus,
}

impl TryFrom<&str> for PatCacheType {
	type Error = PatError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"write-back" => Ok(PatCacheType::WriteBack),
			"uncached" => Ok(PatCacheType::Uncached),
			"write-combining" => Ok(PatCacheType::WriteCombined),
			"write-through" => Ok(PatCacheType::WriteThrough),
			"uncached-minus" => Ok(PatCacheType::UncachedMinus),
			_ => Err(PatError::Parsing(value.to_owned())),
		}
	}
}

pub fn get_relevant_pat_memtypes(valid_ranges: &[&AddressRange]) -> Result<Vec<PatMemtype>, PatError> {
	let memtypes = get_pat_memtypes()?
		.into_iter()
		.filter(|m| valid_ranges.iter().any(|v| v.contains(&m.range)))
		.collect();

	Ok(memtypes)
}

pub fn get_pat_memtypes() -> Result<Vec<PatMemtype>, PatError> {
	fs::read_to_string("/sys/kernel/debug/x86/pat_memtype_list")?
		.lines()
		.skip(1) // first line is a header
		.map(parse_pci_resource)
		.collect()
}

fn parse_pci_resource(line: &str) -> Result<PatMemtype, PatError> {
	let Some(captures) = PAT_REGEX.captures(line) else {
		return Err(PatError::Parsing(line.to_owned()));
	};

	let start = Address::from_str_radix(&captures[1], HEX_RADIX)?;
	let end = Address::from_str_radix(&captures[2], HEX_RADIX)?;
	let cache_type = PatCacheType::try_from(&captures[3])?;

	Ok(PatMemtype {
		range: AddressRange { start, end },
		cache_type,
	})
}
