use rustygit::error::GitError;
use rustygit::Repository;

pub struct GitRepo {
    git_repo: Repository,
    git_dir: Result<String, GitError>,
}

impl GitRepo {
    pub fn new() -> Self {
        let git_repo = Repository::new(".");
        let git_dir = get_git_dir(&git_repo);
        Self {
            git_repo,
            git_dir,
        }
    }
    pub fn git_dir(&self) -> Result<&String, &GitError> {
        self.git_dir.as_ref()
    }
}

fn get_git_dir(git_repo: &Repository) -> Result<String, GitError> {
    git_repo.cmd_out(["rev-parse", "--git-dir"])
        .and_then(|lines| lines.into_iter().next().ok_or(GitError::Undecodable))
}
