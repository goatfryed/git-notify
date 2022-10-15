use crate::Context;

pub fn confirm(context: Context, file: String) {
    let mut updated_data = context.store.data.clone();
    match updated_data.tracked_files.first_mut() {
        Some(file) => {
            match file.visited.first_mut() {
                Some(visited) => {
                    visited.commit = "Kaesuchen".to_string()
                }
                None => {}
            }
        }
        None => {}
    }

}