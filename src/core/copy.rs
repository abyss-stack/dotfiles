use super::config::Package;
use crate::outcome::{AppError, AppMessage, AppResult};
use std::fs;
use std::path::Path;

pub fn apply_copy(pkg: &Package) -> AppResult<()> {
    AppMessage::ApplyCopy {
        package: pkg.name.clone(),
    }
    .emit();

    copy_recursive(&pkg.source, &pkg.target).map_err(|e| AppError::Copy {
        package: pkg.name.clone(),
        source: pkg.source.clone(),
        target: pkg.target.clone(),
        what: e.to_string(),
    })?;

    Ok(())
}

fn copy_recursive(from: &Path, to: &Path) -> std::io::Result<()> {
    if from.is_file() {
        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(from, to)?;
        return Ok(());
    }

    fs::create_dir_all(to)?;

    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let src = entry.path();
        let dst = to.join(entry.file_name());
        copy_recursive(&src, &dst)?;
    }

    Ok(())
}
