use crate::error::Error;

use std::path::PathBuf;
use std::env;


pub struct Store {
    path: PathBuf,
}

impl Store {
    pub fn new() -> Store {
        let path = env::var("STORE")
            .map(|store| PathBuf::from(store))
            .unwrap_or_else(|_| PathBuf::from("/var/lib/s0-store"));

        Store {
            path,
        }
    }

    pub fn has(&self, name: &str) -> bool {
        self.path.join(name).exists()
    }
}


