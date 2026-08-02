use super::config::Package;
use crate::outcome::{AppError, AppMessage, AppResult};
use std::fs;
use std::os::unix::fs::symlink;

pub fn apply_symlink(pkg: &Package) -> AppResult<()> {
    AppMessage::ApplySymlink {
        package: pkg.name.clone(),
    }
    .emit();

    if pkg.target.exists() {
        if pkg.target.is_dir() {
            fs::remove_dir_all(&pkg.target).map_err(|e| AppError::RemoveDir {
                package: pkg.name.clone(),
                path: pkg.target.clone(),
                what: e.to_string(),
            })?;
        } else {
            fs::remove_file(&pkg.target).map_err(|e| AppError::RemoveFile {
                package: pkg.name.clone(),
                path: pkg.target.clone(),
                what: e.to_string(),
            })?;
        }
    }

    symlink(&pkg.source, &pkg.target).map_err(|e| AppError::Symlink {
        package: pkg.name.clone(),
        source: pkg.source.clone(),
        target: pkg.target.clone(),
        what: e.to_string(),
    })?;

    Ok(())
}
