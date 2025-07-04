mod utils;

use crate::utils::copy_dir_with_walkdir;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

fn main() {
    #[cfg(target_os = "windows")]
    {
        let _ = dotenvy::dotenv();
        // 执行编译命令
        let dir = env!("CARGO_MANIFEST_DIR");

        Command::new("cargo")
            .envs(env::vars())
            .args(["build"])
            .current_dir(PathBuf::from(dir))
            .spawn()
            .unwrap()
            .wait_with_output()
            .unwrap();

        let package_name = env::var("CARGO_PKG_NAME").unwrap();
        println!("package_name: {}", package_name);

        let plugin_dir = match env::var("PLUGIN_DIR") {
            Ok(plugin_dir) => PathBuf::from(plugin_dir),
            Err(_) => PathBuf::from(env::var("APPDATA").unwrap())
                .join("FlowLauncher")
                .join("Plugins"),
        };
        let target_dir = plugin_dir.join(&package_name);
        println!("target_dir: {}", target_dir.display());

        fs::create_dir_all(&target_dir).unwrap();

        // 获取可执行文件名
        let package_name = format!("{}.exe", package_name);

        let release_exe = PathBuf::from("target/debug").join(&package_name);

        if release_exe.exists() {
            fs::copy(&release_exe, target_dir.join(&package_name)).unwrap();
        }

        fs::copy("plugin.json", target_dir.join("plugin.json"))
            .expect("Unable to copy plugin.json");

        let template = PathBuf::from("SettingsTemplate.yaml");
        if template.exists() {
            fs::copy(&template, target_dir.join(&template)).unwrap();
        }

        let images_dir = PathBuf::from("Images");

        if images_dir.exists() {
            fs::create_dir_all(target_dir.join(&images_dir)).unwrap();
            let images_dst = target_dir.join(&images_dir);
            copy_dir_with_walkdir(&images_dir, &images_dst).unwrap();
        }
    }
}
