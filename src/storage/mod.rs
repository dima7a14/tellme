pub mod redb_repository;
pub mod repository;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Word {
    pub translation: Option<String>,
    pub definitions: Option<Vec<String>>,
}
