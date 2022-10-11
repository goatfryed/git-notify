use std::fmt::format;
use std::path::PathBuf;
use crate::git::GitRepo;
use crate::store::load_store;

pub fn notify_me() {
    let store = load_store(PathBuf::from("./.git-notify")).unwrap();

    let repo = GitRepo::new();

    let root_path = std::env::var("GIT_WORK_TREE").unwrap_or("".to_string());

    for tracked_file in store.data.tracked_files {
        let path_from_root = root_path.to_owned() + &tracked_file.path;
        let relative_file_path = path_from_root.trim_start_matches("/");

        for visited in tracked_file.visited {
            println!("visited {} at {}", tracked_file.path, visited.commit);
            let diff_arg = format!("{}..HEAD", visited.commit);

            let vec = repo.git_repo.cmd_out([
                "diff", "--stat",
                diff_arg.as_str(),
                "--", &relative_file_path]
            ).unwrap();

            match vec.first() {
                None => println!("no changes for {} since {}", tracked_file.path, visited.commit),
                Some(line) => println!("Update! Checkout {}", line)
            }
        }
    }
}
