// Look through all state struct files
// Generate state library source string
//  by referencing their modules
//  and calling their new()s
// Pass the string to the rust compiler
//  in an OS dependent way
//  outputting the binary to the target/state directory

use toml;
use worldsong_hierarchy;
use worldsong_config;
use system;

use std::io::{Read, Write};
use std::fs;
use std::path::{PathBuf, Path};

pub fn exec(app_dir: &Path) {
    let src_file_path = /*worldsong_hierarchy::get_temp_dir(&app_dir, "state").join("state.rs");*/generate_source(app_dir);

    //let config_display = system::get_compile_config(&mut command, &current_dir, &src_file_path, &target_dir);
    //println!("Compiling state"/*, config_display*/);

    let mut dep_dirs = Vec::new();
    // Link dependencies
    for path in worldsong_hierarchy::get_dependencies_all_target_dirs(app_dir).iter() {
        dep_dirs.push(path.clone())
    }

    system::rustc_compile_lib(app_dir, &dep_dirs, &src_file_path, worldsong_config::get_state_lib_type(), true);
}

fn generate_source(app_dir: &Path) -> PathBuf {
    println!("Generating the state library's source code.");
    let mut state_src_text = String::new();
    state_src_text.push_str("// ATTENTION: This file is automatically generated. How did you even get here? \n// You should probably modify compile_state.rs, the code that generated this file, instead.");
    state_src_text.push_str("\n\n");

    // Add the dependencies imports
    // TODO: Libraries may not have the same name as their folders. Parse the library names in these dirs, or parse the cargo.toml
    state_src_text.push_str("// Dependencies\n");

    // extract manifest from cargo.toml
    let toml_path = worldsong_hierarchy::get_dependencies_dir(app_dir).join("Cargo.toml");
    let mut toml_file = fs::File::open(&toml_path).unwrap();

    let mut toml_text = String::new();
    toml_file.read_to_string(&mut toml_text).unwrap();

    let value = parse(&toml_text, &toml_path);

    // extract dependency names from manifest
    match value.get("dependencies") {
        Some(d) => {
            if let Some(t) = d.as_table() {
                for (name,_) in t.iter() {
                    state_src_text.push_str("#[macro_use]");
                    state_src_text.push_str(&format!("extern crate {};\n", name));
                }
            }
        }
        None => {
            panic!("No dependencies listed in {}.", toml_path.display());
        }
    }

    state_src_text.push_str("// State structs\n");
    let structs_dir = worldsong_hierarchy::get_module_src_dir(app_dir, "state");

    // Get the file_names of the modules
    let mut file_names: Vec<String> = Vec::new();
    let mut type_names: Vec<String> = Vec::new();

    for file in worldsong_hierarchy::get_module_all_src_files(app_dir, "state").iter() {
        let name = file.file_stem().unwrap().to_str().unwrap().to_string();
        println!("Found state file: {}", &name);
        file_names.push(name.to_string().clone());

        type_names.push(to_camel_case(&name));
    }

    for i in 0 .. file_names.len() {
        println!("State is {}", file_names[i]);
        state_src_text.push_str(&format!("pub use {}::{};\n", &file_names[i], type_names[i]));
    }
    state_src_text.push_str("\n");

    for name in file_names.iter() {
        state_src_text.push_str(&format!("#[path = \"{structs_dir}/{struct_name}.rs\"]\n",
            structs_dir = structs_dir.to_str().unwrap(),
            struct_name = name));
        state_src_text.push_str(&format!("mod {};\n", name));
    }
    state_src_text.push_str("\n");

    // Add a data! macro that lists $name: $NameState = $NameState::new() for each name
    state_src_text.push_str("data! {\n");
    state_src_text.push_str("    Data {\n");

    for i in 0 .. file_names.len() {
        state_src_text.push_str(
            &format!("       {name}: {type_name} = {type_name}::new()\n",
            name = file_names[i],
            type_name = type_names[i],
            )
        );
    }

    state_src_text.push_str("    }\n");
    state_src_text.push_str("}\n");

    // save as state.rs
    // It's only used to generate the binary, so throw it in the temp dir.
    let state_src_dir = worldsong_hierarchy::get_temp_dir(&app_dir, "state");
    println!("State tmp dir is {:?}", state_src_dir);
    worldsong_hierarchy::create_fresh_dir(&state_src_dir).unwrap();
    let state_src_path = state_src_dir.join("state.rs");

    println!("Creating new state.rs");
    let mut state_src_file = worldsong_hierarchy::create_file_all(&state_src_path).unwrap();
    state_src_file.write_all(state_src_text.as_bytes()).unwrap();
    state_src_file.flush().unwrap();

    state_src_path
}
    /*
        [May 1, 2015] [04:07:52 ▴] <Kingsquee>  echo "fn main(){println!(\"hello world\");}" | rustc -
        [May 1, 2015] [04:07:56 ▴] <Kingsquee>  does this work on your shitty console
        [May 1, 2015] [04:09:02 ▴] <Kingsquee>  i.e. you get a rust_out
        [May 1, 2015] [04:09:51 ▴] <Kuraitou>   That doesn't
        [May 1, 2015] [04:09:55 ▴] <Kuraitou>   echo fn main(){println!("hello world");} | rustc -
        [May 1, 2015] [04:09:56 ▴] <Kuraitou>   this does
        [May 1, 2015] [04:10:09 ▴] <Kingsquee>  wow really
        [May 1, 2015] [04:10:15 ▴] <Kuraitou>   ya
        [May 1, 2015] [04:10:19 ▴] <Kingsquee>  weird
        [May 1, 2015] [04:10:22 ▴] <Kingsquee>  well at least the piping works
    */

fn to_camel_case(input: &str) -> String {
    let mut formatted = String::new();
    let mut capitalize_next = false;
    let mut first_letter = true;
    for character in input.chars() {
        if character == '_' {
            capitalize_next = true;
            continue
        }
        if capitalize_next == true || first_letter == true {
            formatted.push(character.to_uppercase().next().unwrap());
        } else {
            formatted.push(character);
        }
        capitalize_next = false;
        first_letter = false;
    }
    formatted
}

pub fn parse(toml: &str, file: &Path) -> toml::Table {
    let mut parser = toml::Parser::new(&toml);
    match parser.parse() {
        Some(toml) => return toml,
        None => {}
    }
    let mut error_str = format!("could not parse input TOML\n");
    for error in parser.errors.iter() {
        let (loline, locol) = parser.to_linecol(error.lo);
        let (hiline, hicol) = parser.to_linecol(error.hi);
        error_str.push_str(
            &format!("{}:{}:{}{} {}\n",
                file.display(),
                loline + 1,
                locol + 1,
                if loline != hiline || locol != hicol {
                    format!("-{}:{}", hiline + 1, hicol + 1)
                } else {
                    "".to_string()
                },
                error.desc
            )
        );
    }
    println!("{}", error_str);
    panic!();
}