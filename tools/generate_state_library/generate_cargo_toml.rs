//use rustc_serialize::{Decodable, Decoder};
use std::path::{PathBuf, Path};
use std::fs::{File, PathExt};
use std::io::{Read, Write};
use std::collections::HashMap;
use toml;

use common::hierarchy;

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct TomlManifest {
    dependencies: Option<HashMap<String, TomlDependency>>,
}

#[derive(RustcDecodable, RustcEncodable, Clone, Debug)]
pub enum TomlDependency {
    Simple(String),
    Detailed(DetailedTomlDependency)
}

#[derive(RustcDecodable, RustcEncodable, Clone, Debug, Default)]
pub struct DetailedTomlDependency {
    version: Option<String>,
    path: Option<String>,
    git: Option<String>,
    branch: Option<String>,
    tag: Option<String>,
    rev: Option<String>,
    features: Option<Vec<String>>,
    optional: Option<bool>,
    default_features: Option<bool>,
}

struct Conflict {
    name: String,
    manifest_a: String,
    manifest_b: String,
    version_a: String,
    version_b: String,
}

pub fn exec(struct_src_dirs: &Vec<PathBuf>) {
    println!("Generating the state's Cargo.toml.");

    let mut src_dirs = struct_src_dirs.clone();
    src_dirs.push(hierarchy::get_kernel_src_dir());
    src_dirs.push(hierarchy::get_scheduler_src_dir());

    // Get the Dependency.tomls
    let mut dependency_tomls: Vec<PathBuf> = Vec::with_capacity(src_dirs.len());

    for dir in src_dirs.iter() {
        let dependencies_toml = dir.clone().join("Dependencies.toml");

        if dependencies_toml.exists() && dependencies_toml.is_file() {
            dependency_tomls.push(dependencies_toml)
        }
    }

    let mut toml_manifests = HashMap::new();

    // Extract the manifests from the tomls
    for toml_path in dependency_tomls.iter() {
        let mut toml_file = File::open(toml_path).unwrap();

        let mut toml_text = String::new();
        toml_file.read_to_string(&mut toml_text).unwrap();

        let root = parse(&toml_text, toml_path);
        let toml_manifest: TomlManifest = toml::decode(toml::Value::Table(root)).unwrap();
        /*let mut decoder = toml::Decoder::new(toml::Value::Table(root));
        let toml_manifest: TomlManifest = match Decodable::decode(&mut decoder) {
            Ok(t) => t,
            Err(e) => panic!(
                format!("{} is not a valid manifest\n\n{}",
                    toml_path.file_name().unwrap(), e)
            ),
        };*/

        let dir_name = toml_path.parent().unwrap().file_name().unwrap().to_str().unwrap().to_string();
        toml_manifests.insert(dir_name,toml_manifest);
    }

    let mut conflicts = Vec::new();

    // Wow, this is ugly!
    // I should probably feel ashamed of myself!
    'a_manifest: for (manifest_name_a, manifest_a) in toml_manifests.iter() {
        if manifest_a.dependencies.is_none() { continue }
        'a_dep: for (dep_name_a, dep_a) in manifest_a.dependencies.as_ref().unwrap().iter() {
            'b_manifest: for (manifest_name_b, manifest_b) in toml_manifests.iter() {
                if manifest_b.dependencies.is_none() { continue }
                'b_dep: for (dep_name_b, dep_b) in manifest_b.dependencies.as_ref().unwrap().iter() {
                    if manifest_name_a == manifest_name_b {
                        continue 'a_manifest
                    }
                    if dep_name_a == dep_name_b {
                        let version_a = get_version(dep_a);
                        let version_b = get_version(dep_b);

                        if version_a != version_b {
                            conflicts.push(
                                Conflict {
                                    name: dep_name_a.clone(),
                                    manifest_a: manifest_name_a.clone(),
                                    manifest_b: manifest_name_b.clone(),
                                    version_a: version_a.clone(),
                                    version_b: version_b.clone(),
                                }
                            );
                        }
                    }
                }
            }
        }
    }

    // Fix the hashmap ordering
    for conflict in conflicts.iter_mut() {
        if conflict.manifest_a > conflict.manifest_b {

            let tmp = conflict.manifest_a.clone();
            conflict.manifest_a = conflict.manifest_b.clone();
            conflict.manifest_b = tmp.clone();

            let tmp = conflict.version_a.clone();
            conflict.version_a = conflict.version_b.clone();
            conflict.version_b = tmp.clone();
        }
    }
    conflicts.sort_by(|a, b|
        a.manifest_a.cmp(&b.manifest_a)
    );

    for conflict in conflicts.iter() {
        // Display the conflict to the operator
        println!(
            "ERROR: Version mismatch: {} {}, while {} {}.",
            format!("\"{}\"", conflict.manifest_a),

            if conflict.version_a == "".to_string() {
                format!("uses the latest available version of {}", conflict.name)
            } else {
                format!("requires {} of {}", conflict.version_a, conflict.name)
            },

            format!("\"{}\"", conflict.manifest_b),

            if conflict.version_b == "".to_string() {
                format!("uses the latest available version of {}", conflict.name)
            } else {
                format!("requires {} of {}", conflict.version_b, conflict.name)
            }
        );
    }

    if conflicts.len() != 0 {
        println!("You may be able to resolve these conflicts by modifying their respective \"Dependencies.toml\" files.");
        panic!();
    }

    let mut final_manifest = TomlManifest { dependencies: Some(HashMap::new()) };
    for (_, manifest) in toml_manifests.iter() {
        if manifest.dependencies.is_none() {
            continue
        }
        for (dep_name, dep) in manifest.dependencies.as_ref().unwrap().iter() {
            final_manifest.dependencies.as_mut().unwrap().insert(dep_name.clone(), dep.clone());
        }
    }

    let cargo_toml_text =
format!("#ATTENTION: This file is automatically generated. Don't modify it unless your life is terrible, or you wish it to be so.
[package]
name = \"state\"
version = \"0.0.1\"
authors = [ \"the ghost in the machine\" ]

[lib]
name = \"state\"
path = \"state.rs\"
crate_type = [\"dylib\"]
plugin = true

[dependencies.worldsong-common]
path = \"{common_dir}\"

{generated_text}
",
common_dir = hierarchy::get_common_src_dir().as_os_str().to_str().unwrap(),
generated_text = toml::encode_str(&final_manifest));


    let cargo_toml_path = hierarchy::get_state_src_dir().join("Cargo.toml");

    println!("Creating new Cargo.toml");
    let mut cargo_toml_file = File::create(&cargo_toml_path).unwrap();
    cargo_toml_file.write_all(cargo_toml_text.as_bytes()).unwrap();
    cargo_toml_file.flush().unwrap();
}

fn get_version(d: &TomlDependency) -> String {
    match d {
        &TomlDependency::Simple(ref version) => {
            version.clone()
        }
        &TomlDependency::Detailed(ref details) => {
            match details.version {
                Some(ref version) => version.clone(),
                None => "".to_string()
            }
        }
    }
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
