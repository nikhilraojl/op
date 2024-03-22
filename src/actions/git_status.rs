use crate::error::{Error, Result};
use crate::utils::{get_projects, ActionTrait, HelpTrait};
use crate::Config;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

#[derive(Debug, PartialEq, Default)]
pub struct GitStatusAction {
    pub help: bool,
}
impl HelpTrait for GitStatusAction {
    fn print_help(&self) {
        println!("op --git-status|-g            : Shows uncommitted and non-sync status of all projects. Ignores git uninitiated or clean");
    }
}
impl ActionTrait for GitStatusAction {
    fn execute(&self, config: Config) -> Result<()> {
        if self.help {
            self.print_help();
        } else {
            let projects = get_projects(config)?;
            let all_git_projs = gitstatus_on_multiple_threads(projects.dir_items)?;
            for proj in all_git_projs {
                show_output(proj);
            }
        }
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Git command and parse output
////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct GitProject {
    path: PathBuf,
    branch: Branch,
}

#[derive(Debug)]
pub struct Branch {
    local_clean_worktree: bool,
    ahead_of_remote: i32,
    behind_of_remote: i32,
}

fn parse_git_status_output(status_line: &'_ str, count: usize) -> Option<Branch> {
    // parse output of first line of git status -b -s
    // ## main...remotes/origin/main [ahead 1, behind 2]
    let (_ignore_bangs, branch_details) = status_line.split_once(' ')?;
    if branch_details.starts_with("No commits") {
        return None;
    }
    let mut ahead_of_remote = 0;
    let mut behind_of_remote = 0;
    let local_clean_worktree = count == 0;

    if let Some((_branch_name, branch_status)) = branch_details.split_once(' ') {
        let mut buf = String::new();
        let mut is_ahead = true;
        for ch in branch_status.chars() {
            match ch {
                '[' => {}
                ']' | ',' => {
                    if is_ahead {
                        ahead_of_remote = buf.parse().unwrap_or_default();
                    }
                    behind_of_remote = buf.parse().unwrap_or_default();
                    buf.clear();
                }
                ' ' => {
                    if buf == "behind" {
                        is_ahead = false;
                    }
                    buf.clear();
                }
                _ => {
                    buf.push(ch);
                }
            }
        }
    }

    Some(Branch {
        local_clean_worktree,
        ahead_of_remote,
        behind_of_remote,
    })
}

fn run_git_status(path: &PathBuf) -> Option<GitProject> {
    let mut git = Command::new("git");
    git.arg("-C")
        .arg(path)
        .arg("status")
        .arg("--branch")
        .arg("--short");
    let output = git
        .stdout(Stdio::piped())
        .output()
        .expect("git should be installed");
    if let Some(code) = output.status.code() {
        if code == 0 {
            // success case
            let command_output = String::from_utf8_lossy(&output.stdout);
            let mut lines = command_output.lines();
            let status_line = lines.next()?;
            let remaning_lines = lines.count();
            let branch = parse_git_status_output(status_line, remaning_lines);
            if let Some(branch) = branch {
                return Some(GitProject {
                    path: path.to_owned(),
                    branch,
                });
            }
            return None;
        };
    }
    None
}

fn show_output(proj: Option<GitProject>) {
    // only show an output if current local and remote are out of sync or
    // has uncommitted local changes
    if let Some(proj) = proj {
        let file_name = proj
            .path
            .file_name()
            .expect("Unable to covert to OsStr")
            .to_str()
            .expect("Unable to convert OsStr to &str");
        let mut proj_status: Vec<&str> = Vec::new();
        if proj.branch.ahead_of_remote != 0 && proj.branch.behind_of_remote != 0 {
            proj_status.push("NOT IN SYNC");
        }
        if !proj.branch.local_clean_worktree {
            proj_status.push("DIRTY");
        }
        if !proj_status.is_empty() {
            println!("{:<25}: {:?}", file_name, proj_status);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// thread pool and worker
////////////////////////////////////////////////////////////////////////////////
struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

type AcMx<T> = Arc<Mutex<T>>;

impl Worker {
    fn job(
        _id: usize,
        input_data: AcMx<Vec<PathBuf>>,
        output_data: AcMx<Vec<Option<GitProject>>>,
    ) -> Self {
        // TLDR: values from `input_data` are popped, processed and the output
        // is pushed to `output_data`
        let thread = thread::spawn(move || loop {
            let mut data = input_data
                .lock()
                .expect("Unable to acquire lock on input_data");
            let o_path = data.pop();
            drop(data);
            match o_path {
                Some(path) => {
                    let res = run_git_status(&path);
                    let mut locked_output = output_data
                        .lock()
                        .expect("Unable to acquire lock on output_data");
                    locked_output.push(res);
                }
                None => {
                    break;
                }
            }
        });
        Self {
            thread: Some(thread),
        }
    }
}
struct Executor {
    workers: Vec<Worker>,
}

impl Executor {
    fn run(num_threads: usize, data: Vec<PathBuf>) -> AcMx<Vec<Option<GitProject>>> {
        let mut workers = Vec::with_capacity(num_threads);
        let input_data = Arc::new(Mutex::new(data));

        let git_data: Vec<Option<GitProject>> = Vec::new();
        let atomic_git_data = Arc::new(Mutex::new(git_data));

        // spawn threads which will work until values in `input_data` are exhausted
        for id in 0..num_threads {
            workers.push(Worker::job(id, input_data.clone(), atomic_git_data.clone()));
        }

        Executor { workers };
        atomic_git_data
    }
}
impl Drop for Executor {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            // We need to join all worker threads before proceeding further
            if let Some(t) = worker.thread.take() {
                t.join().expect("Unable to join thread to parent thread");
            }
        }
    }
}

fn gitstatus_on_multiple_threads(paths: Vec<PathBuf>) -> Result<Vec<Option<GitProject>>> {
    let num_threads = 6;
    let pool = Executor::run(num_threads, paths);
    let lock = Arc::into_inner(pool).ok_or(Error::FetchStatus)?;
    let data = lock.into_inner().map_err(|_| Error::FetchStatus)?;
    Ok(data)
}
