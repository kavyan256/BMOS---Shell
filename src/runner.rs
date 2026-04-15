use crate::builtin_command::BuiltinCommand;
use crate::error::not_found::NotFound;
use crate::output_config::OutputConfig;
use crate::path_finder::PathFinder;
use std::env;
use std::io::Write;
use std::ops::ControlFlow;
use std::path::Path;
use std::sync::{Mutex, OnceLock};
use std::collections::HashMap;

//runner file is responsible for actually executing commands, whether they are built-in commands or executables. 
//It defines functions for each built-in command and a function to execute external commands. Each function takes 
//the necessary arguments and an OutputConfig to handle output redirection, and returns a ControlFlow to indicate 
//whether to continue executing further commands or to break. The runner serves as the core execution engine of the 
//shell, coordinating the execution of commands based on their type and handling their output appropriately.

// command list
// 1. exit
// 2. echo
// 3. type
// 4. pwd
// 5. cd
// 6. executable

// Job tracking structure
struct JobManager {
    next_job_number: usize,
    jobs: HashMap<usize, (u32, String)>, // job_number -> (pid, command_name)
}

impl JobManager {
    fn new() -> Self {
        JobManager {
            next_job_number: 1,
            jobs: HashMap::new(),
        }
    }

    fn add_job(&mut self, pid: u32, command_name: String) -> usize {
        let job_number = self.next_job_number;
        self.jobs.insert(job_number, (pid, command_name));
        self.next_job_number += 1;
        job_number
    }

    fn get_jobs(&self) -> Vec<(usize, u32, String)> {
        let mut jobs: Vec<_> = self.jobs.iter()
            .map(|(&num, &(pid, ref cmd))| (num, pid, cmd.clone()))
            .collect();
        jobs.sort_by_key(|&(num, _, _)| num);
        jobs
    }
}

// Global job manager instance
fn get_job_manager() -> &'static Mutex<JobManager> {
    static JOB_MANAGER: OnceLock<Mutex<JobManager>> = OnceLock::new();
    JOB_MANAGER.get_or_init(|| Mutex::new(JobManager::new()))
}

pub fn exit() -> ControlFlow<()> {
    ControlFlow::Break(())
}

pub fn echo(args: &[String], mut output_config: OutputConfig) -> ControlFlow<()> {
    writeln!(output_config.stdout, "{}", args.join(" ")).unwrap();
    ControlFlow::Continue(())
}

//r# is a raw identifier to tell rust to treat type as a regular identifier and not a keyword
pub fn r#type(args: &Vec<String>, mut output_config: OutputConfig) -> ControlFlow<()> {
    for arg in args {
        match BuiltinCommand::try_from(arg.clone()) {
            Ok(_) => {
                writeln!(output_config.stdout, "{} is a shell builtin", arg).unwrap();
            }
            Err(_) => {
                let finder = PathFinder::new(arg.clone());
                match finder.find_executable() {
                    Some(path) => {
                        writeln!(output_config.stdout, "{} is {}", arg, path.display()).unwrap()
                    }
                    None => writeln!(output_config.stderr, "{}: not found", arg).unwrap(),
                }
            }
        }
    }
    ControlFlow::Continue(())
}

pub fn pwd(mut output_config: OutputConfig) -> ControlFlow<()> {
    let path = std::env::current_dir().expect("couldn't access current working directory");
    writeln!(output_config.stdout, "{}", path.display()).unwrap();
    ControlFlow::Continue(())
}

pub fn cd(args: &[String]) -> Result<ControlFlow<()>, NotFound> {
    let home = env::home_dir()
        .expect("couldn't get path of current user's HOME directory")
        .to_string_lossy()
        .into();
    let path = if let Some(p) = args.first() {
        if p == "~" { home } else { p.clone() }
    } else {
        home
    };
    env::set_current_dir(&path)?;
    Ok(ControlFlow::Continue(()))
}

pub fn executable(path: &Path,args: &Vec<String>,mut output_config: OutputConfig,is_background: bool,) -> ControlFlow<()> {
    if is_background {
        let child = std::process::Command::new(path.file_name().unwrap())  
            .args(args)
            .spawn()    //spawns child process and doesnt wait for finish
            .unwrap();
        
        let pid = child.id();
        let command_name = path.file_name().unwrap().to_string_lossy().to_string();
        
        // Add the job to the job manager
        let mut manager = get_job_manager().lock().unwrap();
        let job_number = manager.add_job(pid, command_name.clone());
        writeln!(output_config.stdout, "[{}] {}", job_number, pid).unwrap();
    } else {
        let command_out = std::process::Command::new(path.file_name().unwrap())
            .args(args)
            .output()
            .unwrap();
        output_config.stdout.write_all(&command_out.stdout).unwrap();
        output_config.stderr.write_all(&command_out.stderr).unwrap();
    }
    ControlFlow::Continue(())
}

pub fn jobs() -> ControlFlow<()> {
    //correct implementation for now
    ControlFlow::Continue(())
}