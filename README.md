# BMOS - Basic Multi-purpose Operating System Shell

A lightweight, interactive shell implementation written in Rust that provides command execution, built-in utilities, and advanced shell features.

## Features

- **Interactive Command Line**: User-friendly shell interface with a command prompt
- **Built-in Commands**: Implements essential shell utilities including:
  - `cd` - Change directory
  - `echo` - Print text
  - `pwd` - Print working directory
  - `exit` - Exit the shell
  - `type` - Display command type information
  - `jobs` - Manage background processes
- **Command Completion**: Intelligent command and path auto-completion
- **Background Execution**: Support for running commands in the background
- **I/O Redirection**: Output configuration and redirection capabilities
- **Error Handling**: Comprehensive error management with helpful error messages
- **PATH Resolution**: Automatic command resolution through system PATH

## Project Structure

```
src/
├── main.rs                 # Entry point and REPL loop
├── shell.rs               # Core shell input processing
├── runner.rs              # Command execution engine
├── input.rs               # Input parsing and tokenization
├── argument_parser.rs     # Argument parsing utilities
├── builtin_command.rs     # Built-in command definitions
├── check_builtin.rs       # Built-in command detection
├── order.rs               # Command order/execution structure
├── output.rs              # Output handling
├── output_config.rs       # Output redirection configuration
├── path_finder.rs         # PATH environment variable resolution
├── completion.rs          # Shell completion integration
├── command_completer.rs   # Command name completion
├── path_completer.rs      # Path completion
└── error/
    ├── mod.rs            # Error module definitions
    └── not_found.rs      # Command not found errors
```

## Building

### Prerequisites

- Rust 1.70+ (2021 edition)
- Cargo

### Compile

```bash
cargo build --release
```

The compiled binary will be available at `target/release/bmos`.

## Running

Start the shell:

```bash
cargo run
```

Once running, you can enter commands at the `$ ` prompt:

```
$ echo Hello World
Hello World
$ pwd
/home/user
$ cd /tmp
$ exit
```

## Dependencies

- **rustyline** (18.0.0) - Provides line editing and command history capabilities

## Usage Examples

### Basic Commands

```bash
$ echo "Hello from BMOS"
Hello from BMOS
$ pwd
/home/user/projects
$ cd ..
$ pwd
/home/user
```

### Command Information

```bash
$ type cd
cd is a shell builtin
$ type ls
ls is /bin/ls
```

### Background Execution

```bash
$ long_running_command &
$ jobs
[1] long_running_command
```

## Development

The shell follows a modular architecture with clear separation of concerns:

- **Parser**: Tokenizes and validates user input
- **Executor**: Routes commands to appropriate handlers (built-in or external)
- **Completer**: Provides intelligent auto-completion
- **Output Manager**: Handles I/O redirection and formatting

## License

This project is open source and available for modification and distribution.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for bug reports and feature requests.
