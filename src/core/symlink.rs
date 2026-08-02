use super::config::Package;
use crate::outcome::{AppError, AppMessage, AppResult};
use std::os::unix::fs::symlink;

pub fn apply_symlink(pkg: &Package) -> AppResult<()> {
    AppMessage::ApplySymlink {
        package: pkg.name.clone(),
    }.emit();

    symlink(&pkg.source, &pkg.target).map_err(|e| AppError::Symlink {
        package: pkg.name.clone(),
        source: pkg.source.clone(),
        target: pkg.target.clone(),
        what: e.to_string(),
    })?;
    
    Ok(())
}
