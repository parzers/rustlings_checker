use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{PathBuf, Path};
use std::process::{Command, Stdio};

fn rustlings_dir(path:&PathBuf) -> Result<PathBuf, Error>
{
    for entry in path.read_dir()? {
        let entry = entry?;
        let entry_path = &entry.path();
        let path_str = entry_path.to_str().unwrap();
        if path_str.ends_with("Cargo.toml") {
            return Ok(path.clone());
        }
    }
    for entry in path.read_dir()? {
        let entry = entry?;
        if entry.metadata()?.is_dir() {
            return rustlings_dir(&entry.path());
        }
    }
    let path_str = path.to_str().unwrap();
    Err(Error::new(ErrorKind::Other, format!("no rustlings dir found in {path_str}").as_str()))
}

fn check_rustlings(path:&PathBuf) {
    assert!(std::env::set_current_dir(path).is_ok());
    println!("{}...", path.to_str().unwrap());
    let exercises = ["intro1", "intro2",
     "variables1", "variables2", "variables3", "variables4", "variables5", "variables6",
     "functions1", "functions2", "functions3", "functions4", "functions5",
     "if1", "if2", "if3",
     "quiz1",
     "primitive_types1", "primitive_types2", "primitive_types3", "primitive_types4", "primitive_types5", "primitive_types6", 
     "vecs1", "vecs2"];
     let sum:i32 = exercises.into_iter().map(|ex|{
        let cmd = format!("rustlings run {}", ex);
        let status = Command::new("cmd")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .args(["/C", &cmd])
            .status()
            .expect("Could not run command {cmd}");
        if status.success() { 1 } else {
            println!("-- {cmd} not successful!");
            0 
        }
    }).sum();
    println!("=> score of {}/{}", sum, exercises.len());
}

fn main() -> Result<(), Error> {
    let path = "C:/HTL/local/20250123_5AHMNG_Rustlings5";
    let paths = fs::read_dir(path).expect(format!("Could not open {path}").as_str());
    for entry in paths {
        let entry = entry?;
        if let Ok(rustlings_path) = rustlings_dir(&entry.path()) {
            check_rustlings(&rustlings_path);
        } else {
            println!("ERROR! No rustlings dir found in {entry:?}!");
        }
    }
    Ok(())
}
