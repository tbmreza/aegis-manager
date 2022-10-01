pub fn state_output() -> String {
    use std::process::{Command, Output};

    let output = Command::new("/opt/cisco/anyconnect/bin/vpn")
        .arg("state")
        .output();

    match output {
        Ok(Output { stdout: bytes, .. }) => {
            let s = String::from_utf8(bytes).unwrap_or_default();
            // println!("INFO: {:?}", s);
            s
        }
        e => {
            println!("{:?}", e);
            String::new()
        }
    }
}

pub fn is_disconnected() -> Option<(String, String)> {
    if state_output().contains("Disconnected") {
        return Some((
            String::from("No aegis apps running. Start?"),
            String::from("aegis start 1"),
        ));
    }
    None
}

pub fn disconnect() {
    use std::process::{Command, Output};

    let output = Command::new("/opt/cisco/anyconnect/bin/vpn")
        .arg("disconnect")
        .output();

    match output {
        Ok(Output { stdout: bytes, .. }) => {
            println!("INFO: {:?}", String::from_utf8(bytes));
        }
        e => println!("{:?}", e),
    }
}

/// Wrapper to a shell script. Piping as demonstrated in [this example] doesn't
/// work because the VPN cli blocks on user-input (for typing password).
///
/// Having to ship shell scripts in the release is less than ideal. Hopefully
/// the next iteration of this function will figure out how to natively rewrite
/// the following line.
///
/// ```sh
/// printf '$USER\n$PASSWORD\n' | $USER_INPUT_BLOCKING_CLI`
/// ```
///
/// [this example]: https://doc.rust-lang.org/std/process/struct.Stdio.html#examples-3
///
pub fn connect() {
    use std::process::{Command, Output};

    let release_connect_script = "/Users/reza.handzalah/work/shops-rust/aegis-manager/connect.sh";

    let output = Command::new("sh")
        .arg("-C")
        .arg(release_connect_script)
        .output();

    match output {
        Ok(Output { stdout: _bytes, .. }) => {
            // println!("INFO: {:?}", String::from_utf8(bytes));
        }
        e => println!("{:?}", e),
    }
}
