// build.rs

use std::env;
use std::fs::File;
use std::fs::copy;
use std::io::Write;
use std::path::Path;

fn main() {
    //let out_dir = env::var("PROFILE").unwrap();
    //let dest_path = Path::new("target").join(&out_dir).join("mm2tracker.exe.manifest");
    let src_path = Path::new("resource.rc");
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("resource.rc");
    copy(src_path, dest_path).unwrap();
    let mut res = winres::WindowsResource::new();
    res.set_resource_file("resource.rc")
        .compile().unwrap();
}
