use crate::config::Config;
use std::fs;

pub fn list_local_repos(config: &Config) -> Result<Vec<String>, std::io::Error> {
    let mut folders = vec![];

    for entry in fs::read_dir(&config.root_folder)? {
        let path = entry?.path();
        if path.is_dir() {
            let folder_name = path
                .file_name()
                .ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid folder name")
                })?
                .to_string_lossy()
                .into_owned();
            folders.push(folder_name);
        }
    }

    Ok(folders)
}
