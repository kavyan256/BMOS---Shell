#[derive(Debug)]
pub enum NotFound {
    Command(String),        //when a command is not found in the system
    RedirectTargetFile,     //when a redirection operator is used but no target file is specified
    OutputConfigSymbol,     //when an invalid output configuration symbol is used in the command
    Io(String),             //when an IO error occurs, we can include the error message for more details
}

// this is the function fmt, required by the Display trait which allows us to specify how the NotFound error
// should be formatted when printed. We borrow the current instance of NotFound(self) and a mutable reference
// to a formatter, that is a writable buffer where we can write the formatted string.

// depending on the variant of NotFound, we match and write a different error message to the formatter.
// 1. If it's a Command variant
// 2. If it's a RedirectTargetFile variant
// 3. If it's an OutputConfigSymbol variant
// 4. If it's an Io variant

impl std::fmt::Display for NotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotFound::Command(command) => write!(f, "{}: command not found", command),
            NotFound::RedirectTargetFile => {
                write!(f, "no such file or directory for redirection target")
            }
            NotFound::OutputConfigSymbol => write!(
                f,
                "invalid output config symbol. must be >, 1>, 2>, >>, 1>>< 2>>"
            ),
            NotFound::Io(msg) => write!(f, "{}", msg),
        }
    }
}

// This implementation allows our NotFound error to be treated as a standard error in Rust,
impl std::error::Error for NotFound {}

// This implementation allows us to convert a standard IO error into our custom NotFound error.
impl From<std::io::Error> for NotFound {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            std::io::ErrorKind::NotFound => NotFound::Io("No such file or directory".to_string()),
            _ => panic!("Unhandled IO error kind"),
        }
    }
}