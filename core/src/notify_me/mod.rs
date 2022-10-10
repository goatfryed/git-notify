use crate::git::GitRepo;

pub fn notify_me() {
    let repo = GitRepo::new();
    println!("{}", repo.git_dir().unwrap());
}
