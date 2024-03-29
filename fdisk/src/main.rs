//! `fdisk` is an utility command used to manipulate disk partition tables.
//!
//! The `sfdisk` is also implemented in the same program, it has the purpose as `fdisk`, except it
//! uses scripting instead of prompting.

#![feature(exclusive_range_pattern)]
#![feature(iter_array_chunks)]

mod crc32;
mod disk;
mod guid;
mod partition;

use crate::partition::PartitionTable;
use disk::Disk;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
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
    eprintln!("{prog}: bad usage");
    eprintln!("Try '{prog} --help' for more information.");
}

/// Prints command help.
///
/// - `prog` is the name of the current program.
/// - `script` tells whether the program is run as `sfdisk`.
fn print_help(prog: &str, script: bool) {
    println!();
    println!("Usage:");
    println!(" {prog} [options] [disks...]");
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

/// Imports the script in the file at the given path and applies it to the given disk.
fn import_script(disk: &mut Disk, path: &Path) -> io::Result<()> {
    let script = fs::read_to_string(path)?;
    // TODO handle error
    disk.partition_table = PartitionTable::from_str(&script).unwrap();
    Ok(())
}

/// Exports the given disk as a script to the file at the given path.
fn export_script(disk: &Disk, path: &Path) -> io::Result<()> {
    let mut script_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;
    let serialized = disk.partition_table.serialize(path);
    script_file.write_all(serialized.as_bytes())?;
    script_file.flush()?;
    Ok(())
}

/// TODO doc
///
/// If modifications have been made, the function returns `true`.
fn handle_cmd(cmd: &str, disk_path: &Path, disk: &mut Disk) {
    match cmd {
        "a" => {
            // TODO check whether partition table type supports boot flag. If not, error
            // TODO select partition and toggle boot flag
        }

        "d" => {
            // TODO:
            // - If only one partition is present, use it and print `Selected partition 1`
            // - Else, prompt `Partition number (1,2,... default <>): `
            // - Delete partition and print `Partition <> has been deleted.`
        }

        "F" => todo!(), // TODO

        "l" => disk.partition_table.table_type.print_partition_types(),

        "n" => {
            let _new_partition = disk.partition_table.table_type.prompt_new_partition();
            // TODO insert new partition to disk
        }

        "p" => println!("{disk}\n"),

        "t" => {
            // TODO:
            // - If only one partition is present, use it and print `Selected partition 1`
            // - Else, prompt `Partition number (1,2,... default <>): `
            // - Prompt `Partition type (type L to list all): `
            //   - On L: partition_table_type.print_partition_types()
            // - Change type and print `Changed type of partition `<old>` to `<new>``
        }

        "v" => todo!(), // TODO

        "i" => todo!(), // TODO

        "m" => print_cmd_help(),

        "I" => {
            if let Some(script_path) = prompt(Some("Enter script file name: "), false) {
                let script_path = PathBuf::from(script_path);

                match import_script(disk, &script_path) {
                    Ok(_) => println!("\nScript successfully applied.\n"),
                    Err(e) => eprintln!("cannot import script {}: {e}", script_path.display()),
                }
            }
        }

        "O" => {
            if let Some(script_path) = prompt(Some("Enter script file name: "), false) {
                let script_path = PathBuf::from(script_path);

                match export_script(disk, &script_path) {
                    Ok(_) => println!("\nScript successfully saved.\n"),
                    Err(e) => eprintln!("cannot export script {}: {e}", script_path.display()),
                }
            }
        }

        "w" => {
            // TODO ask only if modifications have been made
            let prompt_str = format!("Write changes to `{}`? (y/n) ", disk_path.display());
            let confirm = prompt(Some(&prompt_str), false)
                .map(|s| s == "y")
                .unwrap_or(false);
            if !confirm {
                return;
            }

            match disk.write() {
                Ok(_) => println!("The partition table has been altered."),
                Err(e) => {
                    eprintln!("cannot write to disk `{}`: {e}", disk_path.display());
                    exit(1);
                }
            }

            match disk::read_partitions(disk.get_path()) {
                Ok(_) => println!("Syncing disks."),
                Err(e) => {
                    eprintln!(
                        "cannot read partition table from `{}`: {e}",
                        disk_path.display()
                    );
                    exit(1);
                }
            }

            exit(0);
        }

        "q" => exit(0),

        "g" => {
            // TODO:
            // - Remove all partitions
            // - If the partition table type is not the same:
            //   - Change partition table type
            //   - Print `Created a new GPT disklabel (GUID: <>)\n`
        }

        "o" => {
            // TODO:
            // - Remove all partitions
            // - If the partition table type is not the same:
            //   - Change partition table type
            //   - Print `Created a new DOS disklabel (identifier: <>)\n`
        }

        _ => eprintln!("{cmd}: unknown command"),
    }

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
        let iter = if !args.disks.is_empty() {
            args.disks.into_iter()
        } else {
            match Disk::list() {
                Ok(disks) => disks.into_iter(),

                Err(e) => {
                    eprintln!("{}: cannot list disks: {e}", args.prog);
                    exit(1);
                }
            }
        };

        for path in iter {
            match Disk::read(path.clone()) {
                Ok(Some(disk)) => print!("{}", disk),

                Ok(None) => {
                    eprintln!(
                        "{}: cannot open {}: Invalid argument",
                        args.prog,
                        path.display()
                    );
                }

                Err(e) => {
                    eprintln!("{}: cannot open {}: {e}", args.prog, path.display());
                }
            }
        }

        return;
    }

    let disk_path = &args.disks[0];

    if !args.script {
        let mut disk = Disk::read(disk_path.clone())
            .unwrap() // TODO handle error
            .unwrap(); // TODO handle error

        while let Some(cmd) = prompt(Some("Command (m for help): "), false) {
            handle_cmd(&cmd, disk_path, &mut disk);
        }
    } else {
        // TODO Read and parse script
        // TODO Write partition table accordingly
        todo!();
    }
}
