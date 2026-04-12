use std::os::unix::fs::MetadataExt;     //to access the mode() method for checking executable permissions
use std::path::Path;                    //to work with file paths
use std::{env, io, path::PathBuf};      //to access environment variables and handle IO errors

//The PathFinder struct is responsible for finding the full path of an executable command by searching 
//through the directories listed in the PATH environment variable. It generates candidate paths for the 
//executable and checks if they have executable permissions. If a valid path is found, it returns it; 
//otherwise, it returns None.

//the logic is as follows:
//1. we read the PATH environment variable and split it into individual directories
//2. for each directory, we create a candidate path by appending the executable name to the directory path
//3. we check if any of the candidate paths have executable permissions, and
//4. if we find one, we return it as the result; if not, we return None

pub struct PathFinder {
    candidates: Vec<PathBuf>,
}

impl PathFinder {
    pub fn new(executable: String) -> PathFinder {
        let path_string = std::env::var_os("PATH").expect("PATH enviroment variable must be set");      //get the PATH env
        let candidates = env::split_paths(&path_string)                                                 //split it into directories
            .map(|dir| {
                let mut path = dir.clone();
                path.push(&executable);
                path
            })
            .collect();

        PathFinder { candidates } //return
    }

    pub fn find_executable(self) -> Option<PathBuf> {
        self.candidates
            .into_iter()
            .find(|candidate| has_exec_permissions(candidate).unwrap_or(false))
    }
}

fn has_exec_permissions(path: &Path) -> Result<bool, io::Error> {
    Ok(path.metadata()?.mode() & 0o111 != 0)    
}