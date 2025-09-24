/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#[macro_use]
extern crate lazy_static;

use std::env;
use std::path::Path;
use std::process::{exit, Command};
use walkdir::WalkDir;

#[cfg(feature = "gecko")]
mod build_gecko;

#[cfg(not(feature = "gecko"))]
mod build_gecko {
    pub fn generate() {}
}

lazy_static! {
    pub static ref PYTHON: String = env::var("PYTHON3").ok().unwrap_or_else(|| {
        let candidates = if cfg!(windows) {
            ["python.exe"]
        } else {
            ["python3"]
        };
        for &name in &candidates {
            if Command::new(name)
                .arg("--version")
                .output()
                .ok()
                .map_or(false, |out| out.status.success())
            {
                return name.to_owned();
            }
        }
        panic!(
            "Can't find python (tried {})! Try fixing PATH or setting the PYTHON3 env var",
            candidates.join(", ")
        )
    });
}

fn generate_properties(engine: &str) {
    for entry in WalkDir::new("properties") {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Warning: Failed to read directory entry: {}", e);
                continue;
            },
        };
        match entry.path().extension().and_then(|e| e.to_str()) {
            Some("mako") | Some("rs") | Some("py") | Some("zip") => {
                println!("cargo:rerun-if-changed={}", entry.path().display());
            },
            _ => {},
        }
    }

    let manifest_dir = match env::var_os("CARGO_MANIFEST_DIR") {
        Some(dir) => dir,
        None => {
            eprintln!("Error: CARGO_MANIFEST_DIR environment variable not set");
            exit(1);
        },
    };

    let script = Path::new(&manifest_dir).join("properties").join("build.py");

    let status = match Command::new(&*PYTHON)
        // `cargo publish` isn't happy with the `__pycache__` files that are created
        // when we run the property generator.
        //
        // TODO(mrobinson): Is this happening because of how we run this script? It
        // would be better to ensure are just placed in the output directory.
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .arg(&script)
        .arg(engine)
        .arg("style-crate")
        .status()
    {
        Ok(status) => status,
        Err(e) => {
            eprintln!("Error: Failed to execute Python script: {}", e);
            exit(1);
        },
    };
    if !status.success() {
        exit(1)
    }
}

fn main() {
    let gecko = cfg!(feature = "gecko");
    let servo = cfg!(feature = "servo");
    let engine = match (gecko, servo) {
        (true, false) => "gecko",
        (false, true) => "servo",
        (false, false) => {
            // Default to servo when no features are explicitly enabled
            // This matches the default feature in Cargo.toml
            "servo"
        },
        (true, true) => {
            // When both features are enabled (e.g., with --all-features), prefer servo
            println!("cargo:warning=Both servo and gecko features enabled, preferring servo");
            "servo"
        },
    };
    println!("cargo:rerun-if-changed=build.rs");
    match env::var("OUT_DIR") {
        Ok(out_dir) => println!("cargo:out_dir={}", out_dir),
        Err(_) => {
            eprintln!("Warning: OUT_DIR environment variable not set");
        },
    }
    generate_properties(engine);
    if engine == "gecko" {
        build_gecko::generate();
    }
}
