use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
pub struct StoreData {
    pub tracked_files: Vec<TrackedFile>
}

#[derive(Serialize,Deserialize)]
pub struct TrackedFile {
    pub path: String,
    pub visited: Vec<VisitData>
}

#[derive(Serialize,Deserialize)]
pub struct VisitData {
    pub commit: String,
}



