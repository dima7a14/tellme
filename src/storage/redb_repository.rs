use crate::storage::{
    Word,
    repository::{WordRepository, WordRepositoryError},
};
use redb::{Database, ReadableDatabase, TableDefinition};

const TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("words");

#[derive(thiserror::Error, Debug)]
pub enum RedbError {
    #[error("failed to open the database")]
    Disconnect(#[from] redb::DatabaseError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Redb(#[from] redb::Error),
}

impl RedbError {
    fn into_repo_error(self, word: &str) -> WordRepositoryError {
        match self {
            Self::Json(source) => WordRepositoryError::Corrupted {
                word: word.to_owned(),
                source: Box::new(source),
            },
            other => WordRepositoryError::Unavailable(Box::new(other)),
        }
    }
}

pub struct RedbRepository {
    db: Database,
}

impl RedbRepository {
    pub fn new() -> Result<Self, RedbError> {
        let db = Database::create("tellme_db.redb")?;

        // Create a table if it does not exist
        {
            let txn = db.begin_write().map_err(redb::Error::from)?;
            txn.open_table(TABLE).map_err(redb::Error::from)?;
            txn.commit().map_err(redb::Error::from)?;
        }

        Ok(Self { db })
    }

    fn put(&self, word: &str, payload: &Word) -> Result<(), RedbError> {
        let bytes = serde_json::to_vec(payload)?;
        let txn = self.db.begin_write().map_err(redb::Error::from)?;

        {
            let mut table = txn.open_table(TABLE).map_err(redb::Error::from)?;
            table
                .insert(word, bytes.as_slice())
                .map_err(redb::Error::from)?;
        }

        txn.commit().map_err(redb::Error::from)?;

        Ok(())
    }

    fn read(&self, word: &str) -> Result<Option<Word>, RedbError> {
        let txn = self.db.begin_read().map_err(redb::Error::from)?;
        let table = txn.open_table(TABLE).map_err(redb::Error::from)?;

        match table.get(word).map_err(redb::Error::from)? {
            Some(guard) => Ok(Some(serde_json::from_slice(guard.value())?)),
            None => Ok(None),
        }
    }
}

impl WordRepository for RedbRepository {
    fn save(
        &self,
        word: &str,
        payload: Word,
    ) -> Result<(), super::repository::WordRepositoryError> {
        let existed = self.get(word)?;

        match existed {
            Some(existed) => {
                let merged_payload = Word {
                    translation: payload.translation.or(existed.translation),
                    definitions: payload.definitions.or(existed.definitions),
                };

                self.put(word, &merged_payload)
                    .map_err(|e| e.into_repo_error(word))
            }
            None => self
                .put(word, &payload)
                .map_err(|e| e.into_repo_error(word)),
        }
    }

    fn get(&self, word: &str) -> Result<Option<Word>, super::repository::WordRepositoryError> {
        self.read(word).map_err(|e| e.into_repo_error(word))
    }

    #[expect(clippy::todo, reason = "list() not implemented yet")]
    fn list(&self) -> Result<Vec<(String, Word)>, super::repository::WordRepositoryError> {
        todo!()
    }
}
