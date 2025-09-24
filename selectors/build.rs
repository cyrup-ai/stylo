/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::process::exit;

fn main() {
    let out_dir = match env::var_os("OUT_DIR") {
        Some(dir) => dir,
        None => {
            eprintln!("Error: OUT_DIR environment variable not set");
            exit(1);
        },
    };

    let path = Path::new(&out_dir).join("ascii_case_insensitive_html_attributes.rs");
    let file = match File::create(&path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error: Failed to create output file: {}", e);
            exit(1);
        },
    };

    let mut file = BufWriter::new(file);

    let mut set = phf_codegen::Set::new();
    for name in ASCII_CASE_INSENSITIVE_HTML_ATTRIBUTES.split_whitespace() {
        set.entry(name);
    }

    match write!(
        &mut file,
        "{{ static SET: ::phf::Set<&'static str> = {}; &SET }}",
        set.build(),
    ) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error: Failed to write to output file: {}", e);
            exit(1);
        },
    }
}

/// <https://html.spec.whatwg.org/multipage/#selectors>
static ASCII_CASE_INSENSITIVE_HTML_ATTRIBUTES: &str = r#"
    accept
    accept-charset
    align
    alink
    axis
    bgcolor
    charset
    checked
    clear
    codetype
    color
    compact
    declare
    defer
    dir
    direction
    disabled
    enctype
    face
    frame
    hreflang
    http-equiv
    lang
    language
    link
    media
    method
    multiple
    nohref
    noresize
    noshade
    nowrap
    readonly
    rel
    rev
    rules
    scope
    scrolling
    selected
    shape
    target
    text
    type
    valign
    valuetype
    vlink
"#;
