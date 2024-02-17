use sled::{Db, IVec};
use std::error::Error;

pub struct VectorDb {
    db: Db,
}

impl VectorDb {
    // Initialize a new database instance
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }

    // Insert or update a value associated with a key
    pub fn insert(&self, key: &str, value: &[u8]) -> Result<(), Box<dyn Error>> {
        self.db.insert(key, value)?;
        self.db.flush()?; // Ensure the data is written to disk
        Ok(())
    }

    // Get a value by key
    pub fn get(&self, key: &str) -> Result<Option<IVec>, Box<dyn Error>> {
        let result = self.db.get(key)?;
        Ok(result)
    }

    // Delete a value by key
    pub fn delete(&self, key: &str) -> Result<(), Box<dyn Error>> {
        self.db.remove(key)?;
        self.db.flush()?; // Ensure the data is written to disk
        Ok(())
    }

    pub fn update(&self, key: &str, new_value: &[u8]) -> Result<(), Box<dyn Error>> {
        self.insert(key, new_value)
    }
}
