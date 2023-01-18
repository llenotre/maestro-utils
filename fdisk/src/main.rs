//! `fdisk` is an utility command used to manipulate disk partition tables.
//!
//! The `sfdisk` is also implemented in the same program, it has the purpose as `fdisk`, except it
//! uses scripting instead of prompting.

mod disk;
mod partition;

use crate::partition::Partition;
use disk::Disk;
use partition::PartitionTableType;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use utils::prompt::prompt;

/// Structure storing command line arguments.
#[derive(Default)]
struct Args {
	/// The name of the current program used in command line.
	prog: String,
	/// Tells whether the command is run in scripting mode.
	script: bool,

	/// If true, print command line help.
	help: bool,

	/// If true, list partitions instead of modifying the table.
	list: bool,

	/// The list of disk devices.
	disks: Vec<PathBuf>,
}

impl Args {
	/// Tells whether arguments are valid.
	fn is_valid(&self) -> bool {
		if self.help || self.list {
			return true;
		}

		self.disks.len() == 1
	}
}

fn parse_args() -> Args {
	let mut args: Args = Default::default();

	let mut iter = env::args();
	args.prog = iter.next().unwrap_or("fdisk".to_owned());
	args.script = args.prog.split('/').last() == Some("sfdisk");

	while let Some(arg) = iter.next() {
		match arg.as_str() {
			"-h" | "--help" => args.help = true,
			"-l" | "--list" => args.list = true,

			// TODO implement other options

			_ => args.disks.push(arg.into()),
		}
	}

	args
}

/// Prints command usage.
///
/// `prog` is the name of the current program.
fn print_usage(prog: &str) {
	eprintln!("{}: bad usage", prog);
	eprintln!("Try '{} --help' for more information.", prog);
}

/// Prints command help.
///
/// - `prog` is the name of the current program.
/// - `script` tells whether the program is run as `sfdisk`.
fn print_help(prog: &str, script: bool) {
	println!();
	println!("Usage:");
	println!(" {} [options] [disks...]", prog);
	println!();
	println!("Prints the list of partitions or modify it.");
	println!();
	println!("Options:");
	println!(" -h, --help\tPrints help.");
	println!(" -l, --list\tLists partitions.");
}

/// Prints help for fdisk's internal commands.
fn print_cmd_help() {
	println!();
	println!("Help:");
	println!();
	println!("  DOS (MBR)");
	println!("   a  toggle a bootable flag");
	println!("   c  toggle the dos compatibility flag");
	println!();
	println!("  Generic");
	println!("   d  delete a partition");
	println!("   F  list free unpartitioned space");
	println!("   l  list known partition types");
	println!("   n  add a new partition");
	println!("   p  print the partition table");
	println!("   t  change a partition type");
	println!("   v  verify the partition table");
	println!("   i  print information about a partition");
	println!();
	println!("  Misc");
	println!("   m  print this menu");
	println!();
	println!("  Script");
	println!("   I  load disk layout from sfdisk script file");
	println!("   O  dump disk layout to sfdisk script file");
	println!();
	println!("  Save & Exit");
	println!("   w  write table to disk and exit");
	println!("   q  quit without saving changes");
	println!();
	println!("  Create a new label");
	println!("   g  create a new empty GPT partition table");
	println!("   o  create a new empty DOS partition table");
	println!();
}


fn main() {
	let args = parse_args();

	if !args.is_valid() {
		print_usage(&args.prog);
		exit(1);
	}
	if args.help {
		print_help(&args.prog, args.script);
		exit(0);
	}

	if args.list {
		let disks_count = args.disks.len();

		for (i, path) in args.disks.into_iter().enumerate() {
			match Disk::read(path.clone()) {
				Ok(Some(disk)) => print!("{}", disk),

				Ok(None) => {
					eprintln!("{}: cannot open {}: Invalid argument", args.prog, path.display());
				},

				Err(e) => {
					eprintln!("{}: cannot open {}: {}", args.prog, path.display(), e);
				},
			}

			if i + 1 < disks_count {
				println!("\n");
			}
		}

		return;
	}

	if !args.script {
		let mut disk = Disk::read(args.disks[0].clone())
			.unwrap() // TODO handle error
			.unwrap(); // TODO handle error
		let partition_table_type = PartitionTableType::MBR; // TODO get from disk

		while let Some(cmd) = prompt(Some("Command (m for help): "), false) {
			match cmd.as_str() {
				"a" => todo!(), // TODO

				"c" => todo!(), // TODO

				"d" => todo!(), // TODO

				"F" => todo!(), // TODO

				"l" => partition_table_type.print_partition_types(),

				"n" => todo!(), // TODO

				"p" => todo!(), // TODO

				"t" => todo!(), // TODO

				"v" => todo!(), // TODO

				"i" => todo!(), // TODO

				"m" => print_cmd_help(),

				"I" => if let Some(script_path) = prompt(Some("Enter script file name: "), false) {
					// TODO
					todo!();
				},

				"O" => if let Some(script_path) = prompt(Some("Enter script file name: "), false) {
					// TODO open the file as writable
					// TODO handle error
					let mut script_file = File::open(script_path).unwrap();

					let serialized = Partition::serialize(&args.disks[0], &disk.partitions);
					// TODO handle error
					script_file.write(serialized.as_bytes()).unwrap();

					println!("\nScript successfully saved.\n");
				},

				"w" => todo!(), // TODO

				"q" => todo!(), // TODO

				"g" => todo!(), // TODO

				"o" => todo!(), // TODO

				_ => eprintln!("{}: unknown command", cmd),
			}

			println!();
		}
		// TODO on exit without save, ask for confirm

		// TODO else on save, write table after confirm
	} else {
		// TODO Read and parse script
		// TODO Write partition table accordingly
		todo!();
	}
}
