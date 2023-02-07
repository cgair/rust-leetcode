/**
 * Aiming to clarify the following questions:
 * 
 * How long will my program take?
 * Why does my program run out of memory?
 */
use lazy_static::lazy_static;
use std::env;
use std::path::{Path, PathBuf};
use std::fs;

pub mod three_sum_ncubed;
pub mod three_sum_fast;

lazy_static! {
    pub static ref SUM: Vec<PathBuf> = get_test_data();
}

fn get_test_data() -> Vec<PathBuf> {
    let mut data_files = Vec::new();
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let data_dir = Path::new(&crate_dir).join("data/sum");
    if data_dir.is_dir() {
        for entry in fs::read_dir(data_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            data_files.push(path);
        }
        return data_files;
    } else {
        vec![]
    }
}


#[test]
fn it_get_test_data() {
    let dd = get_test_data();
    println!("{:?}", dd);
}