use std::{
    fs::File,                       //to work with files for output redirection
    io::{Stderr, Stdout, Write},    //to handle stdout, stderr, and writing to them
};

//a command can have its output redirected to a file
// 1. redirect stdout to a file (">" or "1>")
// 2. redirect stderr to a file ("2>")
// 3. append stdout to a file (">>" or "1>>")
// 4. append stderr to a file ("2>>")

pub enum Output {
    StdOut(Stdout),
    StdErr(Stderr),
    File(File),
}

// implementing Write trait for Output to allow writing to stdout, stderr
// or a file seamlessly, depending on the configuration set in OutputConfig. This allows us to
// write output without worrying about the underlying destination, as the Output enum abstracts that away.

impl Write for Output {

    /// Write a slice of bytes to the output.
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Output::StdOut(s) => s.write(buf),
            Output::StdErr(s) => s.write(buf),
            Output::File(f) => f.write(buf),
        }
    }

    // Flush buffered output to the underlying stream (stdout, stderr, or file)
    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Output::StdOut(s) => s.flush(),
            Output::StdErr(s) => s.flush(),
            Output::File(f) => f.flush(),
        }
    }
}