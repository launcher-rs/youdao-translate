mod utils;

use crate::utils::copy_dir_with_walkdir;
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;

fn main() {
    #[cfg(target_os = "windows")]
    {
        // 执行编译命令
        let dir = env!("CARGO_MANIFEST_DIR");

        Command::new("cargo")
            .envs(env::vars())
            .args(["build", "--release"])
            .current_dir(PathBuf::from(dir))
            .spawn()
            .unwrap()
            .wait_with_output()
            .unwrap();

        let package_name = env::var("CARGO_PKG_NAME").unwrap();
        println!("package_name: {}", package_name);

        let bin_path = PathBuf::from("dist").join(format!("{}", package_name));

        fs::create_dir_all(&bin_path).unwrap();
        fs::create_dir_all(&bin_path.join("images")).unwrap();

        // 获取可执行文件名
        let package_name = format!("{}.exe", package_name);

        let release_exe = PathBuf::from("target/release").join(&package_name);

        if release_exe.exists() {
            fs::copy(&release_exe, &bin_path.join(&package_name)).unwrap();
        }

        fs::copy("plugin.json", &bin_path.join("plugin.json")).expect("Unable to copy plugin.json");

        let template = PathBuf::from("SettingsTemplate.yaml");
        if template.exists() {
            fs::copy(&template, &bin_path.join(&template)).unwrap();
        }

        let images_dir = PathBuf::from("Images");
        if images_dir.exists() {
            let images_dst = bin_path.join(&images_dir);
            copy_dir_with_walkdir(&images_dir, &images_dst).unwrap();
        }

        // 压缩
        // 创建一个 ZipWriter
        let zip_path = format!("{}.zip", bin_path.display());
        let file = File::create(zip_path).expect("Failed to create file");

        let mut zip = zip::ZipWriter::new(file);
        // 设置压缩选项（压缩方法、权限等）
        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);

        for entry in WalkDir::new(&bin_path) {
            let entry = entry.unwrap();
            let path = entry.path();

            // 构造ZIP内的文件/目录名称（相对路径）
            let name = path.strip_prefix(&bin_path).unwrap();

            if path.is_file() {
                println!("添加文件：{}", path.display());

                zip.start_file(name.to_str().unwrap(), options).unwrap();

                let mut f = File::open(path).expect("打开文件失败");
                std::io::copy(&mut f, &mut zip).unwrap();
            } else if path.is_dir() {
                // ZIP中目录名必须以 '/' 结尾
                let dir_name = format!("{}/", name.to_str().unwrap());
                println!("添加目录：{}", dir_name);
                zip.add_directory(dir_name, options).unwrap();
            }
        }

        zip.finish().unwrap();
        println!("压缩完成");
    }
}
