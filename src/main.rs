use std::{
    env::{self, set_current_dir},
    fs,
    process::exit,
};

use home::home_dir;
use pervasives::{path, proc::call_command_};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.get(1).is_none() {
        println!("no arg passed, need at least one, REPO_NAME");
        exit(1)
    }

    let home = home_dir().unwrap();
    let pnpmm = path!(home / ".pnpmm");

    set_current_dir(pnpmm.clone()).unwrap();

    call_command_(&*format!(
        "git clone https://github.com/rajatsharma/{}",
        args.get(1).unwrap()
    ));

    if path!(pnpmm / "pnpm-lock.yaml").exists() {
        println!("Already a pnpm project");
        exit(0)
    }

    call_command_("pnpm import");

    fs::remove_file(path!(pnpmm / "yarn.lock")).expect("unable to delete file: yarn.lock");

    call_command_("git add .");
    call_command_("git commit -m 'move to pnpm'");
    call_command_("git push origin master");
    fs::remove_dir_all(pnpmm).expect("unable to delete dir: .pnpmm");
}
