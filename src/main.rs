mod args;
mod outcome;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Parser;

use crate::args::AppArgs;
use crate::outcome::{AppError, AppMessage, AppResult};

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            e.emit();
            ExitCode::FAILURE
        }
    }
}

fn run() -> AppResult<()> {
    let args = AppArgs::parse();

    //Where to take dotfiles from.
    let raw_source = match args.source {
        Some(s) => s,
        None => std::env::current_dir().map_err(|_| AppError::CurrentDirNotFound)?,
    };
    let source = raw_source
        .canonicalize()
        .map_err(|e| AppError::CanonicalizeError {
            what: e.to_string(),
        })?;

    AppMessage::UsingSourcePath {
        path: source.to_string_lossy().into_owned(),
    }
    .emit();

    // Where to create symlinks.
    let raw_target = match args.target {
        Some(t) => t,
        None => dirs::home_dir().ok_or(AppError::HomeDirNotFound)?,
    };
    let target = raw_target
        .canonicalize()
        .map_err(|e| AppError::CanonicalizeError {
            what: e.to_string(),
        })?;

    AppMessage::UsingTargetPath {
        path: target.to_string_lossy().into_owned(),
    }
    .emit();

    let pre_script = source.join("pre.sh");
    if pre_script.exists() {
        if let Err(e) = duct::cmd!("sh", &pre_script).dir(&source).run() {
            return Err(AppError::ScriptError {
                script: "pre".to_string(),
                what: e.to_string(),
            });
        }
        AppMessage::ScriptFinished {
            script: "pre".to_string(),
        }.emit();
    }

    for package in &args.packages {
        let package_dir = source.join(package);

        if !package_dir.exists() {
            AppMessage::PackageSkipped {
                package: package.clone(),
            }
            .emit();
            continue;
        }

        AppMessage::LinkingPackage {
            package: package.clone(),
        }
        .emit();

        link_recursive(&package_dir, &package_dir, &target)?;
    }
    
    let post_script = source.join("post.sh");
    if post_script.exists() {
        if let Err(e) = duct::cmd!("sh", &post_script).dir(&source).run() {
            return Err(AppError::ScriptError {
                script: "post".to_string(),
                what: e.to_string(),
            });
        }
        AppMessage::ScriptFinished {
            script: "post".to_string(),
        }.emit();
    }

    Ok(())
}

fn link_recursive(base_dir: &Path, current_dir: &Path, target_base: &Path) -> AppResult<()> {
    if let Ok(entries) = fs::read_dir(current_dir) {
        for entry in entries.flatten() {
            let src_path = entry.path();
            let rel_path = src_path.strip_prefix(base_dir).unwrap();
            let dest_path = target_base.join(rel_path);

            if src_path.is_dir() {
                let _ = fs::create_dir_all(&dest_path);
                link_recursive(base_dir, &src_path, target_base)?;
            } else {
                if dest_path.exists() || fs::symlink_metadata(&dest_path).is_ok() {
                    let _ = fs::remove_file(&dest_path);
                    let _ = fs::remove_dir_all(&dest_path);
                }

                if let Some(parent) = dest_path.parent() {
                    let _ = fs::create_dir_all(parent);
                }

                let abs_src = fs::canonicalize(&src_path).unwrap_or(src_path);

                if let Err(e) = std::os::unix::fs::symlink(&abs_src, &dest_path) {
                    return Err(AppError::LinkingError {
                        from: dest_path.to_string_lossy().into_owned(),
                        what: e.to_string(),
                    });
                } else {
                    AppMessage::LinkCreated {
                        from: dest_path.to_string_lossy().into_owned(),
                        to: abs_src.to_string_lossy().into_owned(),
                    }
                    .emit();
                }
            }
        }
    }

    Ok(())
}
