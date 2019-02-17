// build.rs

use std::env;
use std::fs::File;
use std::fs::copy;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("PROFILE").unwrap();
    let dest_path = Path::new("target").join(&out_dir).join("mm2tracker.exe.manifest");
    let src_path = Path::new("manifest.xml");
    copy(src_path, dest_path).unwrap();
}
