mod args;
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let args = args::AppArgs::parse();

    let target_base = args.target.unwrap_or_else(|| {
        PathBuf::from(std::env::var("HOME").expect("ERROR: HOME env var not set"))
    });

    let source_base = args
        .source
        .unwrap_or_else(|| std::env::current_dir().expect("ERROR: Cannot get current directory"));

    for package in &args.packages {
        let src_package_dir = source_base.join(package);

        if !src_package_dir.exists() {
            eprintln!("FAIL: Папка пакета {:?} не существует.", src_package_dir);
            continue;
        }

        println!("\n--- Линковка пакета: {} ---", package);
        link_recursive(&src_package_dir, &src_package_dir, &target_base);
        println!("Пакет '{}' успешно применён!", package);
    }
}

fn link_recursive(base_dir: &Path, current_dir: &Path, target_base: &Path) {
    if let Ok(entries) = fs::read_dir(current_dir) {
        for entry in entries.flatten() {
            let src_path = entry.path();
            let rel_path = src_path.strip_prefix(base_dir).unwrap();
            let dest_path = target_base.join(rel_path);

            if src_path.is_dir() {
                // Если это папка, создаем её в целевом каталоге и идём глубже
                let _ = fs::create_dir_all(&dest_path);
                link_recursive(base_dir, &src_path, target_base);
            } else {
                // Если файл или старая симлинка уже есть — сносим
                if dest_path.exists() || fs::symlink_metadata(&dest_path).is_ok() {
                    let _ = fs::remove_file(&dest_path);
                    let _ = fs::remove_dir_all(&dest_path);
                }

                // Создаем родительские папки, если их нет
                if let Some(parent) = dest_path.parent() {
                    let _ = fs::create_dir_all(parent);
                }

                let abs_src = fs::canonicalize(&src_path).unwrap_or(src_path);

                if let Err(e) = std::os::unix::fs::symlink(&abs_src, &dest_path) {
                    eprintln!("Ошибка линковки {:?}: {}", dest_path, e);
                } else {
                    println!("Создан линк: {:?} -> {:?}", dest_path, abs_src);
                }
            }
        }
    }
}
