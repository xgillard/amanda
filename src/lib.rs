extern crate walkdir;

use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Write, BufWriter};

use walkdir::WalkDir;

pub fn strip_tree(from: &str, to: &str) {
    let from = Path::new(from);
    let to = Path::new(to);

    for entry in WalkDir::new(from) {
        let entry = entry.expect("Could not read entry");

        let original = entry.path();
        let suffix = original.strip_prefix(from).unwrap();
        let target = to.join(suffix);

        if original.is_dir() {
           std::fs::create_dir_all(target)
               .expect("Could not create the target directory");
        } else if original.is_file() {
            let original = File::open(original).unwrap();
            let target= File::create(target).unwrap();
            strip_file(&original, &target);
        }
    }
}

pub fn strip_file(input: &File, output: &File) {
    let source= BufReader::new(input);
    let mut dest= BufWriter::new(output);
    let mut stripping = false;

    for line in source.lines() {
        // ignore binary files
        if let Ok(text) = line {
            match text.trim() {
                ref _t if _t.starts_with("// BEGIN STRIP") => stripping = true,
                ref _t if _t.starts_with("// END STRIP") => stripping = false,
                ref _t if !stripping => {
                    writeln!(&mut dest, "{}", text.replace("// STUDENT", ""))
                        .expect("Could not write to output file");
                },
                _ => { /* strip code = do nothing */ }
            }
        }
    }
}