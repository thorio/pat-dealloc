use clap::{Parser, Subcommand};

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
		start: u64,

		#[arg(long, short, value_parser=hex)]
		end: u64,
	},
}

pub fn hex(mut val: &str) -> Result<u64, String> {
	if val.starts_with("0x") {
		val = &val[2..];
	};

	u64::from_str_radix(val, 16).map_err(|e| format!("{e}"))
}
