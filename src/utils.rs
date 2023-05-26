use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;
use zip::ZipArchive;

pub fn download_file(url: &str, file_name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let tmp_dir_name = std::env::temp_dir();

    let file_path = tmp_dir_name.join(file_name);
    if file_path.exists() {
        fs::remove_file(&file_path)?;
    }

    let output = Command::new("curl")
        .arg("-L")
        .arg("-o")
        .arg(format!("{}/{}", tmp_dir_name.to_str().unwrap(), file_name))
        .arg(url)
        .output()?;
    if !output.status.success() {
        return Err(format!(
            "Failed to download file: {}",
            str::from_utf8(&output.stderr)?
        )
        .into());
    }
    let filename = url.split('/').last().unwrap_or("download");
    let file_path = std::env::temp_dir().join(filename);
    Ok(file_path)
}

pub fn unzip_file(file_path: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut archive = ZipArchive::new(File::open(file_path)?)?;
    let mut output_dir = file_path.parent().unwrap().to_path_buf();
    output_dir.push(file_path.file_stem().unwrap());
    if output_dir.exists() {
        std::fs::remove_dir_all(&output_dir)?;
    }
    std::fs::create_dir(&output_dir)?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let mut output_path = output_dir.clone();
        output_path.push(file.mangled_name());
        if (&*file.name()).ends_with('/') {
            std::fs::create_dir_all(&output_path)?;
        } else {
            if let Some(p) = output_path.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p)?;
                }
            }
            let mut output_file = File::create(&output_path)?;
            std::io::copy(&mut file, &mut output_file)?;
        }
    }
    Ok(output_dir)
}

pub fn move_folder(source: &Path, target: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !source.exists() {
        return Err(format!("Source folder does not exist: {}", source.display()).into());
    }
    if target.exists() {
        std::fs::remove_dir_all(target)?;
    }
    std::fs::rename(source, target)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_and_unzip() {
        let url = "https://github.com/matter-labs/zksync-era/archive/refs/heads/main.zip";
        let file_path = download_file(url, "main.zip").unwrap();
        dbg!(&file_path);
        let output_dir = unzip_file(&file_path).unwrap();
        assert!(output_dir.exists());
        assert!(output_dir.is_dir());
        dbg!(&output_dir);

        let target = PathBuf::from("./schema/migrations");
        move_folder(
            &output_dir.join("zksync-era-main/core/lib/dal/migrations"),
            &target,
        )
        .unwrap();
    }
}
