use clap::{Parser, Subcommand};
use pat_dealloc::Address;

const HEX_RADIX: u32 = 16;

pub fn parse() -> Args {
	Args::parse()
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
	#[command(subcommand)]
	pub command: Command,

	/// enable debug loglevel
	#[arg(long, global = true)]
	pub debug: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
	/// Manually free a PAT memtype by specifiying bounds
	Raw {
		#[arg(long, short, value_parser=hex)]
		start: Address,

		#[arg(long, short, value_parser=hex)]
		end: Address,
	},
	Pci {
		#[arg(long, short)]
		address: String,
	},
}

pub fn hex(mut val: &str) -> Result<Address, String> {
	if val.starts_with("0x") {
		val = &val[2..];
	};

	Address::from_str_radix(val, HEX_RADIX).map_err(|e| format!("{e}"))
}
