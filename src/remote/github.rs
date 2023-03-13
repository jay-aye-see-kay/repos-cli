use std::{process::Command, io};

pub fn get_token() -> io::Result<String> {
    let output = Command::new("gh").arg("auth").arg("token").output()?;

    if !output.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, ""))
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.trim().to_string())
}
