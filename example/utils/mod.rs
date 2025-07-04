use std::path::Path;
use std::{fs, io};
use walkdir::WalkDir;

pub fn copy_dir_with_walkdir(src: &Path, dst: &Path) -> io::Result<()> {
    // 确保目标目录存在
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry_result in WalkDir::new(src) {
        let entry = entry_result?;
        let path = entry.path();
        let relative_path = path.strip_prefix(src).expect("Should strip prefix");

        let dest_path = dst.join(relative_path);

        if entry.file_type().is_dir() {
            // 创建目录
            if !dest_path.exists() {
                fs::create_dir_all(&dest_path)?;
            }
        } else if entry.file_type().is_file() {
            // 复制文件
            fs::copy(path, &dest_path)?;
        } else {
            // 其余类型（符号链接等）可按需求处理或忽略
            println!("跳过特殊文件：{}", path.display());
        }
    }

    Ok(())
}
