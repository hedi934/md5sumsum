extern crate walkdir;
use std::{env, process::Command};
extern crate md5;
use rayon::prelude::*;

fn main() {
    let mut paths: Vec<String> = vec![];
    for arg in env::args().skip(1) {
        for entry in walkdir::WalkDir::new(arg)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir())
        {
            paths.push(String::from(entry.path().to_string_lossy()))
        }
    }

    paths.sort_by(|one, two| one.to_lowercase().cmp(&two.to_lowercase()));	

    let mut path = String::new();

    path = paths.par_iter().map(|s| format!("{:x}", md5::compute(s))).collect::<Vec<String>>().join("");

    let f = md5::compute(path);
    let otp = &format!("{:x}", f);
    print!("{}", otp);
}
