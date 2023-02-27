use std::process::{Command, Stdio};

fn main() {
    execute_commands();
}

fn execute_commands() {
    let first_process = Command::new("echo")
        .arg("-e")
        .arg("Shutdown\nSleep\nHibernate\nReboot")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to echo!\n");

    let second_process = Command::new("fuzzel")
        .arg("-d")
        .arg("-f JetBrainsMono NF")
        .arg("-C ebcb8bff")
        .arg("-b 1d1f21ff")
        .arg("-T footclient")
        .arg("-B 3")
        .arg("-m a3be8cff")
        .arg("-t 88c0d0ff")
        .arg("-S d8dee9ff")
        .arg("-s bf616aff")
        .arg("--show-action")
        .stdin(first_process.stdout.unwrap())
        .output()
        .expect("Failed to execute fuzzel command!\n");

    println!(
        "Output: {}",
        String::from_utf8_lossy(&second_process.stdout)
    );
}
