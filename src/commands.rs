pub fn exit() -> bool {
    println!("Exiting ...");
    true
}

pub fn cmd_not_fnd_err(command: &str) -> bool {
    println!("{}: command not found", command);
    false
}

pub fn echo(args: &[&str]) -> bool {
    println!("{}", args.join(" "));
    false
}

pub fn type_cmd(arg: &str) -> bool {
    match arg {
        "exit" | "echo" | "type" => println!("{} is a shell builtin", arg),
        _ => println!("{}: not found", arg),
    }
    false
}

pub fn type_cmd_err() -> bool {
    println!("type: missing argument");
    false
}