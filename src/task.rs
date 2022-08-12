/// Stop docker containers with "aegis" in its name.
pub fn stop_aegis_containers() {
    docker_stop(container_ids());
}

/// Get space-separated aegis containers' IDs.
// pub fn container_ids() -> String {
//     use std::process::{Command, Output};
//
//     // docker ps --quiet --filter "name=aegis"
//     let output = Command::new("docker")
//         .args(["ps", "-q"])
//         .args(["-f", "name=hello"])
//         .output();
//
//     match output {
//         Ok(Output { stdout: bytes, .. }) => {
//             let stdout = String::from_utf8(bytes).unwrap_or(String::new());
//             stdout.replace("\n", " ").trim().to_owned()
//         }
//         _ => String::new(),
//     }
// }

pub fn container_ids() -> Vec<String> {
    use std::process::{Command, Output};

    // docker ps --quiet --filter "name=aegis"
    let output = Command::new("docker")
        .args(["ps", "-q"])
        .args(["-f", "name=hello"])
        .output();

    match output {
        Ok(Output { stdout: bytes, .. }) => {
            let stdout = String::from_utf8(bytes).unwrap_or(String::new());
            stdout
                .split('\n')
                .filter(|x| !x.is_empty())
                .map(|x| x.to_owned())
                .collect()
        }
        _ => Vec::new(),
    }
}

fn docker_stop(containers: Vec<String>) {
    use std::process::Command;

    let mut cmd = Command::new("docker");
    cmd.arg("stop");

    for container in containers {
        println!("INFO: docker stop {}", &container);
        cmd.arg(container);
    }
    cmd.status().expect("todo");
}

#[test]
fn test_docker_stop() {
    let containers = container_ids();
    docker_stop(containers);
}

pub fn launch_tui() {
    println!("launching beautiful tui...");
}
