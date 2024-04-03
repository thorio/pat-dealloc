use anyhow::{bail, Result};
use cli::Args;
use nix::unistd::Uid;
use pat_dealloc::{PatDealloc, PatResult};
use std::process::{exit, Command};

mod cli;

const MODULE_NAME: &str = "pat_dealloc";

fn main() {
	color_eyre::install().unwrap();

	let cli = cli::parse();
	init_logger(cli.debug);
	log::debug!("{:?}", cli);

	if !Uid::effective().is_root() {
		log::error!("running as non-root");
		exit(1);
	}

	if let Err(e) = run(cli) {
		log::error!("{e}");
	}
}

fn run(cli: Args) -> Result<()> {
	Ok(get_pat_dealloc(cli.load).map(move |p| handle_command(cli, p))??)
}

fn get_pat_dealloc(load: bool) -> Result<PatDealloc> {
	if load {
		log::debug!("loading kernel module");
		modprobe()?;
	}

	Ok(PatDealloc::new()?)
}

fn modprobe() -> Result<()> {
	let output = Command::new("modprobe").arg(MODULE_NAME).output()?;

	if output.status.success() {
		return Ok(());
	}

	let stderr = std::str::from_utf8(&output.stderr).unwrap_or("<conversion errror: stderr was not valid utf8>");
	bail!("modprobe invocation failure: {}", stderr.trim());
}

fn handle_command(cli: cli::Args, mut pat_dealloc: PatDealloc) -> PatResult<()> {
	match cli.command {
		cli::Command::Raw { start, end } => pat_dealloc.free_memtype(start, end),
		cli::Command::Pci { address } => pat_dealloc.free_memtypes_for_pci(&address),
	}
}

fn init_logger(debug: bool) {
	stderrlog::new()
		.timestamp(stderrlog::Timestamp::Off)
		.verbosity(if debug { 3 } else { 2 })
		.init()
		.expect("logger already initialized");
}
