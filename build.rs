use std::process::Command;


fn main() {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "build_dll.bat"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo better use windows")
            .output()
            .expect("failed to execute process")
    };
}