mod args;
mod outcome;

use crate::outcome::{
    AppMessage,
    AppError,
    AppResult,
};
use crate::args::{
    AppArgs,
};

use clap::Parser;
use std::process::ExitCode;
use std::path::{Path, PathBuf};
use std::fs;
use std::io;

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{}", err);
            ExitCode::FAILURE
        },
    }
}

fn run() -> AppResult<()> {
    let args = AppArgs::parse();

    let target_base = args.target.clone().unwrap_or_else(|| {
        PathBuf::from(std::env::var("HOME").expect("HOME env var not set"))
    });

    // Перебираем каждый пакет из переданного списка Vec<String>
    for package in &args.packages {
        println!("\n--- Обработка пакета: {} ---", package);

        // Путь к исходному пакету, например: ./nvim
        let package_dir = args.source.join(package);
        if !package_dir.exists() {
            eprintln!("Ошибка: Пакет '{:?}' не найден. Пропуск.", package_dir);
            continue; // Переходим к следующему пакету вместо выхода
        }

        // Запуск линковки для текущего пакета
        match link_dir(&package_dir, &package_dir, &target_base) {
            Ok(_) => println!("Успешно прилинкован пакет: {}", package),
            Err(e) => eprintln!("Ошибка при обработке пакета {}: {}", package, e),
        }
    }

    Ok(())    
}

fn link_dir(base_dir: &Path, current_dir: &Path, target_base: &Path) -> io::Result<()> {
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let src_path = entry.path();
        
        // Вычисляем относительный путь от корня пакета (например: .config/nvim/init.lua)
        let rel_path = src_path.strip_prefix(base_dir).unwrap();
        let dest_path = target_base.join(rel_path);

        if src_path.is_dir() {
            // Если это папка, воссоздаем структуру в целевой директории и идем глубже
            fs::create_dir_all(&dest_path)?;
            link_dir(base_dir, &src_path, target_base)?;
        } else {
            // Если это файл, создаем символическую ссылку
            if dest_path.exists() {
                println!("Пропущено (уже существует): {:?}", dest_path);
                continue;
            }
            
            // Превращаем исходный путь в абсолютный для корректной работы ссылки
            let abs_src = fs::canonicalize(&src_path)?;
            
            std::os::unix::fs::symlink(&abs_src, &dest_path)?;
                        
            println!("Создана ссылка: {:?} -> {:?}", dest_path, abs_src);
        }
    }
    Ok(())
}
