use std::io::{self, Error, Write};
use std::process::{Command, Output};

fn main() {
    match build_tailwindcss() {
        Ok(_) => (),
        Err(e) => panic!("Failed to build tailwindcss: {}", e),
    }
}

fn build_tailwindcss() -> Result<Output, Error> {
    let output = Command::new("tailwindcss")
        .arg("-o")
        .arg("./tailwind.css")
        .output()?;

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());

    Ok(output)
}
