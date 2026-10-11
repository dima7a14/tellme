pub mod redb_repository;
pub mod repository;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Word {
    pub translation: String,
    pub definitions: Option<Vec<String>>,
}
