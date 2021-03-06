use std::env;
use std::io;
use std::fs;
use std::fs::{File, PathExt};
use std::path::{PathBuf, Path};

pub fn create_fresh_dir(path: &Path) -> io::Result<()> {
    match fs::remove_dir_all(path) {
        Ok(_) => (),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => (),
            _ => {
                return Err(e)
            }
        }
    };

    match fs::create_dir(path) {
        Ok(_) => (),
        Err(e) => match e.kind() {
            io::ErrorKind::AlreadyExists => (),
            _ => return Err(e),
        }
    };

    Ok(())
}

pub fn create_fresh_file(path: &Path) -> io::Result<File> {
    match fs::remove_file(path) {
        Ok(_) => /*println!("Removed file at {}", path.display())*/(),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => (),
            _ => {
                return Err(e)
            }
        }
    };

    File::create(path)
}

pub fn set_is_compiling(value: bool) -> io::Result<()> {
    match value {
        true => {
            File::create(&get_is_compiling_tag()).unwrap();
            Ok(())
        }
        false => {
            match fs::remove_file(&get_is_compiling_tag()) {
                Ok(o) => Ok(o),
                Err(e) => match e.kind() {
                    io::ErrorKind::NotFound => Ok(()),
                    _ => return Err(e),
                }
            }
        }
    }
}

// Worldsong Modules
lazy_static!{
    static ref WORLDSONG_ROOT_DIR: PathBuf = {

        let mut current_dir = env::current_exe().unwrap();
        current_dir.pop();

        let mut wsroot = None;
        'l: loop {
            for entry in fs::read_dir(&current_dir).unwrap() {
                let entry = entry.unwrap().path();
                if entry.is_file() && entry.file_name().unwrap() == ".wsroot" {
                    wsroot = Some(current_dir);
                    break 'l
                }
            }
            if !current_dir.pop() {
                break 'l
            }
        }

        match wsroot {
            Some(wsroot) => wsroot,
            None => panic!("ERROR: Could not find worldsong root. Was the .wsroot file removed?"),
        }
    };
}

pub fn get_worldsong_root_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.clone()
}

define_str!(TARGET_DIR, "/target");

#[cfg(debug_assertions)]
define_str!(CARGO_TARGET_DIR, "/target/debug");

#[cfg(not(debug_assertions))]
define_str!(CARGO_TARGET_DIR, "/target/release");

pub fn append_target_dir(path: &Path) -> PathBuf {
    path.join(TARGET_DIR)
}

// common
define_str!(COMMON_SRC_DIR, "common");
pub fn get_common_src_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(COMMON_SRC_DIR)
}

define_str!(COMMON_TARGET_DIR, COMMON_SRC_DIR!(), CARGO_TARGET_DIR!());
pub fn get_common_target_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(COMMON_TARGET_DIR)
}

// state
define_str!(STATE_SRC_DIR, "state");
pub fn get_state_src_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(STATE_SRC_DIR)
}

define_str!(STATE_TARGET_DIR, STATE_SRC_DIR!(), CARGO_TARGET_DIR!());
pub fn get_state_target_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(STATE_TARGET_DIR)
}

pub fn get_state_dependency_dirs() -> Vec<PathBuf> {
    let mut vec = Vec::new();
    vec.push(get_state_target_dir().join("deps"));
    vec.push(get_state_target_dir().join("native"));
    vec
}


// structs
define_str!(STRUCTS_DIR, "structs");
pub fn get_structs_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(STRUCTS_DIR)
}

pub fn get_all_struct_src_dirs() -> Vec<PathBuf> {
    let structs_dir = get_structs_dir();
    let mut dirs: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir(&structs_dir).unwrap() {
        let entry = entry.unwrap().path();
        if entry.is_dir() {
            dirs.push(entry.clone());
        }
    }
    dirs
}

// TODO: Remove this, it's useless now.
pub fn get_all_struct_target_dirs() -> Vec<PathBuf> {
    let mut dirs = get_all_struct_src_dirs();
    for entry in dirs.iter_mut() {
        entry.push("target")
    }
    dirs
}

pub fn get_all_struct_dep_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    for entry in get_all_struct_target_dirs().iter() {
        dirs.push(entry.clone().join("deps"));
        dirs.push(entry.clone().join("native"));
    }
    dirs
}


// kernel
define_str!(KERNEL_SRC_DIR, "kernel");
pub fn get_kernel_src_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(KERNEL_SRC_DIR)
}

define_str!(KERNEL_TARGET_DIR, KERNEL_SRC_DIR!(), TARGET_DIR!());
pub fn get_kernel_target_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(KERNEL_TARGET_DIR)
}

// scheduler
define_str!(SCHEDULER_SRC_DIR, "scheduler");

pub fn get_scheduler_src_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(SCHEDULER_SRC_DIR)
}

define_str!(SCHEDULER_TARGET_DIR, SCHEDULER_SRC_DIR!(), TARGET_DIR!());

pub fn get_scheduler_target_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(SCHEDULER_TARGET_DIR)
}

// schedules
define_str!(SCHEDULES_DIR, "schedules");
pub fn get_schedules_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(SCHEDULES_DIR)
}

pub fn get_all_schedule_src_dirs() -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(&get_schedules_dir()).unwrap() {
        let entry = entry.unwrap().path();
        if entry.is_dir() {
            dirs.push(entry.clone());
        }
    }
    dirs
}

pub fn get_all_schedule_target_dirs() -> Vec<PathBuf> {
    let mut dirs = get_all_schedule_src_dirs();
    for schedule_path in dirs.iter_mut() {
        //TODO: Replace this with a constant
        schedule_path.push("target")
    }
    dirs
}

// processes
define_str!(PROCESSES_DIR, "processes");
pub fn get_processes_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(PROCESSES_DIR)
}

pub fn get_all_process_src_dirs() -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(&get_processes_dir()).unwrap() {
        let entry = entry.unwrap().path();
        if entry.is_dir() {
            dirs.push(entry.clone());
        }
    }
    dirs
}

pub fn get_all_process_target_dirs() -> Vec<PathBuf> {
    let mut dirs = get_all_process_src_dirs();
    for entry in dirs.iter_mut() {
        //TODO: Replace this with a constant
        entry.push("target")
    }
    dirs
}

// Worldsong Tools

define_str!(RUSTC_PATH, "rustc");
pub fn get_rustc_path() -> PathBuf {
    let mut rustc_path = PathBuf::new();
    rustc_path.push(RUSTC_PATH);
    rustc_path
}

define_str!(CARGO_PATH, "cargo");
pub fn get_cargo_path() -> PathBuf {
    let mut cargo_path = PathBuf::new();
    cargo_path.push(CARGO_PATH);
    cargo_path
}

define_str!(TOOLS_DIR, "tools");
pub fn get_tools_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(TOOLS_DIR)
}

define_str!(RUN_KERNEL_TOOL_SRC_DIR, TOOLS_DIR!(), "/run_kernel");
define_str!(RUN_KERNEL_TOOL_TARGET_DIR, RUN_KERNEL_TOOL_SRC_DIR!(), CARGO_TARGET_DIR!());

pub fn get_run_kernel_tool_src_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(RUN_KERNEL_TOOL_SRC_DIR)
}

pub fn get_run_kernel_tool_target_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(RUN_KERNEL_TOOL_TARGET_DIR)
}

define_str!(NEW_STATE_STRUCT_TOOL_SRC_DIR, TOOLS_DIR!(), "/new_state_struct");
pub fn get_new_state_struct_tool_src_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(NEW_STATE_STRUCT_TOOL_SRC_DIR)
}

define_str!(NEW_STATE_STRUCT_TOOL_TARGET_DIR, NEW_STATE_STRUCT_TOOL_SRC_DIR!(), CARGO_TARGET_DIR!());
pub fn get_new_state_struct_tool_target_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(NEW_STATE_STRUCT_TOOL_TARGET_DIR)
}

define_str!(COMPILE_STATE_STRUCT_TOOL_SRC_DIR, TOOLS_DIR!(), "/compile_state_struct");
pub fn get_compile_state_struct_tool_src_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(COMPILE_STATE_STRUCT_TOOL_SRC_DIR)
}

define_str!(COMPILE_STATE_STRUCT_TOOL_TARGET_DIR, COMPILE_STATE_STRUCT_TOOL_SRC_DIR!(), CARGO_TARGET_DIR!());
pub fn get_compile_state_struct_tool_target_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(COMPILE_STATE_STRUCT_TOOL_TARGET_DIR)
}

define_str!(COMPILE_SCHEDULER_TOOL_SRC_DIR, TOOLS_DIR!(), "/compile_scheduler");
pub fn get_compile_scheduler_tool_src_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(COMPILE_SCHEDULER_TOOL_SRC_DIR)
}

define_str!(COMPILE_SCHEDULER_TOOL_TARGET_DIR, COMPILE_SCHEDULER_TOOL_SRC_DIR!(), CARGO_TARGET_DIR!());
pub fn get_compile_scheduler_tool_target_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(COMPILE_SCHEDULER_TOOL_TARGET_DIR)
}

define_str!(COMPILE_SCHEDULE_TOOL_SRC_DIR, TOOLS_DIR!(), "/compile_schedule");
pub fn get_compile_schedule_tool_src_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(COMPILE_SCHEDULE_TOOL_SRC_DIR)
}

define_str!(COMPILE_SCHEDULE_TOOL_TARGET_DIR, COMPILE_SCHEDULE_TOOL_SRC_DIR!(), CARGO_TARGET_DIR!());
pub fn get_compile_schedule_tool_target_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(COMPILE_SCHEDULE_TOOL_TARGET_DIR)
}

define_str!(COMPILE_PROCESS_TOOL_SRC_DIR, TOOLS_DIR!(), "/compile_process");
pub fn get_compile_process_tool_src_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(COMPILE_PROCESS_TOOL_SRC_DIR)
}

define_str!(COMPILE_PROCESS_TOOL_TARGET_DIR, COMPILE_PROCESS_TOOL_SRC_DIR!(), CARGO_TARGET_DIR!());
pub fn get_compile_process_tool_target_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(COMPILE_PROCESS_TOOL_TARGET_DIR)
}

define_str!(COMPILE_KERNEL_TOOL_SRC_DIR, TOOLS_DIR!(), "/compile_kernel");
pub fn get_compile_kernel_tool_src_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(COMPILE_KERNEL_TOOL_SRC_DIR)
}

define_str!(COMPILE_KERNEL_TOOL_TARGET_DIR, COMPILE_KERNEL_TOOL_SRC_DIR!(), CARGO_TARGET_DIR!());
pub fn get_compile_kernel_tool_target_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(COMPILE_KERNEL_TOOL_TARGET_DIR)
}

define_str!(ADD_PROCESS_TOOL_SRC_DIR, TOOLS_DIR!(), "/add_process");
pub fn get_add_process_tool_src_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(ADD_PROCESS_TOOL_SRC_DIR)
}

define_str!(ADD_PROCESS_TOOL_TARGET_DIR, ADD_PROCESS_TOOL_SRC_DIR!(), CARGO_TARGET_DIR!());
pub fn get_add_process_tool_target_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(ADD_PROCESS_TOOL_TARGET_DIR)
}

define_str!(ADD_STATE_STRUCT_TOOL_SRC_DIR, TOOLS_DIR!(), "/add_state_struct");
pub fn get_add_state_struct_tool_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(ADD_STATE_STRUCT_TOOL_SRC_DIR)
}

define_str!(ADD_STATE_STRUCT_TOOL_TARGET_DIR, ADD_STATE_STRUCT_TOOL_SRC_DIR!(), CARGO_TARGET_DIR!());
pub fn get_add_state_struct_tool_target_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(ADD_STATE_STRUCT_TOOL_TARGET_DIR)
}

define_str!(GENERATE_STATE_LIBRARY_TOOL_SRC_DIR, TOOLS_DIR!(), "/generate_state_library");
pub fn get_generate_state_library_tool_src_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(GENERATE_STATE_LIBRARY_TOOL_SRC_DIR)
}

define_str!(GENERATE_STATE_LIBRARY_TOOL_TARGET_DIR, GENERATE_STATE_LIBRARY_TOOL_SRC_DIR!(), CARGO_TARGET_DIR!());
pub fn get_generate_state_library_tool_target_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(GENERATE_STATE_LIBRARY_TOOL_TARGET_DIR)
}

define_str!(COMPILE_STATE_LIBRARY_TOOL_SRC_DIR, TOOLS_DIR!(), "/compile_state_library");
pub fn get_compile_state_library_tool_src_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(COMPILE_STATE_LIBRARY_TOOL_SRC_DIR)
}

define_str!(COMPILE_STATE_LIBRARY_TOOL_TARGET_DIR, COMPILE_STATE_LIBRARY_TOOL_SRC_DIR!(), CARGO_TARGET_DIR!());
pub fn get_compile_state_library_tool_target_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(COMPILE_STATE_LIBRARY_TOOL_TARGET_DIR)
}

define_str!(GENERATE_SCHEDULE_TAGS_SRC_DIR, TOOLS_DIR!(), "/generate_schedule_tags");
pub fn get_generate_schedule_tags_src_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(GENERATE_SCHEDULE_TAGS_SRC_DIR)
}

define_str!(GENERATE_SCHEDULE_TAGS_TARGET_DIR, GENERATE_SCHEDULE_TAGS_SRC_DIR!(), CARGO_TARGET_DIR!());
pub fn get_generate_schedule_tags_target_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(GENERATE_SCHEDULE_TAGS_TARGET_DIR)
}


// Worldsong Tags

pub fn get_schedule_tags(process_dir: &Path) -> PathBuf {
    process_dir.join(".schedule_tags")
}

pub fn get_generate_schedule_tags_binary() -> PathBuf {
    get_generate_schedule_tags_target_dir().join("generate_schedule_tags")
}

pub fn get_is_compiling_tag() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(".is_compiling")
}

pub fn get_compile_config(dir: &Path) -> PathBuf {
    dir.join("compile.config")
}
