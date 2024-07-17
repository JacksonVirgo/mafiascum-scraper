use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() {
    Command::new("npm")
        .arg("install")
        .arg("-g")
        .arg("tailwindcss")
        .output()
        .expect("TailwindCSS installed");

    let output = Command::new("npx")
        .arg("tailwindcss")
        .arg("-i")
        .arg("./src/static/all.css")
        .arg("-o")
        .arg("./src/static/output.css")
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_dir = PathBuf::from(out_dir).join("static");
    copy_dir_all("static", &dest_dir).unwrap();
}
