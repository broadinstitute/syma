use std::path::PathBuf;
use home::home_dir;
use crate::error::{Error, for_file};

pub(crate) fn get_syma_dir() -> Result<PathBuf, Error> {
    let home_dir =
        home_dir().ok_or_else(|| Error::from("Don't know where your home directory is."))?;
    let syma_dir = home_dir.join(".syma");
    if !syma_dir.exists() {
        for_file(syma_dir.as_os_str().to_string_lossy(), std::fs::create_dir(&syma_dir))?;
    }
    Ok(syma_dir)
}

pub(crate) fn get_history_file() -> Result<PathBuf, Error> {
    let syma_dir = get_syma_dir()?;
    let history_file = syma_dir.join("history");
    Ok(history_file)
}