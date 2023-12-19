use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub fn get_conf_paths() -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let paths = fs::read_dir("/etc/wireguard")
        .unwrap()
        .filter_map(|res| res.ok())
        .map(|dir| dir.path())
        .filter_map(|path| {
            if path.extension().map_or(false, |conf| conf == "conf") {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Ok(paths)
}
