/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::exit;

fn main() {
    let manifest_dir = match env::var_os("CARGO_MANIFEST_DIR") {
        Some(dir) => dir,
        None => {
            eprintln!("Error: CARGO_MANIFEST_DIR environment variable not set");
            exit(1);
        },
    };

    let static_atoms_path = Path::new(&manifest_dir).join("static_atoms.txt");
    let static_atoms_file = match File::open(&static_atoms_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error: Failed to open static_atoms.txt: {}", e);
            exit(1);
        },
    };

    let static_atoms = BufReader::new(static_atoms_file);
    let mut atom_type = string_cache_codegen::AtomType::new("Atom", "atom!");

    macro_rules! predefined {
        ($($name: expr,)+) => {
            {
                $(
                    atom_type.atom($name);
                )+
            }
        }
    }
    include!("./predefined_counter_styles.rs");

    let out_dir = match env::var_os("OUT_DIR") {
        Some(dir) => dir,
        None => {
            eprintln!("Error: OUT_DIR environment variable not set");
            exit(1);
        },
    };

    let lines: Vec<String> = static_atoms
        .lines()
        .filter_map(|line| match line {
            Ok(content) => Some(content),
            Err(e) => {
                eprintln!("Warning: Failed to read line from static_atoms.txt: {}", e);
                None
            },
        })
        .collect();

    match atom_type
        .atoms(lines.into_iter())
        .write_to_file(&Path::new(&out_dir).join("atom.rs"))
    {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error: Failed to write atom.rs: {}", e);
            exit(1);
        },
    }
}
