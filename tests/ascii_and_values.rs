//! ASCII source gate + token value smoke tests.

use std::fs;
use std::path::PathBuf;

use rusty_tokens::{color, radius, space, type_scale};

fn src_rs_files() -> Vec<PathBuf> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    let mut stack = vec![root];
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir).expect("read src") {
            let entry = entry.expect("entry");
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                files.push(path);
            }
        }
    }
    files
}

#[test]
fn source_is_pure_ascii() {
    let mut offenders = Vec::new();
    for path in src_rs_files() {
        let bytes = fs::read(&path).expect("read");
        for (i, b) in bytes.iter().enumerate() {
            if *b > 0x7F {
                offenders.push(format!("{}: offset {i} byte 0x{b:02X}", path.display()));
            }
        }
    }
    assert!(offenders.is_empty(), "{}", offenders.join("\n"));
}

#[test]
fn color_contract() {
    assert_eq!(color::FG, "--rt-color-fg");
    assert_eq!(color::FG_VALUE, "#1a1a1a");
    assert_eq!(space::MD_VALUE, "1rem");
    assert_eq!(radius::MD_VALUE, "0.5rem");
    assert_eq!(type_scale::SIZE_BODY_VALUE, "1rem");
}

#[cfg(feature = "css")]
#[test]
fn root_sheet_smoke() {
    let sheet = rusty_tokens::css::root_sheet();
    assert!(sheet.starts_with(":root {"));
    assert!(sheet.contains(color::FG));
    assert!(sheet.contains(color::FG_VALUE));
}
