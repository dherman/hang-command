use std::process::{Command, Stdio};

fn main() {
    println!("before command");
    Command::new("npm.cmd")
        .arg("install")
	.stderr(Stdio::piped())
	.status()
	.unwrap();
    println!("after command");
}
