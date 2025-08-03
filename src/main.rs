use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("No arguments passed. Expected repo_name".into());
    }
    let repo = &args[1];

    let previous_dir = env::current_dir()?;
    let home_dir = env::home_dir().ok_or("Could not get home directory")?;

    let pnpmm_go_dir = home_dir.join(".pnpmm-go");

    fs::create_dir_all(&pnpmm_go_dir)?;
    env::set_current_dir(&pnpmm_go_dir)?;

    let repo_url = format!("https://github.com/rajatsharma/{}", repo);
    run_proc(&format!("git clone {}", repo_url))?;

    let project_dir = pnpmm_go_dir.join(repo);

    // This is a common Rust pattern for cleanup (defer)
    let _cleanup_guard = Cleanup::new(project_dir.clone());

    if project_dir.join("pnpm-lock.yaml").exists() {
        return Err("Already a pnpm project".into());
    }

    env::set_current_dir(&project_dir)?;
    run_proc("pnpm import")?;

    let yarn_lock = project_dir.join("yarn.lock");
    if yarn_lock.exists() {
        fs::remove_file(&yarn_lock)?;
    }

    run_proc("git add .")?;
    run_proc("git commit -m 'move to pnpm'")?;
    run_proc("git push origin master")?;

    env::set_current_dir(&previous_dir)?;

    Ok(())
}

fn run_proc(command: &str) -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;
    println!("Running '{}' in {}", command, current_dir.display());

    let status = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !status.success() {
        return Err(format!("Unable to run '{}'", command).into());
    }

    Ok(())
}

struct Cleanup {
    path: PathBuf,
}

impl Cleanup {
    fn new(path: PathBuf) -> Self {
        Cleanup { path }
    }
}

impl Drop for Cleanup {
    fn drop(&mut self) {
        println!("Cleaning up project directory: {}", self.path.display());
        if let Err(e) = fs::remove_dir_all(&self.path) {
            eprintln!(
                "Could not delete project dir, please remove it yourself: {}. Error: {}",
                self.path.display(),
                e
            );
        }
    }
}
