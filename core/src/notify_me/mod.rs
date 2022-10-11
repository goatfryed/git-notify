use std::path::PathBuf;
use crate::store::load_store;

pub fn notify_me() {
    let store = load_store(PathBuf::from("./.git-notify")).unwrap();

    for tracked_file in store.data.tracked_files {
        for visited in tracked_file.visited {
            println!("visited {} at {}", tracked_file.path, visited.commit);
        }
    }
}
