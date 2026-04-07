pub fn exit() -> bool {
    println!("Exiting ...");
    true
}

pub fn error_cmd_not_fnd(command: &str) -> bool {
    println!("{}: command not found", command);
    false
}