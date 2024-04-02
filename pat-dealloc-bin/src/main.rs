use crate::cli::Command;
use anyhow::Result;
use nix::unistd::Uid;
use pat_dealloc::{Address, PatDealloc};
use std::process::exit;

mod cli;

fn main() {
	color_eyre::install().unwrap();

	let cli = cli::parse();
	init_logger(cli.debug);
	log::debug!("{:?}", cli);

	if !Uid::effective().is_root() {
		log::error!("running as non-root");
		exit(1);
	}

	match cli.command {
		Command::Raw { start, end } => free_memtype(start, end).unwrap(),
		Command::Pci { address } => free_memtypes_for_pci(address).unwrap(),
	};
}

fn free_memtypes_for_pci(address: String) -> Result<(), anyhow::Error> {
	Ok(PatDealloc::new()?.free_memtypes_for_pci(&address)?)
}

fn free_memtype(start: Address, end: Address) -> Result<(), anyhow::Error> {
	Ok(PatDealloc::new()?.free_memtype(start, end)?)
}

fn init_logger(debug: bool) {
	stderrlog::new()
		.timestamp(stderrlog::Timestamp::Off)
		.verbosity(if debug { 3 } else { 2 })
		.init()
		.expect("logger already initialized");
}
