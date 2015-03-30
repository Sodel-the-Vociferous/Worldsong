#![feature(os)]
#![feature(old_io)]
#![feature(old_path)]
#![feature(old_fs)]

extern crate getopts;
extern crate common;

use getopts::Options;

use std::os;
use std::old_io;
use std::old_path::Path;
use std::old_path::GenericPath;

use common::hierarchy;
use common::system;
use common::settings;

/// Compiles the state lib, and everything else, wot.
fn main() {
    // Program args
    let mut should_update: bool = false;

    let args: Vec<String> = os::args();
    let mut opts = Options::new();
    opts.optflag("u", "update", "Update all state structs' dependencies before compiling.");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("u") {
        should_update = true
    };

    // Lets compile!
    hierarchy::set_is_compiling(true);

    let current_dir = os::self_exe_path().unwrap();
    let current_dir_name = current_dir.filename_str().unwrap();
    let target_path = current_dir.join("target");
    let source_filename = current_dir_name.to_string() + ".rs";

    println!("Compiling the State library");

    if should_update {
        let mut cargo_update_command = old_io::Command::new(hierarchy::get_cargo_path().as_str().unwrap());
        cargo_update_command.cwd(&hierarchy::get_state_src_dir());
        cargo_update_command.arg("update");
        system::execute_command(&mut cargo_update_command);
    }

    let mut cargo_build_command = old_io::Command::new(hierarchy::get_cargo_path().as_str().unwrap());
    cargo_build_command.cwd(&hierarchy::get_state_src_dir());
    cargo_build_command.arg("build");
    system::execute_command(&mut cargo_build_command);

    // Recompile everything

    // Recompile processes
    for path in hierarchy::get_all_process_src_dirs().iter_mut() {
        system::run(&path.join("compile"), Some(vec!["-c"]));
    }

    // Recompile schedules
    for path in hierarchy::get_all_schedule_src_dirs().iter_mut() {
        system::run(&path.join("compile"), Some(vec!["-c"]));
    }

    // Recompile the scheduler
    system::run(
        &hierarchy::get_scheduler_src_dir().join("compile"),
        Some(vec!["-c"])
    );

    // Recompile kernel
    system::run(
        &hierarchy::get_kernel_src_dir().join("compile"),
        Some(vec!["-c"])
    );

    hierarchy::set_is_compiling(false);
}