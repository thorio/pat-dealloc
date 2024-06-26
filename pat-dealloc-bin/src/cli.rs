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

	/// enable debug loglevel
	#[arg(long, global = true)]
	pub load: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
	/// Manually free a PAT memtype by specifiying bounds
	Raw {
		#[arg(long, value_parser=hex)]
		start: Address,

		#[arg(long, value_parser=hex)]
		end: Address,
	},
	/// Free all PAT memtypes within a PCI devices' resources
	Pci {
		/// PCI address in the form 0000:00:00.0
		#[arg(long)]
		address: String,
	},
}

pub fn hex(mut val: &str) -> Result<Address, String> {
	if val.starts_with("0x") {
		val = &val[2..];
	};

	Address::from_str_radix(val, HEX_RADIX).map_err(|e| format!("{e}"))
}
