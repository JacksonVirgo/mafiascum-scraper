use std::process::Command;

fn main() {
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
}
