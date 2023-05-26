use std::process::Command;

use crate::database;

pub async fn drop(db: &database::Database) {
    println!("Droping DB...");
    match db.drop().await {
        Ok(_) => println!("DB dropped"),
        Err(e) => println!("DP drop failed, {}", e),
    }
}

pub async fn setup(db: &database::Database) {
    println!("Setting up DB...");
    match db.setup().await {
        Ok(_) => println!("DB setup complete"),
        Err(e) => println!("DB setup failed, {}", e),
    }
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
