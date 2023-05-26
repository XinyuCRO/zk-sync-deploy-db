use std::process::Command;

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
