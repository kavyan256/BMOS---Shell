#[derive(Debug)]
pub enum BuiltinCommand {
    Cd,
    Echo,
    Exit,
    TypeCmd,
    Pwd,
}

impl BuiltinCommand {
    // Single source of truth for all builtins.
    // Add a new variant above + one entry here — completion picks it up automatically.
    pub fn variants() -> &'static [&'static str] {
        &["cd", "echo", "exit", "type", "pwd"]
    }
}

impl TryFrom<String> for BuiltinCommand {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "cd"   => Ok(BuiltinCommand::Cd),
            "echo" => Ok(BuiltinCommand::Echo),
            "exit" => Ok(BuiltinCommand::Exit),
            "type" => Ok(BuiltinCommand::TypeCmd),
            "pwd"  => Ok(BuiltinCommand::Pwd),
            _      => Err("Not a builtin command".to_string()),
        }
    }
}

impl std::fmt::Display for BuiltinCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuiltinCommand::Cd      => write!(f, "cd"),
            BuiltinCommand::Echo    => write!(f, "echo"),
            BuiltinCommand::Exit    => write!(f, "exit"),
            BuiltinCommand::TypeCmd => write!(f, "type"),
            BuiltinCommand::Pwd     => write!(f, "pwd"),
        }
    }
}