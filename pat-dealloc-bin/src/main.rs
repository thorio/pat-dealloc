use crate::cli::Command;
use nix::unistd::Uid;
use std::{io, process::exit};

mod cli;

fn main() -> io::Result<()> {
	color_eyre::install().unwrap();

	let cli = cli::parse();
	init_logger(cli.debug);
	log::debug!("{:?}", cli);

	if !Uid::effective().is_root() {
		log::error!("running as non-root");
		exit(1);
	}

	match cli.command {
		Command::Raw { start, end } => pat_dealloc::free_memtype(start, end),
	}
}

fn init_logger(debug: bool) {
	stderrlog::new()
		.timestamp(stderrlog::Timestamp::Off)
		.verbosity(if debug { 3 } else { 2 })
		.init()
		.expect("logger already initialized");
}
