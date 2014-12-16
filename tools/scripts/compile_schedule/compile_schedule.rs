extern crate getopts;

use getopts::{optopt,optflag,getopts,OptGroup};
use std::os;
use std::io;
use std::io::fs;
use std::io::fs::PathExtensions;

#[path = "./../compile_settings.rs"]
mod compile_settings;

/// Compiles the schedule by auto-linking all processes
fn main() {

    // Program args
    let mut is_child_script: bool = false;

    let args: Vec<String> = os::args();
    let opts = &[
        optflag("c", "child", "Run as a child compilation script: i.e. Don't recompile dependent modules and don't modify the .iscompiling file.")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("c") {
        is_child_script = true
    };

    // Lets compile!
    if !is_child_script {
        compile_settings::set_is_compiling(true);
    }

    let current_dir = os::self_exe_path().unwrap();

    let schedule_filename = current_dir.filename_str().unwrap().to_string() + ".rs";
    let target_path = current_dir.join("target");

    compile_settings::create_fresh_dir(&target_path);

    println!("Compiling schedule");

    let mut command = io::Command::new(compile_settings::get_rustc_path().as_str().unwrap());

    // Link common target dirs
    for common_target_dir in compile_settings::get_common_target_dirs().iter() {
        command.arg("-L");
        command.arg(common_target_dir.as_str().unwrap());
    }

    // Link process target dirs
    for process_target_dir in compile_settings::get_all_process_target_dirs().iter() {
        command.arg("-L");
        command.arg(process_target_dir.as_str().unwrap());
    }

    command.arg("--out-dir").arg("./target");
    command.arg("--crate-type=".to_string() + compile_settings::get_schedules_lib_type());
    command.arg("-C").arg("prefer-dynamic");
    command.arg(schedule_filename);

    compile_settings::execute_command(command);

    if !is_child_script {
        // Compile the scheduler
        compile_settings::run_external_application(&compile_settings::get_scheduler_src_dir().join(Path::new("compile")), Some(vec!["-c"]));
        compile_settings::set_is_compiling(false);
    }
}
