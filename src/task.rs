use std::collections::HashMap;

/// Stop docker containers with "aegis" in its name.
pub fn stop_aegis_apps() {
    docker_stop(docker_ps_quiet("hello"));
}

/// Get docker container IDs.
pub fn docker_ps_quiet(name_filter: &str) -> Vec<String> {
    use std::process::{Command, Output};

    // docker ps --quiet --filter "name=aegis"
    let output = Command::new("docker")
        .args(["ps", "-q"])
        .args(["-f", &format!("name={}", name_filter)])
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

pub fn docker_ps_format_id_names() -> HashMap<String, String> {
    use std::process::{Command, Output};

    // docker ps --format '{{ .ID }}\t{{ .Names }}'
    let output = Command::new("docker")
        .args(["ps", "--format"])
        .arg("{{ .ID }}\t{{ .Names }}")
        .output();

    match output {
        Ok(Output { stdout: bytes, .. }) => {
            let stdout = String::from_utf8(bytes).unwrap_or_default();
            let pairs = stdout.split('\n').filter(|x| !x.is_empty());

            let mut m: HashMap<String, String> = HashMap::new();
            for pair in pairs {
                let mut kv = pair.split('\t');
                if let (Some(v), Some(k)) = (kv.next(), kv.next()) {
                    m.insert(k.to_string(), v.to_string());
                }
            }
            m
        }
        _ => HashMap::new(),
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
    // TODO assert cmd arity?
    cmd.status().expect("todo");
}

pub fn docker_stop_by_id(container: &str) {
    use std::process::Command;

    Command::new("docker")
        .arg("stop")
        .arg(container)
        .status()
        .expect("todo");

    println!("INFO: docker stop {}", &container);
}

#[test]
fn test_docker_stop() {
    let containers = docker_ps_quiet("hello");
    docker_stop(containers);
}

// TODO add regression test to spin helloworld containers, then cargo test
