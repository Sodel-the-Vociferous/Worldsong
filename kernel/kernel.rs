extern crate time;
extern crate common;

use std::dynamic_lib::DynamicLibrary;
use time::precise_time_ns;
use std::mem;
use std::io;
use std::io::File;
use std::io::BufferedReader;
use std::io::fs::PathExtensions;

use common::data::Data;

fn main() {
    /* //FIXME: new kernel role
    let mut data = Data::new();
    'main: loop {
        let cmd = scheduler.exec(&mut data);
        match cmd {
            //Reset           => { reload_data(); }
            ReloadScheduler => { reload_scheduler(); }
            Quit            => { break 'main }
        }
    }
    */
}

/*
 if data.kernel.load_processes {
     let mut attempts = 0u;
     while data.kernel.load_processes {
         match load_processes( &processes_dir, &mut paths, &mut libs, &mut fixed_update_symbols, &mut variable_update_symbols) {
             Some(()) => {
                 println!("Success!");
                 data.kernel.load_processes = false;
             }
             None => {
                 // Upon a library loading error, spin until console input.
                 attempts += 1;
                 motivate_on_error(attempts);
                 loop {
                     let line = io::stdin().read_line();
                     if line.is_ok() {
                         break
                     }
                 }
             }
         }
     }
 }
 */

fn motivate_on_error(attempts: uint) {
    match attempts {
        1 =>    {
            println!("")
            println!("Wups, that wasn't right! Halting process execution!")
            println!("Try fixing the error, and press [Enter] to reload processes.")
        }
        2 =>    {
            println!("")
            println!("Nope, nice try though! Halting execution!!")
            println!("Give it another shot and hit [Enter] when ready.")
        }
        3 =>    {
            println!("")
            println!("That didn't work either. Halting execution.")
            println!("Take a deep breath and press [Enter] to try again.")
        }
        4 =>    {
            println!("")
            println!("Nope, Execution Halted. Now might be a good time to get some tea.")
            println!("The [Enter] key doesn't need to be pushed quite yet.")
        }
        5...10 =>   {
            println!("")
            println!("Take another sip.")
        }
        11 =>   {
            println!("")
            println!("...o.o")
        }
        _ =>    {
            println!("")
            println!("BAKA. Ganbatte! ov o7 ")
        }
    }
}

fn load_processes(
    processes_dir:              &Path,
    paths:                      &mut Vec<Path>,
    libs:                       &mut Vec<DynamicLibrary>,
    fixed_update_symbols:       &mut Vec<fn(&mut Data) ->()>,
    variable_update_symbols:    &mut Vec<fn(&mut Data) ->()>) -> Option<()>
{
    println!("Loading processes")
    paths                   .clear();
    libs                    .clear();
    fixed_update_symbols    .clear();
    variable_update_symbols .clear();

    *paths  = match load_paths(processes_dir) {
        Some(paths) =>  { paths }
        None        =>  { return None }
    };

    *libs   = match load_libs(paths) {
        Some(libs)  =>  { libs }
        None        =>  { return None }
    };

    *fixed_update_symbols    = load_symbols("fixed_update",     libs);
    *variable_update_symbols = load_symbols("variable_update",  libs);

    Some(())
}

fn load_paths(processes_dir: &Path) -> Option<Vec<Path>> {
    let process_schedule_path = processes_dir.clone().join("process.schedule");
    let mut process_schedule = BufferedReader::new(
        match File::open(&process_schedule_path) {
            Err(why) => {
                println!("");
                println!("ERROR: Failed to load the schedule, was it accidentally moved, or its filename misspelled?");
                println!("The failure states that it {}", why);
                return None
            }
            Ok(value) => {
                value
            }
        }
    );

    let mut process_paths: Vec<Path> = Vec::new();

    for line in process_schedule.lines() {
        let mut name : String = line.unwrap();
        name.pop();
        //name; // FIXME: Remove empty lines and newline char when necessary.
        let process_binary_path: Path = processes_dir.clone().join_many(&[name.clone(), "target".to_string(), "lib".to_string() + name.clone() + ".so".to_string()]);
        if process_binary_path.exists() && process_binary_path.is_file() {
            process_paths.push(process_binary_path);
        } else {
            println!("ERROR: Could not load any process named {}, did you misspell its entry in the schedule, or forget to add a newline at the end of the file?", name.clone());
            return None
        }
    }
    Some(process_paths)
}

fn load_libs(paths: &Vec<Path>) -> Option<Vec<DynamicLibrary>> {
    let mut vec = Vec::new();
    for path in paths.iter() {
        vec.push (
            match DynamicLibrary::open(Some(path)) {
                Err (why)       => {
                    println!("");
                    println!("ERROR: Failed to load process binaries. Either you tried reloading while it was compiling, or you're doing something terrible.");
                    println!("The failure states that it {}", why);
                    return None     }
                    Ok  (binary)    => { binary }
            }
        );
    }
    Some(vec)
}

fn load_symbols(name: &str, libs: &Vec<DynamicLibrary>) -> Vec<fn(&mut Data) -> ()> {
    let mut vec = Vec::new();
    for lib in libs.iter() {
        vec.push(
            unsafe {
                match lib.symbol::<()>(name) {
                    Err (why)   => { /* println! ("{}", why); */ continue}
                    Ok  (func)  => { mem::transmute(func) }
                }
            }
        );
    }
    vec
}

#[inline(always)]
fn execute_symbols(symbols: &Vec<fn(&mut Data) ->()>, data: &mut Data) {
    for symbol in symbols.iter() {
        (*symbol)(data);
    }
}
