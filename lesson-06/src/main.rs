#![warn(clippy::all, clippy::pedantic)]

use std::env::current_dir;
use std::fs::create_dir_all;
use std::io::{Error as IOError};
use std::path::PathBuf;
use std::error::Error as DynError;

fn main() -> Result<(), Box<dyn DynError>> {
    let mut target_path = get_parent_path()?;

    target_path.push("qb1ty");

    match create_dir_in(&target_path) {
        Ok(path) => println!("{path:?}"),
        Err(e) => println!("{e:?}")
    }

    Ok(())
}

fn get_parent_path() -> Result<PathBuf, IOError> {
    let path = current_dir()?;

    Ok(path)
}

fn create_dir_in(path: &PathBuf) -> Result<String, IOError> {
    match create_dir_all(path) {
        Ok(_) => Ok(format!("{}", path.to_string_lossy())),
        Err(e) => Err(e)
    }
}