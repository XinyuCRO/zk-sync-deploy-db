use std::{process::Command, path::PathBuf};

use crate::{database::*, utils::*};

pub async fn drop(db: &Database) {
    println!("Droping DB...");
    match db.drop().await {
        Ok(_) => println!("DB dropped"),
        Err(e) => println!("DP drop failed, {}", e),
    }
}

pub async fn setup(db: &Database) {
    println!("Setting up DB...");
    match db.setup().await {
        Ok(_) => println!("DB setup complete"),
        Err(e) => println!("DB setup failed, {}", e),
    }
}

pub async fn check(db: &Database) {
    println!("Print DB tables...");
    db.print_all_tables().await;
}

pub fn start() {
    // run command "mkdir -p ./volumes/postgres"
    let mkdir_result = Command::new("mkdir")
        .arg("-p")
        .arg("./volumes/postgres")
        .status();

    match mkdir_result {
        Ok(status) => {
            if !status.success() {
                eprintln!("Error: failed to create data directory");
            }
        }
        Err(e) => {
            eprintln!("Error: failed to execute mkdir command: {}", e);
        }
    }

    // run command "docker-compose up -d"
    let docker_result = Command::new("docker-compose").arg("up").arg("-d").status();

    match docker_result {
        Ok(status) => {
            if !status.success() {
                eprintln!("Error: failed to start docker-compose");
            }
        }
        Err(e) => {
            eprintln!("Error: failed to execute docker-compose command: {}", e);
        }
    }
}

pub fn stop() {
    // run command "docker-compose down"
    let docker_result = Command::new("docker-compose").arg("down").status();

    match docker_result {
        Ok(status) => {
            if !status.success() {
                eprintln!("Error: failed to run docker-compose");
            }
        }
        Err(e) => {
            eprintln!("Error: failed to execute docker-compose command: {}", e);
        }
    }
}

pub fn destroy() {
    // run command "docker-compose down -v"
    let docker_result = Command::new("docker-compose")
        .arg("down")
        .arg("-v")
        .status();

    match docker_result {
        Ok(status) => {
            if !status.success() {
                eprintln!("Error: failed to run docker-compose");
            }
        }
        Err(e) => {
            eprintln!("Error: failed to execute docker-compose command: {}", e);
        }
    }

    let data_dir = std::env::var("POSTGRES_DATA_DIR").unwrap();

    if data_dir.len() == 0 {
        eprintln!("Error: POSTGRES_DATA_DIR is not set");
        return;
    }

    // remove env.POSTGRES_DATA_DIR
    let rm_result = Command::new("rm").arg("-rf").arg(data_dir).status();

    match rm_result {
        Ok(status) => {
            if !status.success() {
                eprintln!("Error: failed to remove data directory");
            }
        }
        Err(e) => {
            eprintln!("Error: failed to execute rm command: {}", e);
        }
    }
}

pub async fn sync() {
    println!("Syncing DB schemas...");
    let url = "https://github.com/matter-labs/zksync-era/archive/refs/heads/main.zip";
    println!("Downloading file from {}...", url);
    let file_path = download_file(url, "main.zip").expect("Failed to download file");
    println!("Unzipping file...");
    let output_dir = unzip_file(&file_path).expect("Failed to unzip file");
    println!("Download & unzip success, path: {:?}", output_dir);
    println!("Moving migrations files...");
    let target = PathBuf::from("./schema/migrations");
    move_folder(&output_dir.join("zksync-era-main/core/lib/dal/migrations"), &target).expect("Failed to move folder");

    println!("Sync complete");
}

pub async fn migrate(db: &Database) {
    println!("Migrating DB...");
    match db.migrate().await {
        Ok(_) => println!("DB migrated"),
        Err(e) => println!("DB migrate failed, {}", e),
    }
}

pub fn print(db: &Database) {
    println!("{}", db.url);
}